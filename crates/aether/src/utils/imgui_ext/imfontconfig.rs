pub trait ImFontConfigExt {
    fn new() -> Self;
}

impl ImFontConfigExt for imgui::sys::ImFontConfig {
    fn new() -> Self {
        // copy the default constructor from C++
        Self {
            FontDataOwnedByAtlas: true,
            OversampleH: 3,
            OversampleV: 1,
            GlyphMaxAdvanceX: f32::MAX,
            RasterizerMultiply: 1.0,
            EllipsisChar: imgui::sys::ImWchar::MAX,
            ..<_>::default()
        }
    }
}
