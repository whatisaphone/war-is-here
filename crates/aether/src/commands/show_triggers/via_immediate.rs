use crate::{
    commands::show_triggers::{
        shape::{get_shape, CachedShapeQuery, Shape},
        walk::walk_world,
    },
    darksiders1::{gfc, gfc::Reflect},
    utils::{
        arrayvec::ArrayVecExt,
        color::Rgb,
        color_schemes::colorbrewer,
        coordinate_transformer::CoordinateTransformer,
        geometry::LineSegment3,
        interpolate::{lerp, unlerp},
        meshes::{box_edges, cylinder, transform, uv_sphere},
        na_ext::{Transform3Ext, UnitComplexExt},
    },
};
use arrayvec::ArrayVec;
use imgui::{im_str, WindowDrawList};
use itertools::{iproduct, Itertools};
use na::{Isometry3, Point3, Transform3, Translation, UnitComplex, UnitQuaternion, Vector3};
use ncollide3d::{
    query::{PointQuery, Ray},
    shape::Cuboid,
};
use ordered_float::NotNan;
use std::{cmp::Ordering, collections::BTreeSet, f32::consts::PI, mem};

const MIN_CLOSE_DISTANCE: f32 = 500.0;
const MIN_INSIDE_DISTANCE: f32 = 250.0;

pub fn draw(ui: &imgui::Ui<'_>) {
    let player = match gfc::OblivionGame::get_instance().get_player_actor() {
        Some(player) => player,
        None => return,
    };
    let player_pos = player.get_position();

    let transformer = CoordinateTransformer::create();

    // Sort objects into multiple groups, so we can have categories of objects which
    // are always drawn.
    let mut load_regions = KeepMinCountOrMinPriority::new(1);
    let mut others = KeepMinCountOrMinPriority::new(3);

    walk_world(&mut |object| {
        let object = match gfc::object_safecast::<gfc::DetectorObject>(object) {
            Some(o) => o,
            None => return,
        };

        let category = categorize_object(&object);
        let priority = prioritize_object(&object, &player_pos);
        match category {
            Category::LoadRegion => load_regions.feed(object, priority),
            Category::Other => others.feed(object, priority),
        }
    });

    imgui_token_guard! {
        ui.push_style_var(imgui::StyleVar::WindowBorderSize(0.0));
        ui.push_style_color(imgui::StyleColor::WindowBg, [0.0, 0.0, 0.0, 0.0]);
    }

    imgui::Window::new(im_str!("Triggerrs"))
        .position([0.0, 0.0], imgui::Condition::Always)
        .title_bar(false)
        .resizable(false)
        .build(ui, || {
            let draw_list = ui.get_background_draw_list();
            for object in load_regions.into_iter() {
                draw_object(&draw_list, &transformer, &object, &player_pos);
            }
            for object in others.into_iter() {
                draw_object(&draw_list, &transformer, &object, &player_pos);
            }
        });
}

