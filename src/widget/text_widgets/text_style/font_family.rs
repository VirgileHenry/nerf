
#[derive(Debug, Clone)]
pub enum FontFamily {
    Name(String),
    Serif,
    SansSerif,
    Monospace,
    Cursive,
    Fantasy,
}

impl<'a> Into<cosmic_text::Family<'a>> for &'a FontFamily {
    fn into(self) -> cosmic_text::Family<'a> {
        match self {
            FontFamily::Name(name) => cosmic_text::Family::Name(&name),
            FontFamily::Serif => cosmic_text::Family::Serif,
            FontFamily::SansSerif => cosmic_text::Family::SansSerif,
            FontFamily::Monospace => cosmic_text::Family::Monospace,
            FontFamily::Cursive => cosmic_text::Family::Cursive,
            FontFamily::Fantasy => cosmic_text::Family::Fantasy,
        }
    }
}


