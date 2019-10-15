use na::Point3;

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

pub fn icosahedron() -> impl Iterator<Item = [Point3<f32>; 2]> {
    // http://blog.andreaskahler.com/2009/06/creating-icosphere-mesh-in-code.html
    let phi = (1.0 + 5.0_f32.sqrt()) / 2.0;
    let norm = (1.0 + phi * phi).sqrt();
    let a = 1.0 / norm;
    let b = phi / norm;
    let verts = [
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

    // TODO: remove dups lines
    // TODO: don't allocate
    let indices = vec![
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

    indices.into_iter().flat_map(move |[a, b, c]| {
        // TODO: don't allocate
        vec![[verts[a], verts[b]], [verts[b], verts[c]], [
            verts[c], verts[a],
        ]]
    })
}