fn draw_object(
    draw_list: &WindowDrawList<'_>,
    transformer: &CoordinateTransformer,
    object: &gfc::DetectorObject,
    reference_pos: &Point3<f32>,
) {
    let position = object.get_position();
    let screen = transformer.world_to_screen(&position);

    let color = colorbrewer::qualitative::SET3
        [(object as *const _) as usize / 16 % colorbrewer::qualitative::SET3.len()];

    // Only draw text if the position is in front of the camera.
    let draw_text = screen.z > 0.0;
    if draw_text {
        let class_name = object.class().name();
        let class_name = class_name.c_str().to_str().unwrap_or("<invalid utf-8>");
        draw_list.add_text([screen.x, screen.y], <[f32; 3]>::from(color), class_name);

        let object_name = object.get_name();
        let object_name = object_name.c_str().to_str().unwrap_or("<invalid utf-8>");
        draw_list.add_text(
            [screen.x, screen.y + 10.0],
            <[f32; 3]>::from(color),
            object_name,
        );
    }

    match get_shape(&object) {
        Shape::Aabb(bounds) => {
            let wireframe = box_edges(bounds.min, bounds.max);
            let wireframe = color_wireframe(&wireframe, reference_pos, color);
            draw_colored_wireframe(&draw_list, &wireframe, transformer);
        }
        Shape::Box(size, isometry) => {
            let origin = Point3::origin();
            let mut wireframe = box_edges(origin - size / 2.0, origin + size / 2.0);
            transform(&mut wireframe, &na::convert(isometry));
            let wireframe = color_wireframe(&wireframe, reference_pos, color);
            draw_colored_wireframe(&draw_list, &wireframe, transformer);
        }
        Shape::Sphere(radius, center) => {
            let mut wireframe: Vec<_> = uv_sphere(12, 16);
            let tf = Translation::from(center.coords)
                * Transform3::from_scaling(&Vector3::new(radius, radius, radius));
            transform(&mut wireframe, &tf);
            let wireframe = color_wireframe(&wireframe, reference_pos, color);
            draw_colored_wireframe(&draw_list, &wireframe, transformer);
        }
        Shape::Cylinder(radius, length, pos) => {
            let mut wireframe: Vec<_> = cylinder(16).collect();
            let tf = Translation::from(pos.coords)
                * Transform3::from_scaling(&Vector3::new(radius, radius, length));
            transform(&mut wireframe, &tf);
            let wireframe = color_wireframe(&wireframe, reference_pos, color);
            draw_colored_wireframe(&draw_list, &wireframe, transformer);
        }
    }
}

struct KeepMinCountOrMinPriority {
    min_count: usize,
    set: BTreeSet<PrioritizedObject>,
}

impl KeepMinCountOrMinPriority {
    fn new(min_count: usize) -> Self {
        Self {
            min_count,
            set: BTreeSet::new(),
        }
    }

    fn into_iter(self) -> impl Iterator<Item = gfc::AutoRef<gfc::DetectorObject>> {
        self.set
            .into_iter()
            .map(|PrioritizedObject { object, .. }| object)
    }

    fn feed(&mut self, object: gfc::AutoRef<gfc::DetectorObject>, priority: Priority) {
        self.set.insert(PrioritizedObject { object, priority });
        // Filter as we go.
        if self.set.len() > self.min_count && !self.set.last().unwrap().priority.force_draw() {
            self.set.pop_last();
        }
    }
}

fn categorize_object(object: &gfc::DetectorObject) -> Category {
    if object.class().instanceof(gfc::LoadRegion::class()) {
        return Category::LoadRegion;
    }
    Category::Other
}

fn prioritize_object(object: &gfc::DetectorObject, point: &Point3<f32>) -> Priority {
    let distance = broad_phase_distance(object, point);
    if distance > NotNan::new(MIN_CLOSE_DISTANCE).unwrap() {
        return Priority::Far(distance);
    }

    narrow_phase(object, point)
}

enum Category {
    LoadRegion,
    Other,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
enum Priority {
    Close(NotNan<f32>),
    InsideClose(NotNan<f32>),
    Far(NotNan<f32>),
    InsideFar(NotNan<f32>),
}

impl Priority {
    fn force_draw(&self) -> bool {
        matches!(self, Self::Close(_) | Self::InsideClose(_))
    }
}

/// Helper struct for sorting objects by priority.
///
/// This compares based on `priority`, and ignores `object` entirely.
struct PrioritizedObject {
    object: gfc::AutoRef<gfc::DetectorObject>,
    priority: Priority,
}

impl PartialEq for PrioritizedObject {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for PrioritizedObject {}

impl PartialOrd for PrioritizedObject {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}

impl Ord for PrioritizedObject {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn broad_phase_distance(object: &gfc::DetectorObject, point: &Point3<f32>) -> NotNan<f32> {
    let bounding_box = object.get_bounding_box();

