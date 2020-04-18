#![allow(non_camel_case_types, non_upper_case_globals)]

use super::map;
pub use super::{symbols::*, types::*, types2::*, types3::*, types4::*};
use std::{mem, ptr};

fn transmute_hack() {
    panic!("need to call `bind` first");
}

macro_rules! symbols {
    () => {};
    (
        $($(#[$attr:meta])* pub static $name:ident;)+
    ) => {
        $(
            $(#[$attr])*
            pub static mut $name: *mut map::$name = ptr::null_mut();
            $(#[$attr])*
            pub type $name = map::$name;
        )*
    };
    (
        $($(#[$attr:meta])* pub fn $name:ident;)+
    ) => {
        $(
            $(#[$attr])*
            pub static mut $name: map::$name = unsafe { mem::transmute(transmute_hack as *mut ()) };
            $(#[$attr])*
            pub type $name = map::$name;
        )*
    };
    (
        $($(#[$attr:meta])* pub static $name:ident;)+
        pub fn $($tail:tt)*
    ) => {
        symbols!($($(#[$attr])* pub static $name;)+);
        symbols!(pub fn $($tail)*);
    };
}

symbols! {
    pub static gfc__WorldObject___Class;
    pub static gfc__WorldGroup___Class;
    pub static gfc__CShape___Class;
    pub static gfc__CShapeMesh___Class;
    pub static gfc__CShapeBox___Class;
    pub static gfc__DetectorObject___Class;
    pub static gfc__TriggerRegion___Class;
    pub static gfc__Item___Class;
    pub static gfc__Singleton_gfc__TeleportHelper_gfc__CreateStatic_gfc__DefaultLifetime___InstanceHandle;
    pub static gfc__Singleton_gfc__KGMeshCache_gfc__CreateStatic_gfc__DefaultLifetime___InstanceHandle;
    pub static gfc__Singleton_gfc__KGGraphics_gfc__CreateStatic_gfc__SingletonLongevity__DieLast___InstanceHandle;
    pub static gfc__Singleton_gfc__ResourceManager_gfc__CreateStatic_gfc__DefaultLifetime___InstanceHandle;
    pub static gfc__Singleton_gfc__Darksiders_gfc__CreateStatic_gfc__DefaultLifetime___InstanceHandle;
    pub static gfc__Singleton_gfc__Object3DCache_gfc__CreateStatic_gfc__SingletonLongevity__DieSecond___InstanceHandle;
    pub static gfc__Singleton_gfc__ClassRegistry_gfc__CreateStatic_gfc__SingletonLongevity__DieNextToLast___InstanceHandle;
    pub static gfc__Singleton_gfc__PhysMeshCache_gfc__CreateStatic_gfc__SingletonLongevity__DieSecond___InstanceHandle;
    pub static gfc__Singleton_gfc__DSUIManager_gfc__CreateStatic_gfc__SingletonLongevity__DieFirst___InstanceHandle;
    pub static gfc__Singleton_gfc__WindowHelper_gfc__CreateStatic_gfc__SingletonLongevity__DieFirst___InstanceHandle;
    pub fn gfc__Vector4Parameter__Vector4Parameter;
    pub fn gfc__UIRenderer__setMaterial;
    pub fn gfc__UIRenderer__fillRect;
    pub fn gfc__UIRenderer__drawLine;
    pub fn gfc__UIRenderer__beginRendering;
    pub fn gfc__UIRenderer__endRendering;
    pub fn gfc__UIRenderer__pushClip;
    pub fn gfc__UIRenderer__popClip;
    pub fn gfc__Material__setParameter;
    pub fn gfc__Material__Material;
    pub fn gfc__Material___Material;
    pub fn gfc__UIRenderer__UIRenderer;
    pub fn gfc__UIRenderer__updateTime;
    pub fn gfc__MaterialCache__get;
    pub fn gfc__MaterialCache__get_2;
    pub fn gfc__MeshReader__MeshReader;
    pub fn gfc__MeshCache__getStaticMesh;
    pub fn gfc__MBSubMesh__MBSubMesh;
    pub fn gfc__MeshCache__getStaticMesh_2;
    pub fn gfc__MeshReader__readObject;
    pub fn gfc__MeshCache__loadMesh;
    pub fn gfc__UIRenderer__identity;
    pub fn gfc__UIRenderer__multiplyColor;
    pub fn gfc___UIManager__draw;
    pub fn gfc___UIManager__draw_2;
    pub fn gfc__OblivionGame__getProjMatrix;
    pub fn gfc__OblivionGame__getWorld;
    pub fn gfc__OblivionGame__getViewMatrix;
    pub fn gfc__OblivionGame__update;
    pub fn gfc__StaticObject__setObjectName;
    pub fn gfc__StaticObject__setPackageName;
    pub fn gfc__StaticObject__StaticObject;
    pub fn gfc__StaticObject___StaticObject;
    pub fn gfc__StaticObject__StaticObject_2;
    pub fn gfc__StaticObject__StaticObject_3;
    pub fn gfc__World__getRegion;
    pub fn gfc__WorldObject__removeObjectFromWorld;
    pub fn gfc__WorldObject__attachToObject;
    pub fn gfc__World__World;
    pub fn gfc__Object3DCache__get;
    pub fn gfc__Object3D__Object3D;
    pub fn gfc__Object3DCache__get_2;
    pub fn gfc__StaticMeshVisual__StaticMeshVisual;
    pub fn gfc__StaticMeshVisual__StaticMeshVisual_2;
    pub fn gfc__StaticMeshVisual___StaticMeshVisual;
    pub fn gfc__Object3D___Object3D;
    pub fn gfc__ResourceManager__getPackageID;
    pub fn gfc__ResourceManager__getPackageID_2;
    pub fn gfc__ResourceManager__getPermanentID;
    pub fn gfc__ResourceManager__getPackageID_3;
    pub fn gfc__ResourceManager__loadPackages;
    pub fn gfc__ResourceCache__getResource;
    pub fn gfc__PhysMeshCache__get;
    pub fn gfc__PhysMeshCache__get_2;
    pub fn gfc__HString__HString;
    pub fn gfc__HString__HString_2;
    pub fn gfc__HString__HString_3;
    pub fn gfc__HString__HString_4;
    pub fn gfc__HString__HString_5;
    pub fn gfc__HString__HString_6;
    pub fn gfc__HString___HString;
    pub fn gfc__HString__c_str;
    pub fn gfc__ScriptClass__ScriptClass;
    pub fn gfc__ScriptClass___ScriptClass;
    pub fn gfc__OOObjectWriter__writeObject;
    pub fn gfc__MemFree;
    pub fn gfc__MemAlloc;
    pub fn gfc__ObjectWriter__ObjectWriter;
    pub fn gfc__ByteInputStream___ByteInputStream;
    pub fn gfc__ByteInputStream__ByteInputStream;
    pub fn gfc__ByteInputStream__ByteInputStream_2;
    pub fn gfc__ByteInputStream__ByteInputStream_3;
    pub fn gfc__ByteOutputStream__ByteOutputStream;
    pub fn gfc__ByteOutputStream___ByteOutputStream;
    pub fn gfc__ByteOutputStream__ByteOutputStream_2;
    pub fn gfc__ClassRegistry__classForName;
    pub fn gfc__DetectorRegion__bodyEntered;
    pub fn gfc__PhysicsShapeObject__getTransform;
    pub fn gfc__Player__pickupItem;
    pub fn gfc__Player__pickupItem_2;
    pub fn gfc__Inventory__addItem;
    pub fn gfc__RegionLayer__getRoot;
    pub fn gfc__WorldRegion__getLayer;
    pub fn gfc__Vector4Parameter__Vector4Parameter_2;
    pub fn gfc__Vector4Parameter___Vector4Parameter;
    pub fn gfc__Vector4Parameter__Vector4Parameter_3;
    pub fn gfc__Node3D__getPosition;
    pub fn gfc__ObjectWriter___ObjectWriter;
    pub fn gfc__World__getRegion_2;
    pub fn gfc__Darksiders__processInputEvent;
    pub fn gfc__UIRenderer__translate;
    pub fn gfc__UIRenderer__rotate;
    pub fn gfc__UIRenderer__scale;
    pub fn gfc__UIRenderer__clearShader;
    pub fn gfc__LoadMapMenu__LoadMapMenu;
    pub fn gfc__WindowHelper__pushWindow;
    pub fn gfc__TeleportHelper__warpToMap;
    pub fn gfc__UIRenderer__end;
    pub fn gfc__UIRenderer__setColor;
    pub fn gfc__UIRenderer__setSolidMaterial;
    pub fn gfc__Node3D__getMatrix;
    pub fn gfc__UIRenderer__pushTransform;
    pub fn gfc__UIRenderer__pushParams;
    pub fn gfc__UIRenderer__begin;
    pub fn gfc__UIRenderer__popTransform;
    pub fn gfc__UIRenderer__popParams;
    pub fn gfc__KGGraphics__createStaticMesh;
    pub fn gfc__KGGraphics__createStaticMesh_2;
    pub fn gfc__MeshBuilder__MeshBuilder;
    pub fn gfc__MeshBuilder___MeshBuilder;
    pub fn gfc__String__String;
    pub fn gfc__String___String;
    pub fn gfc__String__String_2;
    pub fn gfc__String__String_3;
    pub fn gfc__String__String_4;
    pub fn gfc__String__String_5;
    pub fn hkpConvexVerticesShape__getOriginalVertices;
}
