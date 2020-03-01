#![allow(dead_code)]

pub use code::vigil::{
    gfc::{
        base::{
            autoref::LoweredAutoRef,
            object::{Lift, Lift1, Lower},
        },
        memory::memoverloads::Heap,
        util::list::List,
    },
    libmodules::havok::hk2014_2_5_r1::source::{
        common::base::object::{
            hkbaseobject::hkBaseObject,
            hkreferencedobject::hkReferencedObject,
        },
        geometry::collide::shapes::hkcdshape::{hkcdShape, hkcdShapeType__ShapeTypeEnum},
        physics2012::collide::shape::{
            convex::{
                convexvertices::hkpconvexverticesshape::hkpConvexVerticesShape,
                hkpconvexshape::hkpConvexShape,
            },
            heightfield::hkpsphererepshape::hkpSphereRepShape,
            hkpshape::hkpShape,
            hkpshapebase::hkpShapeBase,
        },
    },
};

mod code;
pub mod gfc;
pub mod keen;

include!(concat!(env!("OUT_DIR"), "/cast.rs"));
