use crate::{
    commands::show_triggers::shape::{get_shape, Shape},
    darksiders1::gfc,
    utils::{
        arrayvec::ArrayVecExt,
        color::Rgb,
        color_schemes::colorbrewer,
        coordinate_transformer::CoordinateTransformer,
        geometry::LineSegment3,
        interpolate::{lerp, unlerp},
        meshes::{box_edges, cylinder, transform, uv_sphere},
        na_ext::Transform3Ext,
    },
};
use arrayvec::ArrayVec;
use itertools::Itertools;
use na::{Point3, Transform3, Translation, Vector3};
use std::mem;

#[allow(clippy::module_name_repetitions)]
pub fn draw_object(
    draw_list: &imgui::WindowDrawList<'_>,
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
    draw_list: &imgui::WindowDrawList<'_>,
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
