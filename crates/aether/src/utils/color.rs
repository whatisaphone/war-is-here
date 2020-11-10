#[derive(Copy, Clone)]
pub struct Rgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Rgb {
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Rgb { red, green, blue }
    }
}

impl From<Rgb> for [f32; 3] {
    fn from(rgb: Rgb) -> Self {
        fn chan(x: u8) -> f32 {
            f32::from(x) / f32::from(u8::max_value())
        }

        [chan(rgb.red), chan(rgb.green), chan(rgb.blue)]
    }
}
