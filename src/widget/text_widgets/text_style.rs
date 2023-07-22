use crate::Color;

use self::font_family::FontFamily;

use super::text::{text_overflow::TextOverflow, TextAlign};

pub(crate) mod font_family;

pub type FontWeight = cosmic_text::Weight;
pub type FontCharSpacing = cosmic_text::Stretch;
pub type FontStyle = cosmic_text::Style;

#[derive(Debug, Clone)]
pub struct TextStyle {
    pub size: f32,
    pub additional_interline: f32,
    pub family: FontFamily,
    pub weight: FontWeight,
    pub spacing: FontCharSpacing,
    pub style: FontStyle,
    pub color: Color,
    pub overflow: TextOverflow,
    pub align: TextAlign,
}

impl TextStyle {
    pub fn sized(self, size: f32) -> TextStyle {
        TextStyle {
            size,
            ..self
        }
    }

    pub fn interlined(self, additional_interline: f32) -> TextStyle {
        TextStyle {
            additional_interline,
            ..self
        }
    }

    pub fn with_font(self, family: FontFamily) -> TextStyle {
        TextStyle {
            family,
            ..self
        }
    }

    pub fn weighted(self, weight: FontWeight) -> TextStyle {
        TextStyle {
            weight,
            ..self
        }
    }

    pub fn spaced(self, spacing: FontCharSpacing) -> TextStyle {
        TextStyle {
            spacing,
            ..self
        }
    }

    pub fn styled(self, style: FontStyle) -> TextStyle {
        TextStyle {
            style,
            ..self
        }
    }

    pub fn colored(self, color: Color) -> TextStyle {
        TextStyle {
            color,
            ..self
        }
    }

    pub fn with_overflow(self, overflow: TextOverflow) -> TextStyle {
        TextStyle {
            overflow,
            ..self
        }
    }

    pub fn aligned(self, align: TextAlign) -> TextStyle {
        TextStyle {
            align,
            ..self
        }
    }
}

impl<'a> Into<cosmic_text::Attrs<'a>> for &'a TextStyle {
    fn into(self) -> cosmic_text::Attrs<'a> {
        cosmic_text::Attrs::new()
            .family((&self.family).into())
            .weight(self.weight)
            .stretch(self.spacing)
            .style(self.style)
    }
}

impl Default for TextStyle {
    fn default() -> Self {
        TextStyle {
            size: 14.0,
            additional_interline: 0.0,
            family: FontFamily::SansSerif,
            weight: FontWeight::NORMAL,
            spacing: FontCharSpacing::Normal,
            style: FontStyle::Normal,
            color: Color::BLACK,
            overflow: TextOverflow::ClipPixels,
            align: TextAlign::Left,
        }
    }
}