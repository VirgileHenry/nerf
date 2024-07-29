use crate::{
    app::event::AppEvent, drawing::canvas::Canvas, geometry::size_requirements::WidgetSizeRequirement, utils::nonable::Nonable, Rect
};


pub(crate) mod default_widgets;
#[cfg(feature = "skia")]
pub(crate) mod skia_widgets;
#[cfg(feature = "text")]
pub(crate) mod text_widgets;
#[cfg(feature = "svg")]
pub(crate) mod svg_widgets;



/// The widget trait. All widgets are stored as Box<dyn Widget>.
/// This trait can be used to create custom widgets, that can be implemented from scratch or use a combination of existing widgets.
pub trait Widget<UserEvent = ()> {
    type EventResponse: Nonable;
    /// Draw the widget on the canvas. The given rect is the area the widget should draw in, computed by its parent
    /// with it's size requirements. 
    fn draw(&self, canvas: &mut Canvas, rect: Rect);
    /// Get the size requirements of this widget.
    /// If the widgets requests sized outside of the constraints, they will be given smaller sizes to be drawn in.
    fn min_space_requirements(&self) -> (WidgetSizeRequirement, WidgetSizeRequirement);
    /// Handles an event. Returns true if the event was handled, false otherwise.
    /// This will be called on the root, and need to be propagated down the Widget<UserEvent> for each custom widget implementation, 
    /// 
    /// It is needed to recompute the widgets rect while doing so: events are called one after another, and there is no guarantee
    /// that draw will be called between each event. As events can change widget layouts, it is needed to recompute the rect
    /// to ensure that the next event is handled correctly. 
    fn handle_event(&mut self, event: &AppEvent<UserEvent>, rect: Rect) -> Self::EventResponse;
}