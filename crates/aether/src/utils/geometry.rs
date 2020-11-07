use na::Point3;

pub struct LineSegment3 {
    p: Point3<f32>,
    q: Point3<f32>,
}

impl LineSegment3 {
    pub fn new(p: Point3<f32>, q: Point3<f32>) -> Self {
        Self { p, q }
    }

    /// Find the point on this line segment with the minimum distance to the
    /// given point.
    pub fn closest_point(&self, a: &Point3<f32>) -> Point3<f32> {
        let n = (self.q - self.p) / na::distance_squared(&self.p, &self.q);
        let v = a - self.p;
        let t = v.dot(&n);
        if t <= 0.0 {
            self.p
        } else if t >= 1.0 {
            self.q
        } else {
            self.p + t * (self.q - self.p)
        }
    }
}
