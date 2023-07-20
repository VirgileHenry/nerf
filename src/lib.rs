

pub(crate) mod app;
pub(crate) mod drawing;
pub(crate) mod geometry;
pub(crate) mod widget;

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
    }
};
pub use app::{
    App,
    app_context::AppContext,
};
pub use drawing::{
    canvas::Canvas,
    color::Color,
};
pub use geometry::{
    screen_side::ScreenSide,
    alignment::{
        VerticalAlignment,
        HorizontalAlignment,
        Alignment,
    },
    size_requirements::WidgetSizeRequirement,
};