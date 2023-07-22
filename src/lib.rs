

pub(crate) mod app;
pub(crate) mod drawing;
pub(crate) mod geometry;
pub(crate) mod widget;

// default app exports
pub use app::{
    App,
    app_context::AppContext,
    event::{
        input_event::InputEvent,
        event_responses::EventResponse,
    }
};
pub use drawing::{
    canvas::Canvas,
    color::Color,
};
pub use geometry::{
    alignment::{
        VerticalAlignment,
        HorizontalAlignment,
        Alignment,
    },
    rect::Rect,
    screen_side::ScreenSide,
    size_requirements::WidgetSizeRequirement,
};
// default widget exports
pub use widget::{
    Widget,
    default_widgets::{
        align::Align,
        background::Background,
        button::Button,
        center::Center,
        column::Column,
        empty::Empty,
        padder::{Padder, PaddType},
        row::Row,
        sized_box::SizedBox,
        scaffold::Scaffold,
    },
};
// conditional exports
#[cfg(feature = "text")]
pub use widget::text_widgets::{
    text::Text,
    text_style::{
        TextStyle,
        FontWeight,
        FontCharSpacing,
        FontStyle,
        font_family::FontFamily,
    },
};