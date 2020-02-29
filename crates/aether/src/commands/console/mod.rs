use crate::utils::marker::UnsafeSend;
use imgui::{im_str, Condition, Context, Window};
use parking_lot::Mutex;
use std::{
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

fn init() {
    let mut imgui = Context::create();
    imgui.io_mut().display_size = [SCREEN_WIDTH.into(), SCREEN_HEIGHT.into()];
    // Make the background semi-transparent
    imgui.style_mut().colors[imgui::StyleColor::WindowBg as usize][3] = 0.75;
    imgui.set_ini_filename(None);

    let draw = draw::init(SCREEN_WIDTH, SCREEN_HEIGHT, &mut imgui);

    let mut guard = STATE.lock();
    *guard = unsafe {
        Some(UnsafeSend::new(State {
            imgui,
            last_frame: Instant::now(),
            draw,
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
    #[allow(clippy::cast_precision_loss)]
    Window::new(im_str!("Console"))
        .position([WINDOW_LEFT as f32, WINDOW_TOP as f32], Condition::Always)
        .size(
            [WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32],
            Condition::Always,
        )
        .collapsible(false)
        .resizable(false)
        .build(&ui, || {
            ui.text(im_str!("test"));
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
