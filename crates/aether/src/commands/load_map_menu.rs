use crate::hooks::ON_POST_UPDATE_QUEUE;
use darksiders1_sys::target;

pub fn run(_command: &str) {
    let mut guard = ON_POST_UPDATE_QUEUE.lock();
    guard
        .as_mut()
        .unwrap()
        .push_back(Box::new(move || unsafe { go() }));
}

unsafe fn go() {
    let window_helper = *target::gfc__Singleton_gfc__WindowHelper_gfc__CreateStatic_gfc__SingletonLongevity__DieFirst___InstanceHandle;
    target::gfc__WindowHelper__pushWindow(window_helper, hstring!("ui_core/loadmapmenu").as_ptr());
}
