

pub(crate) mod app;
pub(crate) mod drawing;
pub(crate) mod geometry;
pub(crate) mod widget;
pub(crate) mod utils;

// default app exports
pub use app::{
    run_app,
    event::AppEvent,
    app_context::AppContext,
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
    decoration::BorderType,
    rect::Rect,
    screen_side::ScreenSide,
    size_requirements::WidgetSizeRequirement,
};
pub use utils::nonable::Nonable;
// re-export winit
pub use winit;
// default widget exports
pub use widget::{
    Widget,
    default_widgets::{
        align::Align,
        background::Background,
        button::Button,
        center::Center,
        empty::Empty,
        padder::{Padder, PaddType},
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
