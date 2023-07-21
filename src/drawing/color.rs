
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b, a: 255 }
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }

    pub fn value(self) -> u32 {
        (self.r as u32) << 16 | (self.g as u32) << 8 | (self.b as u32)
    } 

    pub fn r(self) -> u8 {
        self.r
    }

    pub fn g(self) -> u8 {
        self.g
    }

    pub fn b(self) -> u8 {
        self.b
    }

    pub fn a(self) -> u8 {
        self.a
    }

    
}

// default values
impl Color {
    pub const BLACK: Color = Color::rgb(0, 0, 0);
    pub const WHITE: Color = Color::rgb(u8::MAX, u8::MAX, u8::MAX);
}


#[cfg(feature = "skia")]
impl<'a> Into<tiny_skia::Paint<'a>> for Color {
    /// If the skia feature is enabled, this will convert the color into a skia paint.
    fn into(self) -> tiny_skia::Paint<'a> {
        let mut paint = tiny_skia::Paint::default();
        // todo : colors seem to be inverted ? r and b are switched ?
        paint.set_color_rgba8(self.b, self.g, self.r, self.a);
        paint
    }
}


#[cfg(feature = "text")]
impl Into<cosmic_text::Color> for Color {
    /// If the text feature is enabled, this will convert the color into a text color.
    fn into(self) -> cosmic_text::Color {
        cosmic_text::Color::rgba(self.r, self.g, self.b, self.a)
    }
}

#[cfg(feature = "text")]
impl From<cosmic_text::Color> for Color {
    /// If the text feature is enabled, this will convert the text color into a color.
    fn from(value: cosmic_text::Color) -> Self {
        Color::rgba(value.r(), value.g(), value.b(), value.a())
    }
}