    let cuboid = Cuboid::new((bounding_box.max - bounding_box.min) / 2.0);
    let isometry = Isometry3::from_parts(
        Translation::from(bounding_box.center().coords),
        UnitQuaternion::identity(),
    );

    let projection = cuboid.project_point(&isometry, point, true);
    let distance = na::distance(point, &projection.point);
    NotNan::new(distance).unwrap()
}

fn narrow_phase(object: &gfc::DetectorObject, point: &Point3<f32>) -> Priority {
    let shape = get_shape(object).to_cached_shape_query();
    let projection = shape.project_point(point, false);

    // Attempt to only take into account the xy plane and ignore the z plane. If
    // you're close to a trigger that covers the entire ground, it's useless to
    // display it all the time, so we try to only draw it when you're near the edge
    // (when moving horizontally).
    //
    // Fall back to ordinary distance if we can't figure it out.
    let distance = distance_along_xy_plane(&shape, point)
        .unwrap_or_else(|| na::distance(point, &projection.point));

    if projection.is_inside {
        if distance <= MIN_INSIDE_DISTANCE {
            Priority::InsideClose(NotNan::new(distance).unwrap())
        } else {
            Priority::InsideFar(NotNan::new(distance).unwrap())
        }
    } else if distance <= MIN_CLOSE_DISTANCE {
        Priority::Close(NotNan::new(distance).unwrap())
    } else {
        Priority::Far(NotNan::new(distance).unwrap())
    }
}

fn distance_along_xy_plane(shape: &CachedShapeQuery, point: &Point3<f32>) -> Option<f32> {
    // Very rough approximation. Cast 8 rays horizontally to approximate the
    // distance to the edge at the current z position.
    iproduct!(
        #[allow(clippy::cast_precision_loss)]
        (0..8).map(|i| 2.0 * PI / 8.0 * i as f32),
        // HACK: do this from the character's feet as well as his head, in case his head is the
        // only thing touching a trigger (for example in CI_03, ci03_mm_trigger_10)
        &[*point, point + Vector3::new(0.0, 0.0, 0.0)]
    )
    .flat_map(|(theta, point)| {
        let rot = UnitComplex::new(theta).around_z_axis();
        let vector = rot * Vector3::x();
        shape.toi_with_ray(&Ray::new(*point, vector), f32::INFINITY, false)
    })
    .map(|x| NotNan::new(x).unwrap())
    .min()
    .map(NotNan::into_inner)
}

fn color_wireframe(
    segments: &[[Point3<f32>; 2]],
    reference_pos: &Point3<f32>,
    color: Rgb,
) -> ColoredWireframe {
    // Convert to more convenient representation

    let mut vertices: Vec<_> = segments.iter().flatten().copied().collect();
    let edges: Vec<[usize; 2]> = (0..vertices.len())
        .tuples()
        .map(|(pi, qi)| [pi, qi])
        .collect();

    // Split edges at the closest point (i.e., maximum weight) if that point is not
    // one of the endpoints. That way the gradient stage can assume all segments
    // have either constant, strictly decreasing, or strictly increasing weight.

    let edges: Vec<_> = edges
        .into_iter()
        .flat_map::<ArrayVec<[_; 2]>, _>(|[pi, qi]| {
            let p = vertices[pi];
            let q = vertices[qi];

            let m = LineSegment3::new(p, q).closest_point(reference_pos);
            if m == p || m == q {
                // Nop (just return back the input changed).
                return ArrayVec::of([[pi, qi]]);
            }
            // Split the segment in two.
            let mi = vertices.len();
            vertices.push(m);
            [[pi, mi], [mi, qi]].into()
        })
        .collect();

    // Give each vertex a weight based on distance. 0 = closest, 1 = furthest

    let mut vertex_weights: Vec<_> = vertices
        .iter()
        .map(|p| {
            let dist = na::distance(p, reference_pos);
            unlerp(dist, 2000.0..500.0).min(1.0)
        })
        .collect();

    // Scale weights so they always cover a minimum range

    let required_range = 0.5;
    let (min_weight, max_weight) = vertex_weights
        .iter()
        .copied()
        .minmax()
        .into_option()
        .unwrap();

    // If all weights are equal, we can't meaningfully scale a single value, so
    // require that minimum != maximum
    if 0.0 < max_weight - min_weight
        // Only do this is the difference is not already what we want
        && max_weight - min_weight < required_range
    {
        let old_min_weight = min_weight;
        let old_max_weight = max_weight;

        let new_min_weight = (max_weight - required_range).max(0.0);
        let new_max_weight = max_weight;

        for weight in &mut vertex_weights {
            let t = unlerp(*weight, old_min_weight..old_max_weight);
            *weight = lerp(t, new_min_weight..new_max_weight);
        }
    }

    // We will fake a gradient by drawing a series of segments, each with a
    // different color. Split the segments at the points where the color will
    // change.

    let edges: Vec<_> = edges
        .into_iter()
        .flat_map(|[mut pi, mut qi]| {
            const COUNT: usize = 4;

            // Make `p` the smaller weight, and `q` the larger weight. It makes the below
            // easier.
            let mut p_weight = vertex_weights[pi];
            let mut q_weight = vertex_weights[qi];
            if p_weight > q_weight {
                mem::swap(&mut pi, &mut qi);
                mem::swap(&mut p_weight, &mut q_weight);
            }

            let p = vertices[pi];
            let q = vertices[qi];

            let mut new_edges = <ArrayVec<[_; COUNT]>>::new();
            new_edges.push([pi, qi]);

            #[allow(clippy::cast_precision_loss)]
            for split in (1..COUNT).map(|i| i as f32 / COUNT as f32) {
                if !(p_weight < split && split < q_weight) {
                    continue;
                }

                let ratio = unlerp(split, p_weight..q_weight);
                let m = lerp(ratio, p..q);
                let mi = vertices.len();
                vertices.push(m);
                vertex_weights.push(split);

                // Split the edge in twain, by changing `[[a, b]]` to `[[a, mid], [mid, b]]`.
                new_edges.last_mut().unwrap()[1] = mi;
                new_edges.push([mi, qi]);
            }

            new_edges
        })
        .collect();

    // Assign a color to each edge based on the weights.

    let edge_colors: Vec<_> = edges
        .iter()
        .copied()
        .map(|[pi, qi]| {
            let p_weight = vertex_weights[pi];
            let q_weight = vertex_weights[qi];
            let weight = p_weight.max(q_weight);

            let [r, g, b]: [f32; 3] = color.into();
            let a = lerp(weight, 0.5_f32..1.0);
            [r, g, b, a]
        })
        .collect();

    // Whew, you made it to the end, have a cookie! 🍪

    ColoredWireframe {
        vertices,
        edges,
        edge_colors,
    }
}

struct ColoredWireframe {
    vertices: Vec<Point3<f32>>,
    edges: Vec<[usize; 2]>,
    edge_colors: Vec<[f32; 4]>,
}

fn draw_colored_wireframe(
    draw_list: &WindowDrawList<'_>,
    ColoredWireframe {
        vertices,
        edges,
        edge_colors,
    }: &ColoredWireframe,
    transformer: &CoordinateTransformer,
) {
    for ([pi, qi], color) in edges.iter().copied().zip(edge_colors.iter().copied()) {
        let p = vertices[pi];
        let q = vertices[qi];

        let [p, q] = match transformer.clip_line_to_frustum_near_plane(p, q) {
            Some([p, q]) => [p, q],
            None => return,
        };

        let p = transformer.world_to_screen(&p);
        let q = transformer.world_to_screen(&q);
        draw_list
            .add_line(p.xy().coords.into(), q.xy().coords.into(), color)
            .thickness(2.0)
            .build();
    }
}
