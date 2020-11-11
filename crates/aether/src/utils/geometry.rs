use na::{Isometry3, Point3, Unit, Vector3};
use ncollide3d::{
    query::{Ray, RayCast},
    shape::Plane,
};

pub struct Plane3 {
    abc: Vector3<f32>,
    d: f32,
}

impl Plane3 {
    pub fn new(abc: Vector3<f32>, d: f32) -> Self {
        Plane3 { abc, d }
    }

    pub fn distance_to_point(&self, p: &Point3<f32>) -> f32 {
        self.abc.dot(&p.coords) + self.d
    }

    /// Calculates whether a point is inside this plane's positive half-space.
    pub fn contains_point(&self, p: &Point3<f32>) -> bool {
        self.distance_to_point(p) >= 0.0
    }

    pub fn to_ncollide(&self) -> (Plane<f32>, Isometry3<f32>) {
        // `ncollide` requires normalization, but we don't. (Should we?)
        let (abc, norm) = Unit::new_and_get(self.abc);
        let d = self.d / norm;

        let plane = Plane::new(abc);
        let isometry = Isometry3::new(*abc * -d, Vector3::zeros());
        (plane, isometry)
    }
}

#[derive(Clone)]
pub struct LineSegment3 {
    pub p: Point3<f32>,
    pub q: Point3<f32>,
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

    pub fn clip_to_plane(&self, plane: &Plane3) -> Option<Self> {
        let (p, q) = (self.p, self.q);
        match (plane.contains_point(&p), plane.contains_point(&q)) {
            (true, true) => Some(self.clone()),
            (false, false) => None,
            (false, true) => {
                let p = Self::intersect_ray_with_plane(&p, &q, plane).unwrap_or(p);
                Some(Self::new(p, q))
            }
            (true, false) => {
                let q = Self::intersect_ray_with_plane(&q, &p, plane).unwrap_or(q);
                Some(Self::new(p, q))
            }
        }
    }

    fn intersect_ray_with_plane(
        p: &Point3<f32>,
        q: &Point3<f32>,
        plane: &Plane3,
    ) -> Option<Point3<f32>> {
        // This should be using lines, not rays, but rays let me be lazy and use
        // ncollide instead of figuring out the math.
        //
        // If it used lines we could get rid of the `Option`.

        let (plane, iso) = plane.to_ncollide();
        let pq = Ray::new(*p, q - p);
        let t = plane.toi_with_ray(&iso, &pq, f32::INFINITY, false)?;
        Some(pq.point_at(t))
    }
}
