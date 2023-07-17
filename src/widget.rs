use crate::{geometry::size_requirements::WidgetSizeRequirement, drawing::canvas::Canvas};


pub(crate) mod align;
pub(crate) mod background;
pub(crate) mod center;
pub(crate) mod column;
pub(crate) mod empty;
pub(crate) mod expanded;
pub(crate) mod padder;
pub(crate) mod row;
pub(crate) mod scaffold;
pub(crate) mod sized_box;


pub trait Widget {
    fn draw(&self, canvas: &mut Canvas, rect: softbuffer::Rect);
    /// Get the size requirements of this widget.
    /// If the widgets requests sized outside of the constraints, they will be given smaller sizes to be drawn in.
    fn min_space_requirements(&self) -> (WidgetSizeRequirement, WidgetSizeRequirement);
}