#![allow(clippy::transmute_ptr_to_ptr)]

use super::{map, target};
use pdbindgen_runtime::BindArgs;
use std::mem;

pub unsafe fn bind(args: &BindArgs) {
    macro_rules! bind {
        ($section:ident, $name:ident) => {
            target::$name = mem::transmute(args.$section.add(map::$name));
        };
    }

    bind!(data, gfc__WorldObject___Class);
    bind!(data, gfc__WorldGroup___Class);
    bind!(data, gfc__CShape___Class);
    bind!(data, gfc__CShapeMesh___Class);
    bind!(data, gfc__CShapeBox___Class);
    bind!(data, gfc__DetectorObject___Class);
    bind!(data, gfc__TriggerRegion___Class);
    bind!(data, gfc__Item___Class);
    bind!(
        data,
        gfc__Singleton_gfc__TeleportHelper_gfc__CreateStatic_gfc__DefaultLifetime___InstanceHandle
    );
    bind!(
        data,
        gfc__Singleton_gfc__KGMeshCache_gfc__CreateStatic_gfc__DefaultLifetime___InstanceHandle
    );
    bind!(data, gfc__Singleton_gfc__KGGraphics_gfc__CreateStatic_gfc__SingletonLongevity__DieLast___InstanceHandle);
    bind!(
        data,
        gfc__Singleton_gfc__ResourceManager_gfc__CreateStatic_gfc__DefaultLifetime___InstanceHandle
    );
    bind!(
        data,
        gfc__Singleton_gfc__Darksiders_gfc__CreateStatic_gfc__DefaultLifetime___InstanceHandle
    );
    bind!(data, gfc__Singleton_gfc__Object3DCache_gfc__CreateStatic_gfc__SingletonLongevity__DieSecond___InstanceHandle);
    bind!(data, gfc__Singleton_gfc__ClassRegistry_gfc__CreateStatic_gfc__SingletonLongevity__DieNextToLast___InstanceHandle);
    bind!(data, gfc__Singleton_gfc__PhysMeshCache_gfc__CreateStatic_gfc__SingletonLongevity__DieSecond___InstanceHandle);
    bind!(data, gfc__Singleton_gfc__DSUIManager_gfc__CreateStatic_gfc__SingletonLongevity__DieFirst___InstanceHandle);
    bind!(data, gfc__Singleton_gfc__WindowHelper_gfc__CreateStatic_gfc__SingletonLongevity__DieFirst___InstanceHandle);
    bind!(text, gfc__Vector4Parameter__Vector4Parameter);
    bind!(text, gfc__UIRenderer__setMaterial);
    bind!(text, gfc__UIRenderer__fillRect);
    bind!(text, gfc__UIRenderer__drawLine);
    bind!(text, gfc__UIRenderer__beginRendering);
    bind!(text, gfc__UIRenderer__endRendering);
    bind!(text, gfc__UIRenderer__pushClip);
    bind!(text, gfc__UIRenderer__popClip);
    bind!(text, gfc__Material__setParameter);
    bind!(text, gfc__Material__Material);
    bind!(text, gfc__Material___Material);
    bind!(text, gfc__UIRenderer__UIRenderer);
    bind!(text, gfc__UIRenderer__updateTime);
    bind!(text, gfc__MaterialCache__get);
    bind!(text, gfc__MaterialCache__get_2);
    bind!(text, gfc__MeshReader__MeshReader);
    bind!(text, gfc__MeshCache__getStaticMesh);
    bind!(text, gfc__MBSubMesh__MBSubMesh);
    bind!(text, gfc__MeshCache__getStaticMesh_2);
    bind!(text, gfc__MeshReader__readObject);
    bind!(text, gfc__MeshCache__loadMesh);
    bind!(text, gfc__UIRenderer__identity);
    bind!(text, gfc__UIRenderer__multiplyColor);
    bind!(text, gfc___UIManager__draw);
    bind!(text, gfc___UIManager__draw_2);
    bind!(text, gfc__OblivionGame__getProjMatrix);
    bind!(text, gfc__OblivionGame__getWorld);
    bind!(text, gfc__OblivionGame__getViewMatrix);
    bind!(text, gfc__OblivionGame__update);
    bind!(text, gfc__StaticObject__setObjectName);
    bind!(text, gfc__StaticObject__setPackageName);
    bind!(text, gfc__StaticObject__StaticObject);
    bind!(text, gfc__StaticObject___StaticObject);
    bind!(text, gfc__StaticObject__StaticObject_2);
    bind!(text, gfc__StaticObject__StaticObject_3);
    bind!(text, gfc__World__getRegion);
    bind!(text, gfc__WorldObject__removeObjectFromWorld);
    bind!(text, gfc__WorldObject__attachToObject);
    bind!(text, gfc__World__World);
    bind!(text, gfc__Object3DCache__get);
    bind!(text, gfc__Object3D__Object3D);
    bind!(text, gfc__Object3DCache__get_2);
    bind!(text, gfc__StaticMeshVisual__StaticMeshVisual);
    bind!(text, gfc__StaticMeshVisual__StaticMeshVisual_2);
    bind!(text, gfc__StaticMeshVisual___StaticMeshVisual);
    bind!(text, gfc__Object3D___Object3D);
    bind!(text, gfc__ResourceManager__getPackageID);
    bind!(text, gfc__ResourceManager__getPackageID_2);
    bind!(text, gfc__ResourceManager__getPermanentID);
    bind!(text, gfc__ResourceManager__getPackageID_3);
    bind!(text, gfc__ResourceManager__loadPackages);
    bind!(text, gfc__ResourceCache__getResource);
    bind!(text, gfc__PhysMeshCache__get);
    bind!(text, gfc__PhysMeshCache__get_2);
    bind!(text, gfc__HString__HString);
    bind!(text, gfc__HString__HString_2);
    bind!(text, gfc__HString__HString_3);
    bind!(text, gfc__HString__HString_4);
    bind!(text, gfc__HString__HString_5);
    bind!(text, gfc__HString__HString_6);
    bind!(text, gfc__HString___HString);
    bind!(text, gfc__HString__c_str);
    bind!(text, gfc__ScriptClass__ScriptClass);
    bind!(text, gfc__ScriptClass___ScriptClass);
    bind!(text, gfc__OOObjectWriter__writeObject);
    bind!(text, gfc__MemFree);
    bind!(text, gfc__MemAlloc);
    bind!(text, gfc__ObjectWriter__ObjectWriter);
    bind!(text, gfc__ByteInputStream___ByteInputStream);
    bind!(text, gfc__ByteInputStream__ByteInputStream);
    bind!(text, gfc__ByteInputStream__ByteInputStream_2);
    bind!(text, gfc__ByteInputStream__ByteInputStream_3);
    bind!(text, gfc__ByteOutputStream__ByteOutputStream);
    bind!(text, gfc__ByteOutputStream___ByteOutputStream);
    bind!(text, gfc__ByteOutputStream__ByteOutputStream_2);
    bind!(text, gfc__ClassRegistry__classForName);
    bind!(text, gfc__DetectorRegion__bodyEntered);
    bind!(text, gfc__PhysicsShapeObject__getTransform);
    bind!(text, gfc__Player__pickupItem);
    bind!(text, gfc__Player__pickupItem_2);
    bind!(text, gfc__Inventory__addItem);
    bind!(text, gfc__RegionLayer__getRoot);
    bind!(text, gfc__WorldRegion__getLayer);
    bind!(text, gfc__Vector4Parameter__Vector4Parameter_2);
    bind!(text, gfc__Vector4Parameter___Vector4Parameter);
    bind!(text, gfc__Vector4Parameter__Vector4Parameter_3);
    bind!(text, gfc__Node3D__getPosition);
    bind!(text, gfc__ObjectWriter___ObjectWriter);
    bind!(text, gfc__World__getRegion_2);
    bind!(text, gfc__Darksiders__processInputEvent);
    bind!(text, gfc__Darksiders__doTheMagic);
    bind!(text, gfc__UIRenderer__translate);
    bind!(text, gfc__UIRenderer__rotate);
    bind!(text, gfc__UIRenderer__scale);
    bind!(text, gfc__UIRenderer__clearShader);
    bind!(text, gfc__LoadMapMenu__LoadMapMenu);
    bind!(text, gfc__WindowHelper__pushWindow);
    bind!(text, gfc__TeleportHelper__warpToMap);
    bind!(text, gfc__UIRenderer__end);
    bind!(text, gfc__UIRenderer__setColor);
    bind!(text, gfc__UIRenderer__setSolidMaterial);
    bind!(text, gfc__Node3D__getMatrix);
    bind!(text, gfc__UIRenderer__pushTransform);
    bind!(text, gfc__UIRenderer__pushParams);
    bind!(text, gfc__UIRenderer__begin);
    bind!(text, gfc__UIRenderer__popTransform);
    bind!(text, gfc__UIRenderer__popParams);
    bind!(text, gfc__KGGraphics__createStaticMesh);
    bind!(text, gfc__KGGraphics__createStaticMesh_2);
    bind!(text, gfc__MeshBuilder__MeshBuilder);
    bind!(text, gfc__MeshBuilder___MeshBuilder);
    bind!(text, gfc__String__String);
    bind!(text, gfc__String___String);
    bind!(text, gfc__String__String_2);
    bind!(text, gfc__String__String_3);
    bind!(text, gfc__String__String_4);
    bind!(text, gfc__String__String_5);
    bind!(text, hkpConvexVerticesShape__getOriginalVertices);
}
