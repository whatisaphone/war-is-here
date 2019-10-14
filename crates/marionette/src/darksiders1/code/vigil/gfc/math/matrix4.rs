use darksiders1_sys::target;

pub type Matrix4<N> = na::Matrix4<N>;

impl_lift_lower!(Matrix4<f32>, target::gfc__Matrix4);
