#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]

use crate::{
    darksiders1::{gfc, Lift, Lift1, List, Lower, LoweredAutoRef},
    hooks::ON_POST_UPDATE_QUEUE,
    library::bitmap_font,
    utils::{liang_barsky::liang_barsky, mem::init_with},
};
use darksiders1_sys::target;
use na::{Matrix4, Point2, Vector3, Vector4};
use pdbindgen_runtime::StaticCast;
use std::{
    convert::TryFrom,
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
    ENABLED.store(true, Ordering::SeqCst);

    walk(&mut |object| {
        if let Some(trigger) = gfc::object_safecast::<gfc::TriggerRegion>(object) {
            mark(&trigger);
        }
    });
}

unsafe fn walk(visitor: &mut dyn FnMut(&gfc::WorldObject)) {
    let world = gfc::OblivionGame::get_instance().get_world();

    let root = (*world.as_ptr()).mRoot.lift_ref();
    walk_group(root, visitor);

    let region_data = (*world.as_ptr()).mRegionData.lift1_ref();
    for (r, _) in region_data.iter().enumerate() {
        let r = i32::try_from(r).unwrap();
        let region = init_with(|p| {
            target::gfc__World__getRegion(world.as_ptr(), p, r);
        });
        let region = region.ptr();
        if region.is_null() {
            continue;
        }

        let layer_data = (*region).mLayerData.lift1_ref();
        for (l, _) in layer_data.iter().enumerate() {
            let l = i32::try_from(l).unwrap();
            let layer = init_with(|p| {
                target::gfc__WorldRegion__getLayer(region, p, l);
            });
            let layer = layer.ptr();
            if layer.is_null() {
                continue;
            }

            let root = init_with(|p| {
                target::gfc__RegionLayer__getRoot(layer, p);
            });
            let root = root.lift();
            walk_group(&root, visitor);
        }
    }
}

unsafe fn walk_group(group: &gfc::WorldGroup, visitor: &mut dyn FnMut(&gfc::WorldObject)) {
    let objects = &mut (*group.as_ptr()).mObjects;
    let objects = List::<target::gfc__AutoRef_gfc__WorldObject_>::from_ptr(objects);
    for object in objects {
        let object = object.lift_ref();

        visitor(object);

        if let Some(group) = gfc::object_safecast::<gfc::WorldGroup>(object) {
            walk_group(&group, visitor);
        }
    }
}

unsafe fn mark(trigger: &gfc::TriggerRegion) {
    let position = init_with(|p| {
        (*trigger.as_ptr()).getPosition(p);
    });
    add_marker(
        (*trigger.as_ptr()).mRegionID,
        (*trigger.as_ptr()).mLayerID,
        position.x,
        position.y,
        position.z,
    );
}

unsafe fn add_marker(region_id: u16, layer_id: u16, x: f32, y: f32, z: f32) {
    let class = gfc::Singleton::<gfc::ClassRegistry>::get_instance()
        .class_for_name(&hstring!("StaticObject"), true)
        .unwrap();
    let obj = class.new_instance();
    let obj = obj.as_ptr().cast::<target::gfc__StaticObject>();

    target::gfc__WorldObject__setRegionID(obj.static_cast(), region_id);
    target::gfc__WorldObject__setLayerID(obj.static_cast(), layer_id);
    target::gfc__StaticObject__setPackageName(obj, hstring!("vfx_shared").as_ptr());
    target::gfc__StaticObject__setObjectName(obj, hstring!("sphere").as_ptr());
    (*obj).setPosition(&target::gfc__TVector3_float_gfc__FloatMath_ { x, y, z });

    let world = gfc::OblivionGame::get_instance().get_world();
    (*obj).addObjectToWorld(world.as_ptr());
}

static ENABLED: AtomicBool = AtomicBool::new(false);

