use crate::{
    darksiders1::{gfc, new},
    hooks::ON_POST_UPDATE_QUEUE,
    utils::mem::init_with,
};
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
    _package_id: i32,
    mesh_name: &gfc::HString,
    _idx: i32,
) -> Option<gfc::AutoRef<gfc::StaticMesh>> {
    if mesh_name == &*MAGIC_MESH_NAME {
        // let mesh = use_mesh_from_game(package_id);
        let mesh = build_cube_mesh();
        return Some(mesh);
    }
    None
}

#[allow(dead_code)]
fn use_mesh_from_game(package_id: i32) -> gfc::AutoRef<gfc::StaticMesh> {
    let cache = gfc::Singleton::<gfc::KGMeshCache>::get_instance();
    unsafe {
        let mesh = init_with(|p| {
            target::gfc__MeshCache__getStaticMesh(
                (*cache.as_ptr()).as_gfc__MeshCache_mut_ptr(),
                p,
                package_id,
                hstring!("city01_glass2_04").as_ptr(),
                0,
            );
        });
        gfc::AutoRef::from_ptr(mesh.p)
    }
}

fn build_cube_mesh() -> gfc::AutoRef<gfc::StaticMesh> {
    let graphics = gfc::KGGraphics::get_instance();
    let builder = read_cooked_meshbuilder();

    unsafe {
        let builder = builder.p as *mut target::gfc__MeshBuilder;

        let factor = 4.0;

        let bounds = &mut (*builder).mBounds;
        bounds.b.min.x *= factor;
        bounds.b.min.y *= factor;
        bounds.b.min.z *= factor;
        bounds.b.max.x *= factor;
        bounds.b.max.y *= factor;
        bounds.b.max.z *= factor;
        bounds.s.radius *= factor;

        let sub_meshes = gfc::Vector::<target::gfc__AutoRef_gfc__MBSubMesh_>::from_ptr_mut(
            &mut (*builder).mSubMeshes,
        );
        let sub_mesh = sub_meshes[0].p as *mut target::gfc__MBSubMesh;
        let position = gfc::Vector::<gfc::TVector3<f32>>::from_ptr_mut(&mut (*sub_mesh).Position);
        for p in position {
            p.x *= factor;
            p.y *= factor;
            p.z *= factor;
        }
    }

    unsafe {
        let result = init_with(|p| {
            ((*(*graphics.as_ptr()).__vfptr).createStaticMesh)(
                (*graphics.as_ptr()).as_gfc__Graphics_mut_ptr(),
                p,
                builder.p.cast(),
            );
        });
        gfc::AutoRef::from_ptr(result.p)
    }
}

fn read_cooked_meshbuilder() -> target::gfc__AutoRef_gfc__MeshBuilder_ {
    unsafe {
        let stream = new(|| gfc::ByteInputStream::new(COOKED));
        let stream = gfc::AutoRef::<gfc::ByteInputStream>::from_ptr(
            (*(*(*(*stream).as_ptr()).as_gfc__InputStream_mut_ptr()).as_gfc__Stream_mut_ptr())
                .as_gfc__IRefObject_mut_ptr(),
        );
        let mut reader = init_with(|this| target::gfc__MeshReader__MeshReader(this));
        let mut valid = true;
        let object = init_with(|p| {
            target::gfc__MeshReader__readObject(
                &mut reader,
                p,
                autoref_cast!(stream, target::gfc__AutoRef_gfc__InputStream_),
                &mut valid,
            );
        });
        if !valid {
            println!("cooked MeshBuilder is not valid!");
        }
        target::gfc__AutoRef_gfc__MeshBuilder_ { p: object.p }
    }
}

