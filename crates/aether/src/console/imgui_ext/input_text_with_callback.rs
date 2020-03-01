use imgui::{sys, ImGuiInputTextFlags, ImStr, ImString, Ui};
use std::{convert::TryInto, marker::PhantomData, os::raw::c_int};

#[must_use]
pub struct InputTextWithCallback<'ui, 'p> {
    label: &'p ImStr,
    buf: &'p mut ImString,
    flags: ImGuiInputTextFlags,
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui, 'p> InputTextWithCallback<'ui, 'p> {
    pub fn new(_: &Ui<'ui>, label: &'p ImStr, buf: &'p mut ImString) -> Self {
        InputTextWithCallback {
            label,
            buf,
            flags: ImGuiInputTextFlags::empty(),
            _phantom: PhantomData,
        }
    }

    pub fn flags(mut self, flags: ImGuiInputTextFlags) -> Self {
        self.flags = flags;
        self
    }

    #[allow(clippy::mut_mut)]
    pub fn build<CB>(self, mut callback: CB) -> bool
    where
        CB: FnMut(*mut sys::ImGuiInputTextCallbackData) -> c_int,
    {
        let (ptr, capacity) = (self.buf.as_mut_ptr(), self.buf.capacity_with_nul());

        let mut data: TrampolineData<'_> = &mut callback;
        let data: *mut _ = &mut data;

        unsafe {
            let result = sys::igInputText(
                self.label.as_ptr(),
                ptr,
                capacity,
                self.flags.bits(),
                Some(callback_trampoline),
                data.cast(),
            );
            self.buf.refresh_len();
            result
        }
    }
}

type TrampolineData<'a> = &'a mut dyn FnMut(*mut sys::ImGuiInputTextCallbackData) -> c_int;

unsafe extern "C" fn callback_trampoline(data: *mut sys::ImGuiInputTextCallbackData) -> c_int {
    let callback = (*data).UserData as *mut TrampolineData<'_>;
    (*callback)(data)
}

pub trait ImGuiInputTextCallbackDataExt {
    unsafe fn yoink_buf(&self) -> String;
    fn replace_buf(&mut self, buf: String);
}

impl ImGuiInputTextCallbackDataExt for sys::ImGuiInputTextCallbackData {
    unsafe fn yoink_buf(&self) -> String {
        String::from_raw_parts(
            self.Buf.cast(),
            self.BufTextLen.try_into().unwrap(),
            self.BufSize.try_into().unwrap(),
        )
    }

    fn replace_buf(&mut self, mut data: String) {
        data.push('\0');
        let (buf, mut len, capacity) = data.into_raw_parts();
        len -= 1;
        self.Buf = buf.cast();
        self.BufTextLen = len.try_into().unwrap();
        self.BufSize = capacity.try_into().unwrap();
    }
}