pub unsafe fn draw(renderer: *mut target::gfc__UIRenderer) {
    if !ENABLED.load(Ordering::SeqCst) {
        return;
    }

    target::gfc__UIRenderer__begin(renderer, true);
    target::gfc__UIRenderer__setMaterial(renderer, (*renderer).mSolidMaterial.ptr());

    let transformer = CoordinateTransformer::create();

    walk(&mut |object| {
        let trigger_region = match gfc::object_safecast::<gfc::TriggerRegion>(object) {
            Some(o) => o,
            _ => return,
        };

        let position = init_with(|p| {
            (*trigger_region.as_ptr()).getPosition(p);
        })
        .lift();
        let screen = transformer.world_to_screen(&position);

        if screen.z >= 0.0 && screen.z < 500.0 {
            let class_name = trigger_region
                .class()
                .name()
                .c_str()
                .to_str()
                .unwrap_or("<invalid utf-8>");
            bitmap_font::draw_string(renderer, screen.x, screen.y, class_name);

            let object_name = (*trigger_region.as_ptr())
                .mName
                .lift_ref()
                .c_str()
                .to_str()
                .unwrap_or("<invalid utf-8>");
            bitmap_font::draw_string(renderer, screen.x, screen.y + 20.0, object_name);

            match get_shape(&trigger_region) {
                Shape::Aabb(b0x) => {
                    draw_wireframe_box(renderer, &b0x);
                }
                Shape::Other(s) => {
                    bitmap_font::draw_string(renderer, screen.x, screen.y + 40.0, s);
                }
            };
        }
    });

    target::gfc__UIRenderer__endRendering(renderer);
}

unsafe fn get_shape(object: &gfc::TriggerRegion) -> Shape {
    match object.shape() {
        gfc::PhysicsShapeObject__Detect::Aabb => {
            Shape::Aabb(std::ptr::read(&(*object.as_ptr()).mBounds))
        }
        gfc::PhysicsShapeObject__Detect::Box => Shape::Other("box"),
        gfc::PhysicsShapeObject__Detect::Sphere => Shape::Other("sphere"),
        gfc::PhysicsShapeObject__Detect::Cylinder => Shape::Other("cylinder"),
    }
}

