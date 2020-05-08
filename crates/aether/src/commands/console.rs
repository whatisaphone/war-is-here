use crate::{
    commands,
    commands::COMMANDS,
    ui,
    utils::{
        imgui_ext::{ImGuiInputTextCallbackDataExt, InputTextWithCallback},
        misc::common_prefix,
    },
};
use imgui::{im_str, sys::ImGuiInputTextCallbackData, ImString};
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::{
    cell::{Cell, RefCell},
    collections::VecDeque,
    os::raw::c_int,
    sync::atomic::{AtomicBool, Ordering},
};

pub fn run(_command: &str) -> &'static str {
    if toggle_visible() {
        "now set to true"
    } else {
        "now set to false"
    }
}

pub fn toggle_visible() -> bool {
    let prev_visible = VISIBLE.fetch_nand(true, Ordering::SeqCst);
    let visible = !prev_visible;
    if visible {
        // `ui` is what draws the console, so it must also be enabled.
        ui::WANT_ENABLED.store(true, Ordering::SeqCst);
    }
    visible
}

struct State {
    scrollback: RefCell<VecDeque<ImString>>,
    need_scroll: Cell<bool>,
    input: RefCell<ImString>,
    last_command: RefCell<Option<String>>,
}

const WINDOW_LEFT: f32 = 50.0;
const WINDOW_TOP: f32 = 50.0;
const WINDOW_WIDTH: f32 = 420.0;
const WINDOW_HEIGHT: f32 = 200.0;

const SCROLLBACK_LINES: usize = 100;

static VISIBLE: AtomicBool = AtomicBool::new(false);
static STATE: Lazy<Mutex<State>> = Lazy::new(|| {
    Mutex::new(State {
        scrollback: RefCell::new(VecDeque::new()),
        need_scroll: Cell::new(false),
        input: RefCell::new(ImString::with_capacity(1023)),
        last_command: RefCell::new(None),
    })
});

pub fn draw(ui: &imgui::Ui<'_>) {
    if !VISIBLE.load(Ordering::SeqCst) {
        return;
    }

    let guard = STATE.lock();
    let state = &*guard;

    imgui::Window::new(im_str!("Console"))
        .position([WINDOW_LEFT, WINDOW_TOP], imgui::Condition::Always)
        .size([WINDOW_WIDTH, WINDOW_HEIGHT], imgui::Condition::Always)
        .title_bar(false)
        .resizable(false)
        .scroll_bar(false)
        .scrollable(false)
        .build(ui, || {
            imgui::ChildWindow::new("##scrollback")
                .flags(imgui::WindowFlags::HORIZONTAL_SCROLLBAR)
                .size([0.0, -23.0])
                .build(&ui, || {
                    for s in &*state.scrollback.borrow() {
                        ui.text_wrapped(s);
                    }
                    if state.need_scroll.get() {
                        ui.set_scroll_here_y();
                        state.need_scroll.set(false);
                    }
                });

            ui.align_text_to_frame_padding();
            ui.text(">");
            ui.same_line(0.0);
            ui.set_keyboard_focus_here(imgui::FocusedWidget::Next);
            ui.set_next_item_width(-1.0);
            let pressed_enter =
                InputTextWithCallback::new(&ui, im_str!("##input"), &mut state.input.borrow_mut())
                    .flags(
                        imgui::ImGuiInputTextFlags::EnterReturnsTrue
                            | imgui::ImGuiInputTextFlags::CallbackCompletion
                            | imgui::ImGuiInputTextFlags::CallbackHistory,
                    )
                    .build(|data| unsafe { input_callback(state, &mut *data) });
            if pressed_enter {
                let mut command = state.input.borrow_mut();
                run_command(state, command.to_str());
                command.clear();
            }
        });
}

unsafe fn input_callback(state: &State, data: &mut ImGuiInputTextCallbackData) -> c_int {
    match data.EventFlag {
        x if x == imgui::ImGuiInputTextFlags::CallbackHistory.bits() => {
            input_history(state, data);
        }
        x if x == imgui::ImGuiInputTextFlags::CallbackCompletion.bits() => {
            input_completion(state, data);
        }
        _ => unreachable!(),
    }
    0
}

unsafe fn input_history(state: &State, data: &mut ImGuiInputTextCallbackData) {
    let last_command = state.last_command.borrow();
    let last_command = match &*last_command {
        Some(c) => c,
        None => return,
    };
    let mut buf = data.yoink_buf();
    buf.clear();
    buf.push_str(last_command);
    data.replace_buf(buf);
    data.set_caret_to_end();
}

unsafe fn input_completion(state: &State, data: &mut ImGuiInputTextCallbackData) {
    let mut scrollback = state.scrollback.borrow_mut();
    let buf = data.buf();

    let commands: Vec<_> = COMMANDS
        .iter()
        .copied()
        .filter(|c| c.starts_with(buf))
        .collect();
    match commands.len() {
        0 => {
            scrollback.push_back(ImString::new("no match"));
            state.need_scroll.set(true);
        }
        1 => {
            let mut buf = data.yoink_buf();
            buf.clear();
            buf.push_str(commands[0]);
            data.replace_buf(buf);
            data.set_caret_to_end();
        }
        _ => {
            let prefix = common_prefix(commands.iter().copied());
            let mut buf = data.yoink_buf();
            buf.clear();
            buf.push_str(prefix);
            data.replace_buf(buf);
            data.set_caret_to_end();

            let buf = data.buf();
            if buf == "" || buf == "/" {
                drop(scrollback);
                run_command(state, "/help");
            } else {
                for command in commands {
                    scrollback.push_back(ImString::new(command));
                }
                state.need_scroll.set(true);
            }
        }
    }
}

fn run_command(state: &State, command: &str) {
    let scrollback = &mut *state.scrollback.borrow_mut();

    scrollback.push_back(ImString::new(format!("> {}", command)));

    run_command_inner(command, scrollback);

    if scrollback.len() > SCROLLBACK_LINES {
        scrollback.drain(..scrollback.len() - SCROLLBACK_LINES);
    }
    state.need_scroll.set(true);
    *state.last_command.borrow_mut() = Some(command.to_string());
}

fn run_command_inner(mut command: &str, scrollback: &mut VecDeque<ImString>) {
    if command.is_empty() {
        return;
    }
    if !command.starts_with('/') {
        command = "/help";
    }
    if command == "/clear" {
        scrollback.clear();
        return;
    }

    // Skip the slash
    command = &command[1..];
    match commands::run(command.as_bytes()) {
        commands::RunResult::Ok => {
            scrollback.push_back(ImString::new("OK"));
        }
        commands::RunResult::Response(text) => {
            for line in text.trim_end().lines() {
                scrollback.push_back(ImString::new(line));
            }
        }
        commands::RunResult::Shutdown => {
            // need to terminate the listener to get this to work
            scrollback.push_back(ImString::new("only valid when using socket"));
        }
    }
}
