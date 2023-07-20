
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


#[cfg(feature = "skia")]
impl<'a> Into<tiny_skia::Paint<'a>> for Color {
    /// If the skia feature is enabled, this will convert the color into a skia paint.
    fn into(self) -> tiny_skia::Paint<'a> {
        let mut paint = tiny_skia::Paint::default();
        paint.set_color_rgba8(self.r, self.g, self.b, self.a);
        paint
    }
}