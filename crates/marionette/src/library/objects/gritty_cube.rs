use crate::{
    darksiders1::{gfc, Heap, Lift, Lift1, Lower, LoweredAutoRef},
    utils::mem::init_with,
};
use darksiders1_sys::target;
use na::Point3;
use once_cell::sync::Lazy;
use pdbindgen_runtime::StaticCast;

pub static PACKAGE_NAME: Lazy<gfc::HString> = Lazy::new(|| hstring!("city01_streets"));
pub static OBJECT_NAME: Lazy<gfc::HString> = Lazy::new(|| hstring!("gritty_cube"));
pub static MESH_NAME: Lazy<gfc::HString> = Lazy::new(|| hstring!("gritty_cube"));
static NODE_NAME: Lazy<gfc::HString> = Lazy::new(|| hstring!("gritty_cube"));

pub fn build_object() -> gfc::AutoRef<gfc::Object3D> {
    unsafe {
        let object = gfc::AutoRef::new(gfc::Object3D::new());

        let skeleton = (*object.as_ptr()).mSkeleton.ptr();

        *(*skeleton).mName.lift_mut() = NODE_NAME.clone();

        let mut visual = Heap::new(init_with(|this| {
            target::gfc__StaticMeshVisual__StaticMeshVisual(this);
        }));
        *visual.mRefNode.lift_mut() = NODE_NAME.clone();
        *visual.mMeshName.lift_mut() = MESH_NAME.clone();
        visual.mMeshID = 0;

        let skeleton_visuals = (*object.as_ptr()).mVisuals.lift1_mut();
        skeleton_visuals.add(LoweredAutoRef::from_ptr(
            Heap::into_raw(visual).static_cast(),
        ));

        object
    }
}

pub fn build_mesh() -> gfc::AutoRef<gfc::StaticMesh> {
    let graphics = gfc::KGGraphics::get_instance();

    let builder = build_meshbuilder();

    let result = unsafe {
        init_with(|p| {
            (*graphics.as_ptr()).createStaticMesh(p, builder.as_ptr());
        })
    };
    result.lift()
}

