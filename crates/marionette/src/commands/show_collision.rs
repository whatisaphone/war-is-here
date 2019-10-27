use crate::{
    darksiders1::{gfc, hkcdShapeType__ShapeTypeEnum, Lift, Lift1, LoweredAutoRef},
    hooks::ON_POST_UPDATE_QUEUE,
    library::bitmap_font,
    utils::coordinate_transformer::CoordinateTransformer,
};
use darksiders1_sys::target::{self, hkpConvexVerticesShape};
use na::Vector4;
use std::{
    arch::x86::__m128,
    sync::atomic::{AtomicBool, Ordering},
};

pub fn run(_command: &str) {
    let mut guard = ON_POST_UPDATE_QUEUE.lock();
    guard
        .as_mut()
        .unwrap()
        .push_back(Box::new(move || unsafe { go() }));
}

unsafe fn go() {
    DRAW_ENABLED.store(true, Ordering::SeqCst);

    let world = match gfc::OblivionGame::get_instance().get_world() {
        Some(world) => world,
        None => return,
    };

    let physics_manager = (*world.as_ptr()).mPhysicsManager.ptr();
    eprintln!(
        "(*physics_manager).mRigidBodies.lift1_ref().len() = {:?}",
        (*physics_manager).mRigidBodies.lift1_ref().len(),
    );
    for &body in (*physics_manager).mRigidBodies.lift1_ref() {
        let body = (*body).lift_ref();

        let rigid_body = (*body.as_ptr()).mRigidBody;
        let shape = (*rigid_body).m_collidable.m_shape;
        let shape_type = (*shape).m_type.m_storage;
        eprintln!("shape_type = {:?}", shape_type);
        if i32::from(shape_type) == hkcdShapeType__ShapeTypeEnum::ConvexVertices as i32 {
            let shape = (*shape.cast::<hkpConvexVerticesShape>()).lift_ref();
            let mut vertices = Vec::new();
            shape.get_original_vertices(&mut vertices);
            let vertices = &*(vertices.as_slice() as *const [__m128] as *const [Vector4<f32>]);
            eprintln!("vertices = {:?}", vertices);
            eprintln!("vertices.len() = {:?}", vertices.len());
        }
    }
}

static DRAW_ENABLED: AtomicBool = AtomicBool::new(false);

pub unsafe fn draw(renderer: &gfc::UIRenderer) {
    if !DRAW_ENABLED.load(Ordering::SeqCst) {
        return;
    }

    let world = match gfc::OblivionGame::get_instance().get_world() {
        Some(world) => world,
        None => return,
    };

    target::gfc__UIRenderer__begin(renderer.as_ptr(), true);
    target::gfc__UIRenderer__setMaterial(
        renderer.as_ptr(),
        (*renderer.as_ptr()).mSolidMaterial.ptr(),
    );

    let transformer = CoordinateTransformer::create();

    let physics_manager = (*world.as_ptr()).mPhysicsManager.ptr();
    for &body in (*physics_manager).mRigidBodies.lift1_ref() {
        let body = (*body).lift_ref();
        let object = (*(*body.as_ptr()).mObject).lift_ref();

        let world_object = (*(*object.as_ptr()).mWorldObject).lift_ref();
        let world_object_name = (*world_object.as_ptr()).mName.lift_ref();

        let rigid_body = (*body.as_ptr()).mRigidBody;
        let shape = (*rigid_body).m_collidable.m_shape;

        let node = (*body.as_ptr()).mNode.lift_ref();
        let position = node.get_position();

        let screen = transformer.world_to_screen(&position);
        if screen.z >= 0.0 && screen.z < 1000.0 {
            bitmap_font::draw_string(
                renderer.as_ptr(),
                screen.x,
                screen.y,
                world_object_name
                    .c_str()
                    .to_str()
                    .unwrap_or("<invalid utf-8>"),
            );

            bitmap_font::draw_string(
                renderer.as_ptr(),
                screen.x,
                screen.y + 20.0,
                &format!("shape type {}", (*shape).m_type.m_storage),
            );
        }
    }

    target::gfc__UIRenderer__endRendering(renderer.as_ptr());
}
