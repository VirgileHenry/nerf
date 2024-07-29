use winit::dpi::PhysicalPosition;



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MousePosition {
    OutOfWindow,
    InWindow(u32, u32),
}

impl From<PhysicalPosition<f64>> for MousePosition {
    fn from(value: PhysicalPosition<f64>) -> Self {
        // is this safe enough ? all values are integers, and we are sure they are positive.
        // it's unlikely we'll have any value bigger than u32::MAX.
        MousePosition::InWindow(value.x as u32, value.y as u32)
    }
}

impl MousePosition {
    pub fn is_in_rect(&self, rect: softbuffer::Rect) -> bool {
        match self {
            MousePosition::OutOfWindow => false,
            MousePosition::InWindow(x, y) => {
                x >= &rect.x && x < &(rect.x + rect.width.get()) && y >= &rect.y && y < &(rect.y + rect.height.get())
            }
        }
    }
}