fn build_meshbuilder() -> gfc::AutoRef<gfc::MeshBuilder> {
    let half_size = 25.0;

    unsafe {
        let builder = gfc::AutoRef::new(gfc::MeshBuilder::new());

        (*builder.as_ptr()).mBounds = target::gfc__BoundingVolume {
            b: Lower::lower(gfc::TBox::new(
                Point3::new(-half_size, -half_size, -half_size),
                Point3::new(half_size, half_size, half_size),
            )),
            s: target::gfc__TSphere_float_gfc__FloatMath_ {
                center: Lower::lower(Point3::origin().coords),
                radius: 0.0,
            },
            r#type: 0,
        };

        let vertex_format_format = (*builder.as_ptr()).mVertexFormat.mFormat.lift_mut();
        // wtf are these
        vertex_format_format.add(1);
        vertex_format_format.add(2);
        vertex_format_format.add(12);
        vertex_format_format.add(14);
        vertex_format_format.add(4);
        vertex_format_format.add(4);
        vertex_format_format.add(4);
        vertex_format_format.add(7);

        let sub_mesh = Heap::new({
            let mut sub_mesh = init_with(|this| {
                target::gfc__MBSubMesh__MBSubMesh(this);
            });
            sub_mesh.PrimType = 0;
            sub_mesh.MaterialID = 0;
            sub_mesh.MaterialName = gfc::String::from_cstr(cstr!("City01_glass")).into_raw();
            sub_mesh.VertexCount = 8;

            let position = sub_mesh.Position.lift_mut();
            position.add(gfc::TVector3::new(-half_size, -half_size, -half_size));
            position.add(gfc::TVector3::new(half_size, -half_size, -half_size));
            position.add(gfc::TVector3::new(half_size, half_size, -half_size));
            position.add(gfc::TVector3::new(-half_size, half_size, -half_size));
            position.add(gfc::TVector3::new(-half_size, half_size, half_size));
            position.add(gfc::TVector3::new(half_size, half_size, half_size));
            position.add(gfc::TVector3::new(half_size, -half_size, half_size));
            position.add(gfc::TVector3::new(-half_size, -half_size, half_size));

            let normal = sub_mesh.Normal.lift_mut();
            normal.add(gfc::TVector3::new(0.0, 0.0, 1.0));
            normal.add(gfc::TVector3::new(0.0, 0.0, 1.0));
            normal.add(gfc::TVector3::new(0.0, 0.0, 1.0));
            normal.add(gfc::TVector3::new(0.0, 0.0, 1.0));
            normal.add(gfc::TVector3::new(0.0, 0.0, 1.0));
            normal.add(gfc::TVector3::new(0.0, 0.0, 1.0));
            normal.add(gfc::TVector3::new(0.0, 0.0, 1.0));
            normal.add(gfc::TVector3::new(0.0, 0.0, 1.0));

            let tangent = sub_mesh.Tangent.lift_mut();
            tangent.add(gfc::TVector4::new(1.0, 0.0, 0.0, 1.0));
            tangent.add(gfc::TVector4::new(1.0, 0.0, 0.0, 1.0));
            tangent.add(gfc::TVector4::new(1.0, 0.0, 0.0, 1.0));
            tangent.add(gfc::TVector4::new(1.0, 0.0, 0.0, 1.0));
            tangent.add(gfc::TVector4::new(1.0, 0.0, 0.0, 1.0));
            tangent.add(gfc::TVector4::new(1.0, 0.0, 0.0, 1.0));
            tangent.add(gfc::TVector4::new(1.0, 0.0, 0.0, 1.0));
            tangent.add(gfc::TVector4::new(1.0, 0.0, 0.0, 1.0));

            let binormal = sub_mesh.Binormal.lift_mut();
            binormal.add(gfc::TVector3::new(0.0, 1.0, 0.0));
            binormal.add(gfc::TVector3::new(0.0, 1.0, 0.0));
            binormal.add(gfc::TVector3::new(0.0, 1.0, 0.0));
            binormal.add(gfc::TVector3::new(0.0, 1.0, 0.0));
            binormal.add(gfc::TVector3::new(0.0, 1.0, 0.0));
            binormal.add(gfc::TVector3::new(0.0, 1.0, 0.0));
            binormal.add(gfc::TVector3::new(0.0, 1.0, 0.0));
            binormal.add(gfc::TVector3::new(0.0, 1.0, 0.0));

            let tex0 = sub_mesh.Tex0.lift_mut();
            tex0.add(gfc::TVector3::new(0.970, -0.033, 0.0));
            tex0.add(gfc::TVector3::new(0.970, -0.599, 0.0));
            tex0.add(gfc::TVector3::new(0.759, -0.599, 0.0));
            tex0.add(gfc::TVector3::new(0.759, -0.033, 0.0));
            tex0.add(gfc::TVector3::new(0.970, -0.599, 0.0));
            tex0.add(gfc::TVector3::new(0.970, -0.033, 0.0));
            tex0.add(gfc::TVector3::new(0.759, -0.033, 0.0));
            tex0.add(gfc::TVector3::new(0.759, -0.599, 0.0));

            let tex1 = sub_mesh.Tex1.lift_mut();
            tex1.add(gfc::TVector3::new(-198.432, 94.423, 0.0));
            tex1.add(gfc::TVector3::new(-198.432, 94.423, 0.0));
            tex1.add(gfc::TVector3::new(-198.432, 110.157, 0.0));
            tex1.add(gfc::TVector3::new(-198.432, 110.157, 0.0));
            tex1.add(gfc::TVector3::new(-198.432, 94.423, 0.0));
            tex1.add(gfc::TVector3::new(-198.432, 94.423, 0.0));
            tex1.add(gfc::TVector3::new(-198.432, 110.157, 0.0));
            tex1.add(gfc::TVector3::new(-198.432, 110.157, 0.0));

            let tex2 = sub_mesh.Tex2.lift_mut();
            tex2.add(gfc::TVector3::new(0.454, 67.799, 0.0));
            tex2.add(gfc::TVector3::new(0.454, 0.201, 0.0));
            tex2.add(gfc::TVector3::new(23.546, 0.201, 0.0));
            tex2.add(gfc::TVector3::new(23.546, 67.799, 0.0));
            tex2.add(gfc::TVector3::new(0.454, 0.201, 0.0));
            tex2.add(gfc::TVector3::new(0.454, 67.799, 0.0));
            tex2.add(gfc::TVector3::new(23.546, 67.799, 0.0));
            tex2.add(gfc::TVector3::new(23.546, 0.201, 0.0));

            let color0 = sub_mesh.Color0.lift_mut();
            color0.add(gfc::TVector4::new(255.0, 255.0, 255.0, 255.0));
            color0.add(gfc::TVector4::new(255.0, 255.0, 255.0, 255.0));
            color0.add(gfc::TVector4::new(255.0, 255.0, 255.0, 255.0));
            color0.add(gfc::TVector4::new(255.0, 255.0, 255.0, 255.0));
            color0.add(gfc::TVector4::new(255.0, 255.0, 255.0, 255.0));
            color0.add(gfc::TVector4::new(255.0, 255.0, 255.0, 255.0));
            color0.add(gfc::TVector4::new(255.0, 255.0, 255.0, 255.0));
            color0.add(gfc::TVector4::new(255.0, 255.0, 255.0, 255.0));

            let indices = sub_mesh.Indices.lift_mut();
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

        let sub_meshes = (*builder.as_ptr()).mSubMeshes.lift1_mut();
        sub_meshes.add(LoweredAutoRef::from_ptr(Heap::into_raw(sub_mesh)));

        (*builder.as_ptr()).mFlags.flags = 31;

        builder
    }
}
