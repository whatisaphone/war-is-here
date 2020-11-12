use crate::{
    commands::show_triggers::shape::{get_shape, Shape},
    darksiders1::gfc,
    utils::{
        arrayvec::ArrayVecExt,
        color::Rgb,
        color_schemes::colorbrewer,
        coordinate_transformer::CoordinateTransformer,
        coordinates::Rect,
        geometry::LineSegment3,
        imgui_ext::ImStrExt,
        interpolate::{lerp, unlerp},
        meshes::{box_edges, cylinder, transform, uv_sphere},
        na_ext::Transform3Ext,
    },
};
use arrayvec::ArrayVec;
use imgui::{im_str, ImStr, ImString};
use itertools::Itertools;
use na::{Point3, Transform3, Translation, Vector3};
use std::mem;

#[allow(clippy::module_name_repetitions)]
pub fn draw_object(
    ui: &imgui::Ui<'_>,
    transformer: &CoordinateTransformer,
    object: &gfc::DetectorObject,
    reference_pos: &Point3<f32>,
    label_groups: &mut Vec<LabelGroup>,
) {
    let position = object.get_position();
    let screen = transformer.world_to_screen(&position);

    let draw_list = ui.get_background_draw_list();

    let color = colorbrewer::qualitative::PASTEL2_MODIFIED
        [(object as *const _) as usize / 16 % colorbrewer::qualitative::PASTEL2_MODIFIED.len()];

    // Only draw text if the position is in front of the camera.
    let draw_text = screen.z > 0.0;
    if draw_text {
        fn hstring_to_imstring(hstring: &gfc::HString) -> ImString {
            ImStr::try_from_cstr(hstring.c_str())
                .unwrap_or(im_str!("<invalid utf-8>"))
                .into()
        }

        let mut group = LabelGroup {
            labels: <_>::default(),
            bounds: <_>::default(),
        };

        let mut push_label = |text: &gfc::HString, [x, y]: [f32; 2]| {
            let text = hstring_to_imstring(text);
            let [w, h] = ui.calc_text_size(&text, false, 0.0);
            group.labels.push(Label {
                text,
                bounds: Rect::from_xywh(x - w / 2.0, y, w, h),
                color,
            });
        };

        push_label(object.class().name(), [screen.x, screen.y]);
        push_label(object.get_name(), [screen.x, screen.y + 14.0]);

        group.bounds = group
            .labels
            .iter()
            .map(|l| l.bounds.clone())
            .fold1(|a, b| a.union(&b))
            .unwrap();
        label_groups.push(group);
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

pub struct LabelGroup {
    labels: ArrayVec<[Label; 2]>,
    bounds: Rect,
}

struct Label {
    text: imgui::ImString,
    bounds: Rect,
    color: Rgb,
}

pub fn fix_label_overlaps(groups: &mut [LabelGroup]) {
    for gi in 0..groups.len() {
        let (prev_groups, current_groups) = groups.split_at_mut(gi);
        let group = current_groups.first_mut().unwrap();
        let shift = fix_overlap(&group.bounds, prev_groups);

        group.bounds.y += shift;
        for label in &mut group.labels {
            label.bounds.y += shift;
        }
    }
}

fn fix_overlap(original_bounds: &Rect, groups: &[LabelGroup]) -> f32 {
    let mut existing_center = None;
    let mut bounds = original_bounds.clone();
    while let Some(overlap) = find_overlap(&bounds, groups) {
        let existing_center = existing_center.get_or_insert(overlap);
        let current_center = bounds.center();
        let direction = (current_center.y - existing_center.y).signum();
        let shift = match direction {
            d if d < 0.0 => overlap.y - bounds.bottom(),
            d if d > 0.0 => overlap.bottom() - bounds.y,
            _ => unreachable!(),
        };
        bounds.y += shift;
    }
    bounds.y - original_bounds.y
}

fn find_overlap<'g>(bounds: &Rect, groups: &'g [LabelGroup]) -> Option<&'g Rect> {
    for group in groups {
        if bounds.intersection(&group.bounds).is_some() {
            return Some(&group.bounds);
        }
    }
    None
}

#[allow(clippy::module_name_repetitions)]
pub fn draw_label_groups(ui: &imgui::Ui<'_>, groups: &[LabelGroup]) {
    let draw_list = ui.get_background_draw_list();

    for group in groups {
        for label in &group.labels {
            // Draw drop shadow, then actual text
            draw_list.add_text(
                [label.bounds.x + 1.0, label.bounds.y + 1.0],
                [0.0, 0.0, 0.0],
                &label.text,
            );
            draw_list.add_text([label.bounds.x, label.bounds.y], label.color, &label.text);
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

    // Give each vertex a weight based on distance. Closer distance = higher weight.

    let mut vertex_weights: Vec<_> = vertices
        .iter()
        .map(|p| -na::distance(p, reference_pos))
        .collect();

    // Scale weights so they always cover a constant range

    let (min_weight, max_weight) = vertex_weights
        .iter()
        .copied()
        .minmax()
        .into_option()
        .unwrap();

    for weight in &mut vertex_weights {
        let t = unlerp(*weight, min_weight..max_weight);
        *weight = lerp(t, 0.25_f32..1.0);
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

        let [p, q] = match transformer.clip_line_segment_to_frustum_near_plane(&p, &q) {
            Some([p, q]) => [p, q],
            None => continue,
        };

        let p = transformer.world_to_screen(&p);
        let q = transformer.world_to_screen(&q);
        draw_list
            .add_line(p.xy().coords.into(), q.xy().coords.into(), color)
            .thickness(3.0)
            .build();
    }
}
