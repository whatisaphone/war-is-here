use crate::{
    darksiders1::{gfc, hkcdShapeType__ShapeTypeEnum, Lift, Lift1, LoweredAutoRef},
    hooks::ON_POST_UPDATE_QUEUE,
    library::bitmap_font,
    utils::coordinate_transformer::CoordinateTransformer,
};
use darksiders1_sys::target::{hkpConvexVerticesShape, hkpMoppBvTreeShape};
use std::{
    convert::TryFrom,
    fs,
    slice,
    sync::atomic::{AtomicBool, Ordering},
};

static DRAW_ENABLED: AtomicBool = AtomicBool::new(false);

pub fn run(_command: &str) -> &'static str {
    let prev_enabled = DRAW_ENABLED.fetch_nand(true, Ordering::SeqCst);
    let enabled = !prev_enabled;

    if enabled {
        let mut guard = ON_POST_UPDATE_QUEUE.lock();
        guard
            .as_mut()
            .unwrap()
            .push_back(Box::new(move || unsafe { go() }));
    }

    if enabled {
        "now set to true"
    } else {
        "now set to false"
    }
}

unsafe fn go() {
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
        if i32::from(shape_type) == hkcdShapeType__ShapeTypeEnum::Mopp as i32 {
            let shape = shape.cast::<hkpMoppBvTreeShape>();

            let code = slice::from_raw_parts(
                (*shape).m_moppData,
                usize::try_from((*shape).m_moppDataSize).unwrap(),
            );
            fs::write(
                format!(r"C:\Users\John\AppData\Local\Temp\data_{:p}.bin", shape),
                code,
            )
            .unwrap();

            let child = (*shape).m_child.m_childShape;
            eprintln!(
                "(*child).m_type.m_storage = {:?}",
                (*child).m_type.m_storage,
            );
        }
        if i32::from(shape_type) == hkcdShapeType__ShapeTypeEnum::ConvexVertices as i32 {
            let shape = (*shape.cast::<hkpConvexVerticesShape>()).lift_ref();
            let mut vertices = Vec::new();
            let vertices = shape.get_original_vertices(&mut vertices);
            eprintln!("vertices = {:?}", vertices);
            eprintln!("vertices.len() = {:?}", vertices.len());
        }
    }
}

pub unsafe fn draw(renderer: &gfc::UIRenderer) {
    if !DRAW_ENABLED.load(Ordering::SeqCst) {
        return;
    }

    let world = match gfc::OblivionGame::get_instance().get_world() {
        Some(world) => world,
        None => return,
    };

    renderer.begin(true);
    renderer.set_material(renderer.solid_material());

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
                renderer,
                screen.x,
                screen.y,
                2,
                world_object_name
                    .c_str()
                    .to_str()
                    .unwrap_or("<invalid utf-8>"),
            );

            bitmap_font::draw_string(
                renderer,
                screen.x,
                screen.y + 20.0,
                2,
                &format!("shape type {}", (*shape).m_type.m_storage),
            );
        }
    }

    renderer.end();
}
