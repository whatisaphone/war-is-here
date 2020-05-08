use crate::{
    darksiders1::keen::{self, InputEventExt},
    ui::{InputHandled, STATE},
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
            let down = typ == keen::InputEventType::RawButtonDown;
            let data = &*(*event).data_ptr().cast::<target::keen__KeyEventData>();

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
        }
        keen::InputEventType::MouseWheel => {
            let data = &*(*event)
                .data_ptr()
                .cast::<target::keen__MouseWheelEventData>();

            io.mouse_wheel = (data.wheelDelta / 120) as f32;
        }
        _ => unreachable!(),
    }

    if io.want_capture_mouse {
        InputHandled::Swallow
    } else {
        InputHandled::Continue
    }
}
