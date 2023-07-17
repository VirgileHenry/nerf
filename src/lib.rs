

pub(crate) mod app;
pub(crate) mod drawing;
pub(crate) mod geometry;
pub(crate) mod widget;

pub use widget::{
    // export the widget trait to allow user created widgets.
    Widget,
    // export basic widgets.
    align::Align,
    background::Background,
    center::Center,
    column::Column,
    empty::Empty,
    padder::{Padder, PaddType},
    row::Row,
    sized_box::SizedBox,
    scaffold::Scaffold,
};
pub use app::App;
pub use drawing::color::Color;
pub use geometry::{
    screen_side::ScreenSide,
    alignment::{
        VerticalAlignment,
        HorizontalAlignment,
        Alignment,
    }
};