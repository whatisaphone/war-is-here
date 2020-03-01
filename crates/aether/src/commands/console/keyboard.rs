use crate::{
    commands::console::{InputHandled, STATE, WANT_ENABLED},
    darksiders1::keen,
};
use darksiders1_sys::target;
use imgui::{Io, Key};
use std::{char, convert::TryFrom, sync::atomic::Ordering};

pub fn init(io: &mut Io) {
    io[Key::Tab] = keen::KeyboardButtonId::Tab.into();
    io[Key::LeftArrow] = keen::KeyboardButtonId::ArrowLeft.into();
    io[Key::RightArrow] = keen::KeyboardButtonId::ArrowRight.into();
    io[Key::UpArrow] = keen::KeyboardButtonId::ArrowUp.into();
    io[Key::PageUp] = keen::KeyboardButtonId::PageUp.into();
    io[Key::PageDown] = keen::KeyboardButtonId::PageDown.into();
    io[Key::Home] = keen::KeyboardButtonId::Home.into();
    io[Key::End] = keen::KeyboardButtonId::End.into();
    io[Key::Insert] = keen::KeyboardButtonId::Insert.into();
    io[Key::Delete] = keen::KeyboardButtonId::Delete.into();
    io[Key::Backspace] = keen::KeyboardButtonId::Backspace.into();
    io[Key::Space] = keen::KeyboardButtonId::Space.into();
    io[Key::Enter] = keen::KeyboardButtonId::Return.into();
    io[Key::Escape] = keen::KeyboardButtonId::Escape.into();
    // Key::KeyPadEnter
    io[Key::A] = keen::KeyboardButtonId::A.into();
    io[Key::C] = keen::KeyboardButtonId::C.into();
    io[Key::V] = keen::KeyboardButtonId::V.into();
    io[Key::X] = keen::KeyboardButtonId::X.into();
    io[Key::Y] = keen::KeyboardButtonId::Y.into();
    io[Key::Z] = keen::KeyboardButtonId::Z.into();
}

pub unsafe fn handle_event(event: *const target::keen__InputEvent) -> InputHandled {
    // Work around pdbindgen unsupported union
    let data = (event as *const u8)
        .offset(4)
        .cast::<target::keen__KeyEventData>();

    let key_code = (*data).keyCode;

    let typ = keen::InputEventType::try_from((*event).r#type).unwrap();
    match typ {
        keen::InputEventType::RawButtonDown => handle_raw_button(key_code, true),
        keen::InputEventType::RawButtonUp => handle_raw_button(key_code, false),
        keen::InputEventType::Key => handle_key(key_code),
        _ => unreachable!(),
    }
}

fn handle_raw_button(key_code: u32, down: bool) -> InputHandled {
    if down && key_code == keen::KeyboardButtonId::AccentTilde.into() {
        WANT_ENABLED.fetch_nand(true, Ordering::SeqCst);
        return InputHandled::Swallow;
    }

    let mut guard = STATE.lock();
    let state = match guard.as_mut() {
        Some(s) => s,
        None => return InputHandled::Continue,
    };

    let io = state.imgui.io_mut();
    io.keys_down[usize::try_from(key_code).unwrap()] = down;
    InputHandled::Swallow
}

fn handle_key(key_code: u32) -> InputHandled {
    let mut guard = STATE.lock();
    let state = match guard.as_mut() {
        Some(s) => s,
        None => return InputHandled::Continue,
    };

    let io = state.imgui.io_mut();
    if let Some(ch) = char::from_u32(key_code) {
        io.add_input_character(ch);
    }
    InputHandled::Swallow
}
