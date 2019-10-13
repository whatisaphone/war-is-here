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
