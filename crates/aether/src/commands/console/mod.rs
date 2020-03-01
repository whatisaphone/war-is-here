use crate::{darksiders1::keen, utils::marker::UnsafeSend};
use darksiders1_sys::target;
use imgui::{im_str, Condition, Context, FocusedWidget, ImString, InputText, Key, Window};
use parking_lot::Mutex;
use std::{
    char,
    convert::TryFrom,
    sync::atomic::{AtomicBool, Ordering},
    time::Instant,
};

mod draw;

// TODO: don't hardcode
const SCREEN_WIDTH: u16 = 1280;
const SCREEN_HEIGHT: u16 = 720;
const WINDOW_LEFT: i32 = 100;
const WINDOW_TOP: i32 = 500;
const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 200;

pub static WANT_ENABLED: AtomicBool = AtomicBool::new(false);
pub static IS_ENABLED: AtomicBool = AtomicBool::new(false);
// Safety: although this is stored in a static, it must only be accessed from
// the game's render thread.
static STATE: Mutex<Option<UnsafeSend<State>>> = Mutex::new(None);

struct State {
    imgui: Context,
    last_frame: Instant,
    draw: draw::State,
    ui: UIState,
}

struct UIState {
    input: ImString,
}

pub fn run(_command: &str) -> &'static str {
    let prev_enabled = WANT_ENABLED.fetch_nand(true, Ordering::SeqCst);
    let enabled = !prev_enabled;
    if enabled {
        "now set to true"
    } else {
        "now set to false"
    }
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

pub fn handle_input_event(event: &target::keen__InputEvent) -> bool {
    let mut guard = STATE.lock();
    let state = match guard.as_mut() {
        Some(s) => s,
        None => return true,
    };

    let typ = keen::InputEventType::try_from(event.r#type).ok();
    match typ {
        Some(keen::InputEventType::RawButtonDown)
        | Some(keen::InputEventType::RawButtonUp)
        | Some(keen::InputEventType::Key) => {}
        _ => {
            return true;
        }
    };

    // If we get here, we know `data` is a `KeyEventData`.
    let data = unsafe {
        // Work around pdbindgen unsupported union
        let data = (event as *const _ as *const u8).offset(4);
        &*data.cast::<target::keen__KeyEventData>()
    };

    let io = state.imgui.io_mut();

    match typ {
        Some(keen::InputEventType::RawButtonDown) => {
            io.keys_down[usize::try_from(data.keyCode).unwrap()] = true;
        }
        Some(keen::InputEventType::RawButtonUp) => {
            io.keys_down[usize::try_from(data.keyCode).unwrap()] = false;
        }
        Some(keen::InputEventType::Key) => {
            if let Some(ch) = char::from_u32(data.keyCode) {
                io.add_input_character(ch);
            }
        }
        _ => unreachable!(),
    }

    // Swallow key event
    false
}

fn init() {
    let mut imgui = Context::create();
    imgui.set_ini_filename(None);

    let io = imgui.io_mut();

    macro_rules! key {
        ($key:expr, $id:expr) => {
            io[$key] = $id.into();
        };
    }
    key!(Key::Tab, keen::KeyboardButtonId::Tab);
    key!(Key::LeftArrow, keen::KeyboardButtonId::ArrowLeft);
    key!(Key::RightArrow, keen::KeyboardButtonId::ArrowRight);
    key!(Key::UpArrow, keen::KeyboardButtonId::ArrowUp);
    key!(Key::PageUp, keen::KeyboardButtonId::PageUp);
    key!(Key::PageDown, keen::KeyboardButtonId::PageDown);
    key!(Key::Home, keen::KeyboardButtonId::Home);
    key!(Key::End, keen::KeyboardButtonId::End);
    key!(Key::Insert, keen::KeyboardButtonId::Insert);
    key!(Key::Delete, keen::KeyboardButtonId::Delete);
    key!(Key::Backspace, keen::KeyboardButtonId::Backspace);
    key!(Key::Space, keen::KeyboardButtonId::Space);
    key!(Key::Enter, keen::KeyboardButtonId::Return);
    key!(Key::Escape, keen::KeyboardButtonId::Escape);
    // Key::KeyPadEnter
    key!(Key::A, keen::KeyboardButtonId::A);
    key!(Key::C, keen::KeyboardButtonId::C);
    key!(Key::V, keen::KeyboardButtonId::V);
    key!(Key::X, keen::KeyboardButtonId::X);
    key!(Key::Y, keen::KeyboardButtonId::Y);
    key!(Key::Z, keen::KeyboardButtonId::Z);

    io.display_size = [SCREEN_WIDTH.into(), SCREEN_HEIGHT.into()];

    // Make the background semi-transparent
    imgui.style_mut().colors[imgui::StyleColor::WindowBg as usize][3] = 0.75;

    let draw = draw::init(SCREEN_WIDTH, SCREEN_HEIGHT, &mut imgui);

    let mut guard = STATE.lock();
    *guard = unsafe {
        Some(UnsafeSend::new(State {
            imgui,
            last_frame: Instant::now(),
            draw,
            ui: UIState {
                input: ImString::with_capacity(1024),
            },
        }))
    };
}

fn cleanup() {
    let mut guard = STATE.lock();
    drop(guard.take());
}

fn run_frame() {
    let mut guard = STATE.lock();
    let mut state = &mut **guard.as_mut().unwrap();

    let io = state.imgui.io_mut();
    state.last_frame = io.update_delta_time(state.last_frame);

    let ui = state.imgui.frame();
    let uist = &mut state.ui;
    #[allow(clippy::cast_precision_loss)]
    Window::new(im_str!("Console"))
        .position([WINDOW_LEFT as f32, WINDOW_TOP as f32], Condition::Always)
        .size(
            [WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32],
            Condition::Always,
        )
        .title_bar(false)
        .resizable(false)
        .build(&ui, || {
            ui.push_item_width(-1.0);
            InputText::new(&ui, im_str!("input"), &mut uist.input).build();
            ui.set_keyboard_focus_here(FocusedWidget::Previous);
        });

    let draw_data = ui.render();
    draw::draw(
        &mut state.draw,
        draw_data,
        WINDOW_LEFT,
        WINDOW_TOP,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    );
}
