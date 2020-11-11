pub mod colorbrewer {
    pub mod qualitative {
        use crate::utils::color::Rgb;

        /// Taken from <https://colorbrewer2.org/#type=qualitative&scheme=Pastel2&n=8>,
        /// modified so the colors have approximately constant lightness.
        pub const PASTEL2_MODIFIED: &[Rgb] = &[
            Rgb::new(179, 226, 205),
            Rgb::new(253, 205, 172),
            Rgb::new(203, 213, 232),
            Rgb::new(244, 202, 228),
            Rgb::new(211, 237, 158),
            Rgb::new(255, 233, 123),
            Rgb::new(230, 204, 164),
            Rgb::new(204, 204, 204),
        ];
    }
}
