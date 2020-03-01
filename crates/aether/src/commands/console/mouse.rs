use crate::{
    commands::console::{
        InputHandled,
        STATE,
        WINDOW_HEIGHT,
        WINDOW_LEFT,
        WINDOW_TOP,
        WINDOW_WIDTH,
    },
    darksiders1::keen::{self, InputEventExt},
};
use darksiders1_sys::target;
use std::convert::TryFrom;

pub unsafe fn handle_event(event: *const target::keen__InputEvent) -> InputHandled {
    let mut guard = STATE.lock();
    let state = match guard.as_mut() {
        Some(s) => s,
        None => return InputHandled::Continue,
    };

    let io = state.imgui.io_mut();

    let typ = keen::InputEventType::try_from((*event).r#type).unwrap();
    match typ {
        keen::InputEventType::RawButtonDown | keen::InputEventType::RawButtonUp => {
            let data = &*(*event).data_ptr().cast::<target::keen__KeyEventData>();

            let [x, y] = io.mouse_pos;
            if !inside_window(x, y) {
                return InputHandled::Continue;
            }

            // Convert `InputEvent` buttons to `imgui` buttons
            let button = match data.keyCode {
                0 => Some(0), // Left
                1 => Some(2), // Middle
                2 => Some(1), // Right
                _ => None,
            };
            if let Some(button) = button {
                io.mouse_down[button] = down;
            }
        }
        keen::InputEventType::MouseMove => {
            let data = &*(*event).data_ptr().cast::<target::keen__MouseEventData>();

            let x = data.position.x;
            let y = data.position.y;
            io.mouse_pos = [x, y];

            if !inside_window(x, y) {
                return InputHandled::Continue;
            }
        }
        keen::InputEventType::MouseWheel => {
            let data = &*(*event)
                .data_ptr()
                .cast::<target::keen__MouseWheelEventData>();

            io.mouse_wheel = (data.wheelDelta / 120) as f32;
        }
        _ => unreachable!(),
    }

    InputHandled::Swallow
}

fn inside_window(x: f32, y: f32) -> bool {
    let x = x as i32;
    let y = y as i32;
    x >= WINDOW_LEFT
        && x < WINDOW_LEFT + WINDOW_WIDTH
        && y >= WINDOW_TOP
        && y < WINDOW_TOP + WINDOW_HEIGHT
}
