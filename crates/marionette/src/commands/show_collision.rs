use crate::{
    darksiders1::{gfc, Lift, Lift1, LoweredAutoRef},
    hooks::ON_POST_UPDATE_QUEUE,
};

pub fn run(_command: &str) {
    let mut guard = ON_POST_UPDATE_QUEUE.lock();
    guard
        .as_mut()
        .unwrap()
        .push_back(Box::new(move || unsafe { go() }));
}

unsafe fn go() {
    let world = match gfc::OblivionGame::get_instance().get_world() {
        Some(world) => world,
        None => return,
    };

    let physics_manager = (*world.as_ptr()).mPhysicsManager.ptr();
    let rigid_bodies = (*physics_manager).mRigidBodies.lift1_ref();
    for (i, &body) in rigid_bodies.iter().enumerate() {
        let body = (*body).lift_ref();
        let classname = body.shape().class().name();
        eprintln!("{}: {:?}", i, classname.c_str());
    }
}
