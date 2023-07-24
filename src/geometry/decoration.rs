use tiny_skia::PathBuilder;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BorderType(u8);


impl BorderType {
    pub const ROUND_NONE: BorderType = BorderType(0b0000_0000);
    pub const ROUND_TOP_LEFT: BorderType = BorderType(0b0000_0001);
    pub const ROUND_TOP_RIGHT: BorderType = BorderType(0b0000_0010);
    pub const ROUND_BOTTOM_RIGHT: BorderType = BorderType(0b0000_10100);
    pub const ROUND_BOTTOM_LEFT: BorderType = BorderType(0b0000_1000);
    pub const ROUND_TOP: BorderType = BorderType(0b0000_0011);
    pub const ROUND_BOTTOM: BorderType = BorderType(0b0000_1100);
    pub const ROUND_LEFT: BorderType = BorderType(0b0000_1001);
    pub const ROUND_RIGHT: BorderType = BorderType(0b0000_1001);
    pub const ROUND_ALL: BorderType = BorderType(0b0000_1111);
}


#[cfg(feature = "skia")]
impl BorderType {
    pub fn build_path(&self, path: &mut PathBuilder, rect: softbuffer::Rect, border_width: &mut u32, radius: u32) {

        // radius can not be smaller than the border width, nor bigger than half the rect size
        let radius = radius.max(*border_width).min((rect.width.get() >> 1).min(rect.height.get() >> 1));
        // the border width can not be bigger than half the rect size
        *border_width = (*border_width).min(rect.width.get() >> 1).min(rect.height.get() >> 1);

        let border_offset = *border_width >> 1;
        // top left corner
        if self.0 & 1 > 0 {
            path.move_to(
                (rect.x + border_offset) as f32,
                (rect.y + radius) as f32
            );
            path.quad_to(
                (rect.x + border_offset) as f32,
                (rect.y + border_offset) as f32,
                (rect.x + radius) as f32,
                (rect.y + border_offset) as f32,
            );
        }
        else {
            path.move_to(
                (rect.x + border_offset) as f32,
                (rect.y + border_offset) as f32
            );
        }
        // top right corner
        if self.0 & (1 << 1) > 0 {
            path.line_to(
                (rect.x + rect.width.get() - radius) as f32,
                (rect.y + border_offset) as f32
            );
            path.quad_to(
                (rect.x + rect.width.get() - border_offset) as f32,
                (rect.y + border_offset) as f32,
                (rect.x + rect.width.get() - border_offset) as f32,
                (rect.y + radius) as f32,
            );
        }
        else {
            path.line_to(
                (rect.x + rect.width.get() - border_offset) as f32,
                (rect.y + border_offset) as f32
            );
        }
        // bottom right corner
        if self.0 & (1 << 2) > 0 {
            path.line_to(
                (rect.x + rect.width.get() - border_offset) as f32,
                (rect.y + rect.height.get()) as f32 - radius as f32,
            );
            path.quad_to(
                (rect.x + rect.width.get() - border_offset) as f32,
                (rect.y + rect.height.get() - border_offset) as f32,
                (rect.x + rect.width.get()) as f32 - radius as f32,
                (rect.y + rect.height.get() - border_offset) as f32,
            );
        }
        else {
            path.line_to(
                (rect.x + rect.width.get() - border_offset) as f32,
                (rect.y + rect.height.get() - border_offset) as f32
            );
        }
        // bottom left corner
        if self.0 & (1 << 3) > 0 {
            path.line_to(
                (rect.x + radius) as f32,
                (rect.y + rect.height.get() - border_offset) as f32,
            );
            path.quad_to(
                (rect.x + border_offset) as f32,
                (rect.y + rect.height.get() - border_offset) as f32,
                (rect.x + border_offset) as f32,
                (rect.y + rect.height.get()) as f32 - radius as f32,
            );
        }
        else {
            path.line_to(
                (rect.x + border_offset) as f32,
                (rect.y + rect.height.get() - border_offset) as f32
            );
        }
        // close the path
        path.close();

    }

}


impl std::ops::BitOr for BorderType {
    type Output = BorderType;

    fn bitor(self, rhs: Self) -> Self::Output {
        BorderType(self.0 | rhs.0)
    }
}