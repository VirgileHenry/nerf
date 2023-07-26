pub(crate) mod app;
pub(crate) mod drawing;
pub(crate) mod geometry;
pub mod prelude;
pub(crate) mod widget;

// default app exports
pub use app::{
    app_context::AppContext,
    event::{
        event_responses::EventResponse,
        input_event::{mouse_position::MousePosition, InputEvent},
    },
    App,
};
pub use drawing::{canvas::Canvas, color::Color};
pub use geometry::{
    alignment::{Alignment, HorizontalAlignment, VerticalAlignment},
    decoration::BorderType,
    rect::Rect,
    screen_side::ScreenSide,
    size_requirements::WidgetSizeRequirement,
};
// re-export winit
pub use winit;
// default widget exports
pub use widget::{
    default_widgets::{
        align::Align,
        background::Background,
        button::Button,
        center::Center,
        column::Column,
        empty::Empty,
        padder::{PaddType, Padder},
        row::Row,
        scaffold::Scaffold,
        sized_box::SizedBox,
        stack::Stack,
    },
    Widget,
};
// conditional exports
#[cfg(feature = "skia")]
pub use widget::skia_widgets::decorated_background::DecoratedBackground;
#[cfg(feature = "text")]
pub use widget::text_widgets::{
    text::Text,
    text_style::{font_family::FontFamily, FontCharSpacing, FontStyle, FontWeight, TextStyle},
};