const COOKED: &[u8] = &[
    0x4D, 0x53, 0x48, 0x04, 0x00, 0x01, 0x01, 0x00, 0x01, 0x00, 0x00, 0x4F, 0x02, 0x00, 0x00, 0x78,
    0xDA, 0xE3, 0x60, 0x60, 0x60, 0x60, 0x64, 0x60, 0x62, 0xE0, 0x61, 0xE0, 0x63, 0x60, 0x01, 0x43,
    0x76, 0x86, 0x84, 0xDD, 0x3B, 0x0E, 0x32, 0x30, 0x34, 0xEC, 0xF8, 0x63, 0xD8, 0x7E, 0x08, 0xC8,
    0x76, 0x04, 0x2A, 0x61, 0x00, 0xB2, 0x9D, 0x18, 0x19, 0x20, 0xE0, 0x3F, 0x08, 0xF0, 0x30, 0x38,
    0x67, 0x96, 0x54, 0x1A, 0x18, 0xC6, 0xA7, 0xE7, 0x24, 0x16, 0x17, 0xB3, 0x00, 0x85, 0x41, 0x18,
    0xA6, 0xF5, 0x0D, 0x44, 0xEB, 0x41, 0x98, 0x56, 0x98, 0x31, 0x6F, 0xE0, 0x6C, 0x88, 0xF1, 0x20,
    0x3D, 0x0F, 0x3E, 0x2C, 0xD7, 0x06, 0xF2, 0xF7, 0xAB, 0x27, 0x7C, 0x32, 0x79, 0x08, 0x65, 0x6B,
    0x20, 0xB1, 0xD1, 0xC5, 0x59, 0xC0, 0x8E, 0x68, 0xD8, 0xBF, 0xE2, 0xC7, 0x7F, 0xED, 0xFB, 0xF7,
    0xEE, 0x9B, 0x81, 0xD8, 0x20, 0x3C, 0x59, 0xED, 0xB5, 0x36, 0xEB, 0x83, 0x2B, 0x70, 0x3E, 0xBA,
    0x7C, 0x42, 0x2A, 0x97, 0xCE, 0xCE, 0xBB, 0xAF, 0xC0, 0x7C, 0x90, 0x19, 0x42, 0xC2, 0x39, 0xDB,
    0x80, 0x66, 0x6F, 0x01, 0xF1, 0x8D, 0xAF, 0x67, 0x6C, 0xD3, 0x80, 0xB2, 0x91, 0xC5, 0x3F, 0xF8,
    0xE5, 0xC3, 0xC5, 0x41, 0x7A, 0xC2, 0xD2, 0x2B, 0xEC, 0x73, 0xD7, 0x71, 0xEC, 0x05, 0xB9, 0xA0,
    0x0A, 0xC8, 0x3E, 0x11, 0x2F, 0xB9, 0x1F, 0xC4, 0x96, 0x0E, 0x72, 0xB2, 0xBF, 0x02, 0x65, 0x7F,
    0x0B, 0x74, 0xB2, 0xF7, 0x5F, 0x0F, 0x51, 0x03, 0xD2, 0xB3, 0x23, 0xCF, 0xED, 0x70, 0xD4, 0x8D,
    0x3D, 0x4E, 0x0C, 0x58, 0xD8, 0xC9, 0x01, 0x77, 0xE0, 0xEC, 0x34, 0x28, 0x1B, 0xEC, 0xBF, 0x29,
    0x2F, 0xEC, 0xBE, 0xCD, 0x68, 0x07, 0xF3, 0x41, 0x6C, 0x06, 0x16, 0x3F, 0x3B, 0x10, 0x73, 0x43,
    0xEC, 0x1E, 0x47, 0x06, 0x11, 0x04, 0xFB, 0x1F, 0x54, 0x0D, 0x24, 0x4C, 0xEA, 0x9D, 0x29, 0xC3,
    0x10, 0xC0, 0x06, 0xA5, 0x41, 0xD1, 0xCD, 0x04, 0xC5, 0xCC, 0x0C, 0x08, 0x00, 0x12, 0xFF, 0xCF,
    0xC7, 0xE0, 0x93, 0x99, 0x9E, 0x51, 0x92, 0x9B, 0x58, 0xE0, 0x9C, 0x9F, 0x5F, 0x94, 0x52, 0xFC,
    0x9F, 0x85, 0xA1, 0xA4, 0xA8, 0x34, 0x15, 0x00, 0xA3, 0xDE, 0xB1, 0x80,
];
