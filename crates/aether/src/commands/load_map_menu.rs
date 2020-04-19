use crate::{darksiders1::gfc, hooks::ON_POST_UPDATE_QUEUE};

pub fn run(_command: &str) {
    let mut guard = ON_POST_UPDATE_QUEUE.lock();
    guard
        .as_mut()
        .unwrap()
        .push_back(Box::new(move || unsafe { go() }));
}

unsafe fn go() {
    let window_helper = <gfc::Singleton<gfc::WindowHelper>>::get_instance();
    window_helper.push_window(&hstring!("ui_core/loadmapmenu"));
}
