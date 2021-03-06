#![allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]

use crate::{
    commands::{console, log_events, show_triggers},
    darksiders1::{gfc, keen},
    ui::draw::check_screen_resolution_change,
    utils::marker::UnsafeSend,
};
use darksiders1_sys::target;
use imgui::Context;
use parking_lot::Mutex;
use std::{
    convert::{TryFrom, TryInto},
    sync::atomic::{AtomicBool, AtomicI32, Ordering},
    time::Instant,
};

mod draw;
mod fonts;
mod keyboard;
mod mouse;

/// This is basically a reference count of how many components currently request
/// the UI to be enabled.
pub static WANT_ENABLED: AtomicI32 = AtomicI32::new(0);
pub static IS_ENABLED: AtomicBool = AtomicBool::new(false);
// Safety: These must only be accessed from the game's render thread.
static STATE: Mutex<UnsafeSend<Option<State>>> = Mutex::new(UnsafeSend::new(None));
static FONT_GNU_UNIFONT: Mutex<UnsafeSend<Option<imgui::FontId>>> =
    Mutex::new(UnsafeSend::new(None));

struct State {
    imgui: Context,
    last_frame: Instant,
    draw: draw::State,
}

pub fn pump() {
    match (
        IS_ENABLED.load(Ordering::SeqCst),
        WANT_ENABLED.load(Ordering::SeqCst) != 0,
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

pub fn get_font_gnu_unifont() -> imgui::FontId {
    FONT_GNU_UNIFONT.lock().into_inner().unwrap()
}

fn init() {
    let mut imgui = Context::create();
    imgui.set_ini_filename(None);

    let viewport = gfc::KGGraphics::get_instance().get_viewport();
    let screen_width: u16 = viewport.width().try_into().unwrap();
    let screen_height: u16 = viewport.height().try_into().unwrap();

    let io = imgui.io_mut();
    keyboard::init(io);
    io.display_size = [screen_width.into(), screen_height.into()];

    // Make the background semi-transparent
    imgui.style_mut().colors[imgui::StyleColor::WindowBg as usize][3] = 0.75;

    let mut fonts = imgui.fonts();
    fonts.add_font(&[imgui::FontSource::DefaultFontData { config: None }]);
    unsafe {
        *FONT_GNU_UNIFONT.lock().as_mut() = Some(fonts::add_gnu_unifont(&mut fonts));
    }
    drop(fonts);

    let draw = draw::init(screen_width, screen_height, &mut imgui);

    let mut guard = STATE.lock();
    *guard = UnsafeSend::new(Some(State {
        imgui,
        last_frame: Instant::now(),
        draw,
    }));
}

fn cleanup() {
    let mut guard = STATE.lock();
    drop(unsafe { guard.as_mut() }.take());
}

fn run_frame() {
    let mut guard = STATE.lock();
    let state = unsafe { guard.as_mut() }.as_mut().unwrap();

    let viewport = gfc::KGGraphics::get_instance().get_viewport();
    let screen_width: u16 = viewport.width().try_into().unwrap();
    let screen_height: u16 = viewport.height().try_into().unwrap();
    check_screen_resolution_change(&mut state.draw, screen_width, screen_height);

    let io = state.imgui.io_mut();
    io.display_size = [viewport.width() as f32, viewport.height() as f32];
    let now = Instant::now();
    io.update_delta_time(now - state.last_frame);
    state.last_frame = now;

    let ui = state.imgui.frame();

    console::draw(&ui);
    log_events::draw(&ui, screen_width, screen_height);
    show_triggers::draw(&ui);

    let draw_data = ui.render();
    draw::draw(&mut state.draw, draw_data);
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
