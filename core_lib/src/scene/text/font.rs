#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum FontFamily {
    Cursive,
    Fantasy,
    Monospace,
    #[default]
    SansSerif,
    Serif,
}

impl FontFamily {
    pub fn to_glyphon(&self) -> glyphon::Family {
        match self {
            FontFamily::Cursive => glyphon::Family::Cursive,
            FontFamily::Fantasy => glyphon::Family::Fantasy,
            FontFamily::Monospace => glyphon::Family::Monospace,
            FontFamily::SansSerif => glyphon::Family::SansSerif,
            FontFamily::Serif => glyphon::Family::Serif,
        }
    }
}
