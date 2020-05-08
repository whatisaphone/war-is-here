#![allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]

use crate::{commands::console, darksiders1::keen, utils::marker::UnsafeSend};
use darksiders1_sys::target;
use imgui::Context;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::{
    convert::TryFrom,
    sync::atomic::{AtomicBool, Ordering},
    time::Instant,
};

mod draw;
mod keyboard;
mod mouse;

// TODO: don't hardcode
const SCREEN_WIDTH: u16 = 1280;
const SCREEN_HEIGHT: u16 = 720;

pub static WANT_ENABLED: AtomicBool = AtomicBool::new(false);
pub static IS_ENABLED: AtomicBool = AtomicBool::new(false);
// Safety: although this is stored in a static, it must only be accessed from
// the game's render thread.
static STATE: Lazy<Mutex<UnsafeSend<Option<State>>>> =
    Lazy::new(|| Mutex::new(unsafe { UnsafeSend::new(None) }));

struct State {
    imgui: Context,
    last_frame: Instant,
    draw: draw::State,
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
    imgui.set_ini_filename(None);

    let io = imgui.io_mut();
    keyboard::init(io);
    io.display_size = [SCREEN_WIDTH.into(), SCREEN_HEIGHT.into()];

    // Make the background semi-transparent
    imgui.style_mut().colors[imgui::StyleColor::WindowBg as usize][3] = 0.75;

    let draw = draw::init(SCREEN_WIDTH, SCREEN_HEIGHT, &mut imgui);

    let mut guard = STATE.lock();
    *guard = unsafe {
        UnsafeSend::new(Some(State {
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
    let state = guard.as_mut().unwrap();

    let io = state.imgui.io_mut();
    state.last_frame = io.update_delta_time(state.last_frame);

    let ui = state.imgui.frame();

    console::draw(&ui);

    let draw_data = ui.render();
    draw::draw(
        &mut state.draw,
        draw_data,
        0,
        0,
        SCREEN_WIDTH.into(),
        SCREEN_HEIGHT.into(),
    );
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
