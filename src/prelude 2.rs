// default app exports
pub use crate::app::{
    app_context::AppContext,
    event::{
        event_responses::EventResponse,
        input_event::{mouse_position::MousePosition, InputEvent},
    },
    App,
};
pub use crate::drawing::{canvas::Canvas, color::Color};
pub use crate::geometry::{
    alignment::{Alignment, HorizontalAlignment, VerticalAlignment},
    decoration::BorderType,
    rect::Rect,
    screen_side::ScreenSide,
    size_requirements::WidgetSizeRequirement,
};
// re-export winit
pub use winit;
// default widget exports
pub use crate::widget::{
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
pub use crate::widget::skia_widgets::decorated_background::DecoratedBackground;
#[cfg(feature = "text")]
pub use crate::widget::text_widgets::{
    text::Text,
    text_style::{font_family::FontFamily, FontCharSpacing, FontStyle, FontWeight, TextStyle},
};
