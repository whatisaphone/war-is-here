use crate::{darksiders1::gfc, hooks::ON_POST_UPDATE_QUEUE, utils::mem::init_with};
use darksiders1_sys::target;
use once_cell::sync::Lazy;

pub fn run(command: &str) {
    let args = match parse(command) {
        Ok(args) => args,
        Err(()) => {
            println!("parse error");
            return;
        }
    };
    let mut guard = ON_POST_UPDATE_QUEUE.lock();
    guard
        .as_mut()
        .unwrap()
        .push_back(Box::new(move || unsafe { go(&args) }));
}

fn parse(command: &str) -> Result<Args, ()> {
    let mut words = command.split_ascii_whitespace();
    words.next().ok_or(())?;
    let x = words.next().ok_or(())?.parse::<f32>().map_err(|_| ())?;
    let y = words.next().ok_or(())?.parse::<f32>().map_err(|_| ())?;
    let z = words.next().ok_or(())?.parse::<f32>().map_err(|_| ())?;
    Ok(Args { x, y, z })
}

#[derive(Debug)]
struct Args {
    x: f32,
    y: f32,
    z: f32,
}

unsafe fn go(args: &Args) {
    let darksiders = gfc::OblivionGame::get_instance();
    let world = darksiders.get_world();

    let class_registry = gfc::Singleton::<gfc::ClassRegistry>::get_instance();
    let class = class_registry
        .class_for_name(&hstring!("StaticObject"), true)
        .unwrap();

    let obj = class.new_instance();
    #[allow(clippy::cast_ptr_alignment)]
    let obj = obj.as_ptr() as *mut target::gfc__StaticObject;

    target::gfc__StaticObject__setPackageName(obj, MAGIC_PACKAGE_NAME.as_ptr());
    target::gfc__StaticObject__setObjectName(obj, MAGIC_OBJECT_NAME.as_ptr());
    ((*(*obj).__vfptr).setPosition)(
        (*obj).as_gfc__WorldObject_mut_ptr(),
        &target::gfc__TVector3_float_gfc__FloatMath_ {
            x: args.x,
            y: args.y,
            z: args.z,
        },
    );

    ((*(*obj).__vfptr).addObjectToWorld)((*obj).as_gfc__WorldObject_mut_ptr(), world.as_ptr());
}

static MAGIC_PACKAGE_NAME: Lazy<gfc::HString> = Lazy::new(|| hstring!("city01_streets"));
static MAGIC_OBJECT_NAME: Lazy<gfc::HString> = Lazy::new(|| hstring!("injected_object3d_name"));
static MAGIC_MESH_NAME: Lazy<gfc::HString> = Lazy::new(|| hstring!("injected_mesh_name"));

pub fn override_get_object3d(
    package_id: i32,
    object_name: &gfc::HString,
) -> Option<gfc::AutoRef<gfc::Object3D>> {
    if object_name == &*MAGIC_OBJECT_NAME {
        let cache = gfc::Singleton::<gfc::Object3DCache>::get_instance();
        unsafe {
            let object = init_with(|p| {
                target::gfc__Object3DCache__get(
                    cache.as_ptr(),
                    p,
                    package_id,
                    hstring!("city01_glass2_04").as_ptr(),
                );
            });
            let object = gfc::AutoRef::<gfc::Object3D>::from_ptr(object.p);
            #[allow(clippy::cast_ptr_alignment)]
            let visual =
                (*(*object.as_ptr()).mVisuals.mData).p as *mut target::gfc__StaticMeshVisual;
            (*visual).mMeshName = MAGIC_MESH_NAME.clone().into_raw();
            return Some(object);
        }
    }
    None
}

pub fn override_get_static_mesh(
    package_id: i32,
    mesh_name: &gfc::HString,
    idx: i32,
) -> Option<gfc::AutoRef<gfc::StaticMesh>> {
    if mesh_name == &*MAGIC_MESH_NAME {
        let cache = gfc::Singleton::<gfc::KGMeshCache>::get_instance();
        unsafe {
            let mesh = init_with(|p| {
                target::gfc__MeshCache__getStaticMesh(
                    (*cache.as_ptr()).as_gfc__MeshCache_mut_ptr(),
                    p,
                    package_id,
                    hstring!("city01_glass2_04").as_ptr(),
                    idx,
                );
            });
            let mesh = gfc::AutoRef::from_ptr(mesh.p);
            return Some(mesh);
        }
    }
    None
}
