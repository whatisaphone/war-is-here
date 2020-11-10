pub mod colorbrewer {
    pub mod qualitative {
        use crate::utils::color::Rgb;

        /// <https://colorbrewer2.org/#type=qualitative&scheme=Set3&n=12>
        pub const SET3: &[Rgb] = &[
            Rgb::new(141, 211, 199),
            Rgb::new(255, 255, 179),
            Rgb::new(190, 186, 218),
            Rgb::new(251, 128, 114),
            Rgb::new(128, 177, 211),
            Rgb::new(253, 180, 98),
            Rgb::new(179, 222, 105),
            Rgb::new(252, 205, 229),
            Rgb::new(217, 217, 217),
            Rgb::new(188, 128, 189),
            Rgb::new(204, 235, 197),
            Rgb::new(255, 237, 111),
        ];
    }
}
