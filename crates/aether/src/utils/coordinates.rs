use na::Point2;

#[derive(Clone, Default, Debug)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub fn from_xywh(x: f32, y: f32, w: f32, h: f32) -> Self {
        Rect { x, y, w, h }
    }

    pub fn from_ltrb(l: f32, t: f32, r: f32, b: f32) -> Self {
        Rect {
            x: l,
            y: t,
            w: r - l,
            h: b - t,
        }
    }

    pub fn right(&self) -> f32 {
        self.x + self.w
    }

    pub fn bottom(&self) -> f32 {
        self.y + self.h
    }

    pub fn center(&self) -> Point2<f32> {
        Point2::new(self.x + self.w / 2.0, self.y + self.h / 2.0)
    }

    pub fn union(&self, other: &Self) -> Self {
        Self::from_ltrb(
            self.x.min(other.x),
            self.y.min(other.y),
            self.right().max(other.right()),
            self.bottom().max(other.bottom()),
        )
    }

    pub fn intersection(&self, other: &Self) -> Option<Self> {
        let (al, at, ar, ab) = (self.x, self.y, self.right(), self.bottom());
        let (bl, bt, br, bb) = (other.x, other.y, other.right(), other.bottom());
        let result = Self::from_ltrb(al.max(bl), at.max(bt), ar.min(br), ab.min(bb));
        if result.w <= 0.0 || result.h <= 0.0 {
            return None;
        }
        Some(result)
    }
}
