use std::num::NonZeroU32;


pub type Rect = softbuffer::Rect;

pub struct NullableRect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}




impl TryFrom<NullableRect> for softbuffer::Rect {
    type Error = ();
    fn try_from(value: NullableRect) -> Result<Self, Self::Error> {
        match (NonZeroU32::new(value.width), NonZeroU32::new(value.height)) {
            (Some(width), Some(height)) => Ok(softbuffer::Rect {
                x: value.x, y: value.y, width, height
            }),
            _ => Err(())
        }
    }
}


