use darksiders1_sys::target;

pub type Matrix4<T> = na::Matrix4<T>;

impl_lift_lower!(Matrix4<f32>, target::gfc__Matrix4);
