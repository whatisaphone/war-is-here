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
    let scale = words.next().ok_or(())?.parse::<f32>().map_err(|_| ())?;
    Ok(Args { x, y, z, scale })
}

#[derive(Debug)]
struct Args {
    x: f32,
    y: f32,
    z: f32,
    scale: f32,
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
    ((*(*obj).__vfptr).setScale)(
        (*obj).as_gfc__WorldObject_mut_ptr(),
        &target::gfc__TVector3_float_gfc__FloatMath_ {
            x: args.scale,
            y: args.scale,
            z: args.scale,
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

    let builder = build_cube_meshbuilder();
    let builder = builder.p as *mut target::gfc__MeshBuilder;

    unsafe {
        let result = init_with(|p| {
            ((*(*graphics.as_ptr()).__vfptr).createStaticMesh)(
                (*graphics.as_ptr()).as_gfc__Graphics_mut_ptr(),
                p,
                builder,
            );
        });
        gfc::AutoRef::from_ptr(result.p)
    }
}

fn build_cube_meshbuilder() -> target::gfc__AutoRef_gfc__MeshBuilder_ {
    unsafe {
        let mut builder = new(|| {
            init_with(|this| {
                target::gfc__MeshBuilder__MeshBuilder(this);
            })
        });

        (*builder).mBounds = target::gfc__BoundingVolume {
            b: target::gfc__TBox_float_gfc__FloatMath_ {
                min: target::gfc__TVector3_float_gfc__FloatMath_ {
                    x: -100.0,
                    y: -100.0,
                    z: -100.0,
                },
                max: target::gfc__TVector3_float_gfc__FloatMath_ {
                    x: 100.0,
                    y: 100.0,
                    z: 100.0,
                },
            },
            s: target::gfc__TSphere_float_gfc__FloatMath_ {
                center: target::gfc__TVector3_float_gfc__FloatMath_ {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                radius: 0.0,
            },
            r#type: 0,
        };

        let vertex_format_format =
            gfc::Vector::<u16>::from_ptr_mut(&mut (*builder).mVertexFormat.mFormat);
        // wtf are these
        vertex_format_format.add(1);
        vertex_format_format.add(2);
        vertex_format_format.add(12);
        vertex_format_format.add(14);
        vertex_format_format.add(4);
        vertex_format_format.add(4);
        vertex_format_format.add(4);
        vertex_format_format.add(7);

        let sub_mesh = new(|| {
            let mut sub_mesh = init_with(|this| target::gfc__MBSubMesh__MBSubMesh(this));
            sub_mesh.PrimType = 0;
            sub_mesh.MaterialID = 0;
            sub_mesh.MaterialName = gfc::String::from_cstr(cstr!("City01_glass")).into_raw();
            sub_mesh.VertexCount = 8;

            let position = gfc::Vector::<gfc::TVector3<f32>>::from_ptr_mut(&mut sub_mesh.Position);
            position.add(gfc::TVector3::new(-1.0, -1.0, -1.0));
            position.add(gfc::TVector3::new(1.0, -1.0, -1.0));
            position.add(gfc::TVector3::new(1.0, 1.0, -1.0));
            position.add(gfc::TVector3::new(-1.0, 1.0, -1.0));
            position.add(gfc::TVector3::new(-1.0, 1.0, 1.0));
            position.add(gfc::TVector3::new(1.0, 1.0, 1.0));
            position.add(gfc::TVector3::new(1.0, -1.0, 1.0));
            position.add(gfc::TVector3::new(-1.0, -1.0, 1.0));

            let normal = gfc::Vector::<gfc::TVector3<f32>>::from_ptr_mut(&mut sub_mesh.Normal);
            normal.add(gfc::TVector3::new(0.0, 0.0, 1.0));
            normal.add(gfc::TVector3::new(0.0, 0.0, 1.0));
            normal.add(gfc::TVector3::new(0.0, 0.0, 1.0));
            normal.add(gfc::TVector3::new(0.0, 0.0, 1.0));
            normal.add(gfc::TVector3::new(0.0, 0.0, 1.0));
            normal.add(gfc::TVector3::new(0.0, 0.0, 1.0));
            normal.add(gfc::TVector3::new(0.0, 0.0, 1.0));
            normal.add(gfc::TVector3::new(0.0, 0.0, 1.0));

            let tangent = gfc::Vector::<gfc::TVector4<f32>>::from_ptr_mut(&mut sub_mesh.Tangent);
            tangent.add(gfc::TVector4::new(1.0, 0.0, 0.0, 1.0));
            tangent.add(gfc::TVector4::new(1.0, 0.0, 0.0, 1.0));
            tangent.add(gfc::TVector4::new(1.0, 0.0, 0.0, 1.0));
            tangent.add(gfc::TVector4::new(1.0, 0.0, 0.0, 1.0));
            tangent.add(gfc::TVector4::new(1.0, 0.0, 0.0, 1.0));
            tangent.add(gfc::TVector4::new(1.0, 0.0, 0.0, 1.0));
            tangent.add(gfc::TVector4::new(1.0, 0.0, 0.0, 1.0));
            tangent.add(gfc::TVector4::new(1.0, 0.0, 0.0, 1.0));

            let binormal = gfc::Vector::<gfc::TVector3<f32>>::from_ptr_mut(&mut sub_mesh.Binormal);
            binormal.add(gfc::TVector3::new(0.0, 1.0, 0.0));
            binormal.add(gfc::TVector3::new(0.0, 1.0, 0.0));
            binormal.add(gfc::TVector3::new(0.0, 1.0, 0.0));
            binormal.add(gfc::TVector3::new(0.0, 1.0, 0.0));
            binormal.add(gfc::TVector3::new(0.0, 1.0, 0.0));
            binormal.add(gfc::TVector3::new(0.0, 1.0, 0.0));
            binormal.add(gfc::TVector3::new(0.0, 1.0, 0.0));
            binormal.add(gfc::TVector3::new(0.0, 1.0, 0.0));

            let tex0 = gfc::Vector::<gfc::TVector3<f32>>::from_ptr_mut(&mut sub_mesh.Tex0);
            tex0.add(gfc::TVector3::new(0.970, -0.033, 0.0));
            tex0.add(gfc::TVector3::new(0.970, -0.599, 0.0));
            tex0.add(gfc::TVector3::new(0.759, -0.599, 0.0));
            tex0.add(gfc::TVector3::new(0.759, -0.033, 0.0));
            tex0.add(gfc::TVector3::new(0.970, -0.599, 0.0));
            tex0.add(gfc::TVector3::new(0.970, -0.033, 0.0));
            tex0.add(gfc::TVector3::new(0.759, -0.033, 0.0));
            tex0.add(gfc::TVector3::new(0.759, -0.599, 0.0));

            let tex1 = gfc::Vector::<gfc::TVector3<f32>>::from_ptr_mut(&mut sub_mesh.Tex1);
            tex1.add(gfc::TVector3::new(-198.432, 94.423, 0.0));
            tex1.add(gfc::TVector3::new(-198.432, 94.423, 0.0));
            tex1.add(gfc::TVector3::new(-198.432, 110.157, 0.0));
            tex1.add(gfc::TVector3::new(-198.432, 110.157, 0.0));
            tex1.add(gfc::TVector3::new(-198.432, 94.423, 0.0));
            tex1.add(gfc::TVector3::new(-198.432, 94.423, 0.0));
            tex1.add(gfc::TVector3::new(-198.432, 110.157, 0.0));
            tex1.add(gfc::TVector3::new(-198.432, 110.157, 0.0));

            let tex2 = gfc::Vector::<gfc::TVector3<f32>>::from_ptr_mut(&mut sub_mesh.Tex2);
            tex2.add(gfc::TVector3::new(0.454, 67.799, 0.0));
            tex2.add(gfc::TVector3::new(0.454, 0.201, 0.0));
            tex2.add(gfc::TVector3::new(23.546, 0.201, 0.0));
            tex2.add(gfc::TVector3::new(23.546, 67.799, 0.0));
            tex2.add(gfc::TVector3::new(0.454, 0.201, 0.0));
            tex2.add(gfc::TVector3::new(0.454, 67.799, 0.0));
            tex2.add(gfc::TVector3::new(23.546, 67.799, 0.0));
            tex2.add(gfc::TVector3::new(23.546, 0.201, 0.0));

            let color0 = gfc::Vector::<gfc::TVector4<f32>>::from_ptr_mut(&mut sub_mesh.Color0);
            color0.add(gfc::TVector4::new(255.0, 255.0, 255.0, 255.0));
            color0.add(gfc::TVector4::new(255.0, 255.0, 255.0, 255.0));
            color0.add(gfc::TVector4::new(255.0, 255.0, 255.0, 255.0));
            color0.add(gfc::TVector4::new(255.0, 255.0, 255.0, 255.0));
            color0.add(gfc::TVector4::new(255.0, 255.0, 255.0, 255.0));
            color0.add(gfc::TVector4::new(255.0, 255.0, 255.0, 255.0));
            color0.add(gfc::TVector4::new(255.0, 255.0, 255.0, 255.0));
            color0.add(gfc::TVector4::new(255.0, 255.0, 255.0, 255.0));

            let indices = gfc::Vector::from_ptr_mut(&mut sub_mesh.Indices);
            indices.add(0);
            indices.add(1);
            indices.add(2);

            indices.add(2);
            indices.add(3);
            indices.add(0);

            indices.add(0);
            indices.add(1);
            indices.add(6);

            indices.add(6);
            indices.add(7);
            indices.add(0);

            indices.add(0);
            indices.add(3);
            indices.add(4);

            indices.add(4);
            indices.add(7);
            indices.add(0);

            indices.add(1);
            indices.add(2);
            indices.add(5);

            indices.add(5);
            indices.add(6);
            indices.add(1);

            indices.add(2);
            indices.add(3);
            indices.add(4);

            indices.add(4);
            indices.add(5);
            indices.add(2);

            indices.add(4);
            indices.add(5);
            indices.add(6);

            indices.add(6);
            indices.add(7);
            indices.add(4);

            sub_mesh
        });
        let sub_mesh = gfc::AutoRef::<target::gfc__MBSubMesh>::from_ptr(sub_mesh as *mut _);

        let sub_meshes = gfc::Vector::<target::gfc__AutoRef_gfc__MBSubMesh_>::from_ptr_mut(
            &mut (*builder).mSubMeshes,
        );
        sub_meshes.add(target::gfc__AutoRef_gfc__MBSubMesh_ {
            p: gfc::AutoRef::into_ptr(sub_mesh),
        });

        (*builder).mFlags.flags = 31;

        let builder = gfc::AutoRef::<target::gfc__MeshBuilder>::from_ptr(
            (*(*builder).as_gfc__Object_mut_ptr()).as_gfc__IRefObject_mut_ptr(),
        );
        target::gfc__AutoRef_gfc__MeshBuilder_ {
            p: gfc::AutoRef::into_ptr(builder),
        }
    }
}