pub enum Shape {
    Aabb(target::gfc__TBox_float_gfc__FloatMath_),
    Other(&'static str),
}

unsafe fn draw_wireframe_box(
    renderer: *mut target::gfc__UIRenderer,
    b0x: &target::gfc__TBox_float_gfc__FloatMath_,
) {
    let transformer = CoordinateTransformer::create();

    for (p, q) in &[
        (
            Vector3::new(b0x.min.x, b0x.min.y, b0x.min.z),
            Vector3::new(b0x.max.x, b0x.min.y, b0x.min.z),
        ),
        (
            Vector3::new(b0x.min.x, b0x.min.y, b0x.min.z),
            Vector3::new(b0x.min.x, b0x.max.y, b0x.min.z),
        ),
        (
            Vector3::new(b0x.min.x, b0x.max.y, b0x.min.z),
            Vector3::new(b0x.max.x, b0x.max.y, b0x.min.z),
        ),
        (
            Vector3::new(b0x.max.x, b0x.min.y, b0x.min.z),
            Vector3::new(b0x.max.x, b0x.max.y, b0x.min.z),
        ),
        (
            Vector3::new(b0x.min.x, b0x.min.y, b0x.max.z),
            Vector3::new(b0x.max.x, b0x.min.y, b0x.max.z),
        ),
        (
            Vector3::new(b0x.min.x, b0x.min.y, b0x.max.z),
            Vector3::new(b0x.min.x, b0x.max.y, b0x.max.z),
        ),
        (
            Vector3::new(b0x.min.x, b0x.max.y, b0x.max.z),
            Vector3::new(b0x.max.x, b0x.max.y, b0x.max.z),
        ),
        (
            Vector3::new(b0x.max.x, b0x.min.y, b0x.max.z),
            Vector3::new(b0x.max.x, b0x.max.y, b0x.max.z),
        ),
        (
            Vector3::new(b0x.min.x, b0x.min.y, b0x.min.z),
            Vector3::new(b0x.min.x, b0x.min.y, b0x.max.z),
        ),
        (
            Vector3::new(b0x.max.x, b0x.min.y, b0x.min.z),
            Vector3::new(b0x.max.x, b0x.min.y, b0x.max.z),
        ),
        (
            Vector3::new(b0x.min.x, b0x.max.y, b0x.min.z),
            Vector3::new(b0x.min.x, b0x.max.y, b0x.max.z),
        ),
        (
            Vector3::new(b0x.max.x, b0x.max.y, b0x.min.z),
            Vector3::new(b0x.max.x, b0x.max.y, b0x.max.z),
        ),
    ] {
        let p = transformer.world_to_screen(&p);
        let q = transformer.world_to_screen(&q);
        clunky_draw_line(renderer, Point2::new(p.x, p.y), Point2::new(q.x, q.y));
    }
}

unsafe fn clunky_draw_line(renderer: *mut target::gfc__UIRenderer, p: Point2<f32>, q: Point2<f32>) {
    let viewport = gfc::KGGraphics::get_instance().get_viewport();
    let viewport = viewport.convert(|x| x as f32);
    let (p, q, steps) = match plan_line(&viewport, p, q) {
        Some(plan) => plan,
        None => return,
    };
    for i in 0..steps {
        let step = i as f32;
        let (x, y, w, h) = positive_rect(
            p.x + (q.x - p.x) * (step / steps as f32),
            p.y + (q.y - p.y) * (step / steps as f32),
            (q.x - p.x) / steps as f32,
            (q.y - p.y) / steps as f32,
        );
        target::gfc__UIRenderer__fillRect(
            renderer,
            x,
            y,
            w.max(1.0),
            h.max(1.0),
            &Lower::lower(gfc::TVector4::new(0.0, 0.0, 1.0, 1.0)),
            &Lower::lower(gfc::TVector4::new(0.0, 0.0, 1.0, 1.0)),
        );
    }
}

fn plan_line(
    viewport: &gfc::TRect<f32>,
    p: Point2<f32>,
    q: Point2<f32>,
) -> Option<(Point2<f32>, Point2<f32>, i32)> {
    let [p, q] = liang_barsky(viewport, p, q)?;

    let mut steps = f32::min((q.x - p.x).abs(), (q.y - p.y).abs()) as i32;
    steps = (steps / 2).max(1);
    if steps > 100 {
        steps = 100;
    }

    Some((p, q, steps))
}

fn positive_rect(x: f32, y: f32, w: f32, h: f32) -> (f32, f32, f32, f32) {
    let (x, w) = if w >= 0.0 { (x, w) } else { (x + w, -w) };
    let (y, h) = if h >= 0.0 { (y, h) } else { (y + h, -h) };
    (x, y, w, h)
}

struct CoordinateTransformer {
    viewport_width: f32,
    viewport_height: f32,
    view_proj: Matrix4<f32>,
}

impl CoordinateTransformer {
    pub fn create() -> Self {
        unsafe {
            let viewport = gfc::KGGraphics::get_instance().get_viewport();
            let viewport_width = viewport.width() as f32;
            let viewport_height = viewport.height() as f32;

            let darksiders = gfc::OblivionGame::get_instance();
            let view = init_with(|p| {
                target::gfc__OblivionGame__getViewMatrix(darksiders.as_ptr().static_cast(), p);
            })
            .lift();
            let proj = init_with(|p| {
                target::gfc__OblivionGame__getProjMatrix(darksiders.as_ptr().static_cast(), p);
            })
            .lift();

            Self {
                viewport_width,
                viewport_height,
                view_proj: proj * view,
            }
        }
    }

    pub fn world_to_screen(&self, world: &Vector3<f32>) -> Vector3<f32> {
        let world_homo = Vector4::new(world.x, world.y, world.z, 1.0);
        let screen = self.view_proj * world_homo;
        let x = (1.0 + screen.x / screen.w) * self.viewport_width / 2.0;
        let y = (1.0 - screen.y / screen.w) * self.viewport_height / 2.0;
        if screen.z >= 0.0 {
            Vector3::new(x, y, screen.z)
        } else {
            Vector3::new(-x, -y, screen.z)
        }
    }
}
