

/// Describes how horizontal text overflows are handled.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TextOverflow {
    /// Clip the text at a pixel level
    ClipPixels,
    /// Clip the text at a character level
    ClipChar,
    /// Clip the text at a word level
    ClipWord,
    /// Replace the overflow with an ellipsis at a char level
    CharEllipsis,
    /// Replace the overflow with an ellipsis at a word level 
    WordEllipsis,
    /// Put the overflow on a new line. This might cause vertical overflow,
    /// Which is clipped.
    NewLine,
}

impl Into<cosmic_text::Wrap> for TextOverflow {
    fn into(self) -> cosmic_text::Wrap {
        match self {
            TextOverflow::ClipPixels => cosmic_text::Wrap::None,
            TextOverflow::ClipChar => cosmic_text::Wrap::Glyph,
            TextOverflow::ClipWord => cosmic_text::Wrap::Word,
            TextOverflow::CharEllipsis => cosmic_text::Wrap::Glyph,
            TextOverflow::WordEllipsis => cosmic_text::Wrap::Word,
            TextOverflow::NewLine => cosmic_text::Wrap::Word,
        }
    }
}