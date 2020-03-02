#![allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]

use crate::{
    commands,
    commands::RunResult,
    console::imgui_ext::{ImGuiInputTextCallbackDataExt, InputTextWithCallback},
    darksiders1::keen,
    utils::marker::UnsafeSend,
};
use darksiders1_sys::target;
use imgui::{
    im_str,
    sys::ImGuiInputTextCallbackData,
    ChildWindow,
    Condition,
    Context,
    FocusedWidget,
    ImGuiInputTextFlags,
    ImString,
    Window,
    WindowFlags,
};
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::{
    cell::{Cell, RefCell},
    collections::VecDeque,
    convert::TryFrom,
    os::raw::c_int,
    sync::atomic::{AtomicBool, Ordering},
    time::Instant,
};

mod draw;
mod imgui_ext;
mod keyboard;
mod mouse;

// TODO: don't hardcode
const SCREEN_WIDTH: u16 = 1280;
const SCREEN_HEIGHT: u16 = 720;
const WINDOW_LEFT: i32 = 50;
const WINDOW_TOP: i32 = 50;
const WINDOW_WIDTH: i32 = 400;
const WINDOW_HEIGHT: i32 = 200;

const SCROLLBACK_LINES: usize = 100;

pub static WANT_ENABLED: AtomicBool = AtomicBool::new(false);
pub static IS_ENABLED: AtomicBool = AtomicBool::new(false);
// Safety: although this is stored in a static, it must only be accessed from
// the game's render thread.
static STATE: Lazy<Mutex<UnsafeSend<State>>> = Lazy::new(|| {
    Mutex::new(unsafe {
        UnsafeSend::new(State {
            enabled: None,
            ui: UIState {
                scrollback: RefCell::new(VecDeque::new()),
                need_scroll: Cell::new(false),
                input: RefCell::new(ImString::with_capacity(1023)),
                last_command: RefCell::new(None),
            },
        })
    })
});

struct State {
    enabled: Option<EnabledState>,
    ui: UIState,
}

struct EnabledState {
    imgui: Context,
    last_frame: Instant,
    draw: draw::State,
}

struct UIState {
    scrollback: RefCell<VecDeque<ImString>>,
    need_scroll: Cell<bool>,
    input: RefCell<ImString>,
    last_command: RefCell<Option<String>>,
}

pub fn pump() {
    match (
        IS_ENABLED.load(Ordering::SeqCst),
        WANT_ENABLED.load(Ordering::SeqCst),
    ) {
        (false, false) => {}
        (true, false) => {
            cleanup();
            IS_ENABLED.store(false, Ordering::SeqCst);
        }
        (false, true) => {
            IS_ENABLED.store(true, Ordering::SeqCst);
            init();

            run_frame();
        }
        (true, true) => {
            run_frame();
        }
    }
}

#[derive(Eq, PartialEq)]
pub enum InputHandled {
    Swallow,
    Continue,
}

