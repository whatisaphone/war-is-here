#![allow(clippy::cast_precision_loss)]

use na::{Point3, Transform3};
use std::{collections::HashMap, iter};

pub fn box_edges(p1: Point3<f32>, p2: Point3<f32>) -> [[Point3<f32>; 2]; 12] {
    [
        [Point3::new(p1.x, p1.y, p1.z), Point3::new(p2.x, p1.y, p1.z)],
        [Point3::new(p1.x, p1.y, p1.z), Point3::new(p1.x, p2.y, p1.z)],
        [Point3::new(p1.x, p2.y, p1.z), Point3::new(p2.x, p2.y, p1.z)],
        [Point3::new(p2.x, p1.y, p1.z), Point3::new(p2.x, p2.y, p1.z)],
        [Point3::new(p1.x, p1.y, p2.z), Point3::new(p2.x, p1.y, p2.z)],
        [Point3::new(p1.x, p1.y, p2.z), Point3::new(p1.x, p2.y, p2.z)],
        [Point3::new(p1.x, p2.y, p2.z), Point3::new(p2.x, p2.y, p2.z)],
        [Point3::new(p2.x, p1.y, p2.z), Point3::new(p2.x, p2.y, p2.z)],
        [Point3::new(p1.x, p1.y, p1.z), Point3::new(p1.x, p1.y, p2.z)],
        [Point3::new(p2.x, p1.y, p1.z), Point3::new(p2.x, p1.y, p2.z)],
        [Point3::new(p1.x, p2.y, p1.z), Point3::new(p1.x, p2.y, p2.z)],
        [Point3::new(p2.x, p2.y, p1.z), Point3::new(p2.x, p2.y, p2.z)],
    ]
}

// Based on http://blog.andreaskahler.com/2009/06/creating-icosphere-mesh-in-code.html
pub fn icosphere() -> impl Iterator<Item = [Point3<f32>; 2]> {
    let phi = (1.0 + 5.0_f32.sqrt()) / 2.0;
    let norm = (1.0 + phi * phi).sqrt();
    let a = 1.0 / norm;
    let b = phi / norm;
    let mut verts = vec![
        Point3::new(-a, b, 0.0),
        Point3::new(a, b, 0.0),
        Point3::new(-a, -b, 0.0),
        Point3::new(a, -b, 0.0),
        Point3::new(0.0, -a, b),
        Point3::new(0.0, a, b),
        Point3::new(0.0, -a, -b),
        Point3::new(0.0, -a, -b),
        Point3::new(b, 0.0, -a),
        Point3::new(b, 0.0, a),
        Point3::new(-b, 0.0, -a),
        Point3::new(-b, 0.0, a),
    ];

    let faces = vec![
        // 5 faces around point 0
        [0, 11, 5],
        [0, 5, 1],
        [0, 1, 7],
        [0, 7, 10],
        [0, 10, 11],
        // 5 adjacent faces
        [1, 5, 9],
        [5, 11, 4],
        [11, 10, 2],
        [10, 7, 6],
        [7, 1, 8],
        // 5 faces around point 3
        [3, 9, 4],
        [3, 4, 2],
        [3, 2, 6],
        [3, 6, 8],
        [3, 8, 9],
        // 5 adjacent faces
        [4, 9, 5],
        [2, 4, 11],
        [6, 2, 10],
        [8, 6, 7],
        [9, 8, 1],
    ];

    let mut split_edge_cache = HashMap::new();
    let mut split_edge = |p, q, num, denom| {
        if num == 0 || p == q {
            p
        } else if num == denom {
            q
        } else {
            *split_edge_cache.entry((p, q, num)).or_insert_with(|| {
                let frac = num as f32 / denom as f32;
                let vert: Point3<_> = verts[p] + (verts[q] - verts[p]) * frac;
                let vert = Point3::from(vert.coords.normalize());
                verts.push(vert);
                verts.len() - 1
            })
        }
    };

    let mut split_faces = Vec::new();
    for &[a, b, c] in &faces {
        let count = 3;
        for row in 0..count {
            let first_d = split_edge(a, b, row, count);
            let last_d = split_edge(a, c, row, count);
            let first_e = split_edge(a, b, row + 1, count);
            let last_f = split_edge(a, c, row + 1, count);

            let cols = row + 1;
            for col in 0..cols {
                let d = split_edge(first_d, last_d, col, cols - 1);
                let e = split_edge(first_e, last_f, col, cols);
                let f = split_edge(first_e, last_f, col + 1, cols);
                split_faces.push([d, e, f]);

                if col != cols - 1 {
                    let e = split_edge(first_d, last_d, col + 1, cols - 1);
                    split_faces.push([d, e, f]);
                }
            }
        }
    }

    // TODO: dedup the edges?
    split_faces.into_iter().flat_map(move |[a, b, c]| {
        iter::once([verts[a], verts[b]])
            .chain(iter::once([verts[b], verts[c]]))
            .chain(iter::once([verts[c], verts[a]]))
    })
}

pub fn cylinder(resolution: usize) -> impl Iterator<Item = [Point3<f32>; 2]> {
    let mut result = Vec::with_capacity(resolution * 3);
    for i in 0..resolution {
        let rad1 = (i as f32 / resolution as f32) * 2.0 * std::f32::consts::PI;
        let rad2 = ((i + 1) as f32 / resolution as f32) * 2.0 * std::f32::consts::PI;
        let (y1, x1) = rad1.sin_cos();
        let (y2, x2) = rad2.sin_cos();
        result.push([Point3::new(x1, y1, -1.0), Point3::new(x2, y2, -1.0)]);
        result.push([Point3::new(x1, y1, 1.0), Point3::new(x2, y2, 1.0)]);
        result.push([Point3::new(x1, y1, -1.0), Point3::new(x1, y1, 1.0)]);
    }
    result.into_iter()
}

pub fn transform(wireframe: &mut [[Point3<f32>; 2]], transform: &Transform3<f32>) {
    for [p, q] in wireframe {
        *p = transform * *p;
        *q = transform * *q;
    }
}