pub unsafe fn handle_input_event(event: *const target::keen__InputEvent) -> InputHandled {
    let class = keen::ControllerClass::try_from((*event).controllerClass);
    let typ = keen::InputEventType::try_from((*event).r#type);
    match (class, typ) {
        (Ok(keen::ControllerClass::Keyboard), Ok(keen::InputEventType::RawButtonDown))
        | (Ok(keen::ControllerClass::Keyboard), Ok(keen::InputEventType::RawButtonUp))
        | (Ok(keen::ControllerClass::Keyboard), Ok(keen::InputEventType::Key)) => {
            keyboard::handle_event(event)
        }
        (Ok(keen::ControllerClass::Mouse), Ok(keen::InputEventType::RawButtonDown))
        | (Ok(keen::ControllerClass::Mouse), Ok(keen::InputEventType::RawButtonUp))
        | (Ok(keen::ControllerClass::Mouse), Ok(keen::InputEventType::MouseMove))
        | (Ok(keen::ControllerClass::Mouse), Ok(keen::InputEventType::MouseWheel)) => {
            mouse::handle_event(event)
        }
        _ => InputHandled::Continue,
    }
}

fn init() {
    let mut imgui = Context::create();
    imgui.set_ini_filename(None);

    let io = imgui.io_mut();
    keyboard::init(io);
    io.display_size = [SCREEN_WIDTH.into(), SCREEN_HEIGHT.into()];

    // Make the background semi-transparent
    imgui.style_mut().colors[imgui::StyleColor::WindowBg as usize][3] = 0.75;

    let draw = draw::init(SCREEN_WIDTH, SCREEN_HEIGHT, &mut imgui);

    let mut guard = STATE.lock();
    guard.ui.need_scroll.set(true);
    guard.enabled = Some(EnabledState {
        imgui,
        last_frame: Instant::now(),
        draw,
    });
}

fn cleanup() {
    let mut guard = STATE.lock();
    drop(guard.enabled.take());
}

fn run_frame() {
    let mut guard = STATE.lock();
    let state = &mut **guard;
    let enabled = state.enabled.as_mut().unwrap();

    let io = enabled.imgui.io_mut();
    enabled.last_frame = io.update_delta_time(enabled.last_frame);

    let ui = enabled.imgui.frame();
    let uist = &mut state.ui;

    Window::new(im_str!("Console"))
        .position([WINDOW_LEFT as f32, WINDOW_TOP as f32], Condition::Always)
        .size(
            [WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32],
            Condition::Always,
        )
        .title_bar(false)
        .resizable(false)
        .scroll_bar(false)
        .scrollable(false)
        .build(&ui, || {
            ChildWindow::new("##scrollback")
                .flags(WindowFlags::HORIZONTAL_SCROLLBAR)
                .size([0.0, -23.0])
                .build(&ui, || {
                    for s in &*uist.scrollback.borrow() {
                        ui.text_wrapped(s);
                    }
                    if uist.need_scroll.get() {
                        ui.set_scroll_here_y();
                        uist.need_scroll.set(false);
                    }
                });

            ui.set_keyboard_focus_here(FocusedWidget::Next);
            ui.set_next_item_width(-1.0);
            let pressed_enter =
                InputTextWithCallback::new(&ui, im_str!("##input"), &mut uist.input.borrow_mut())
                    .flags(
                        ImGuiInputTextFlags::CallbackHistory
                            | ImGuiInputTextFlags::EnterReturnsTrue,
                    )
                    .build(|data| unsafe { input_callback(uist, &mut *data) });
            if pressed_enter {
                run_command(uist);
            }
        });

    let draw_data = ui.render();
    draw::draw(
        &mut enabled.draw,
        draw_data,
        WINDOW_LEFT,
        WINDOW_TOP,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    );
}

unsafe fn input_callback(uist: &UIState, data: &mut ImGuiInputTextCallbackData) -> c_int {
    if let Some(command) = &*uist.last_command.borrow() {
        let mut buf = data.yoink_buf();
        buf.clear();
        buf.push_str(command);
        data.replace_buf(buf);
        data.BufDirty = true;
        data.CursorPos = data.BufTextLen;
        data.SelectionStart = data.BufTextLen;
        data.SelectionEnd = data.BufTextLen;
    }
    0
}

fn run_command(uist: &UIState) {
    let scrollback = &mut *uist.scrollback.borrow_mut();
    let input = &mut *uist.input.borrow_mut();

    scrollback.push_back(ImString::new(format!(">{}", input)));

    run_command_inner(input.to_str(), |s| {
        scrollback.push_back(ImString::new(s));
    });

    if scrollback.len() > SCROLLBACK_LINES {
        scrollback.drain(..scrollback.len() - SCROLLBACK_LINES);
    }
    uist.need_scroll.set(true);
    *uist.last_command.borrow_mut() = Some(input.to_string());
    input.clear();
}

fn run_command_inner(mut command: &str, mut write: impl FnMut(&str)) {
    if command.is_empty() {
        return;
    }
    if !command.starts_with('/') {
        command = "/help";
    }
    // Skip the slash
    command = &command[1..];

    match commands::run(command.as_bytes()) {
        RunResult::Ok => {
            write("OK");
        }
        RunResult::Response(text) => {
            for line in text.trim_end().lines() {
                write(line);
            }
        }
        RunResult::Shutdown => {
            // need to terminate the listener to get this to work
            write("only valid when using socket");
        }
    }
}
