use std::num::NonZeroU32;

use crate::{
    Widget,
    geometry::{
        size_requirements::WidgetSizeRequirement,
        screen_side::ScreenSide
    }
};

/// The Scaffold is a widget with an app bar and a child.
/// The app bar can be on the top, bottom, left or right of the child.
/// There are no assumptions about the app bar size, and if both appbar and child widget have flex requirements,
/// they will be given equal amounts of space. If you want to fix the size of the appbar, you can wrap it in a SizedBox.
pub struct Scaffold {
    appbar_side: ScreenSide,
    appbar: Box<dyn Widget>,
    child: Box<dyn Widget>,
}

impl Scaffold {
    pub fn new(appbar_side: ScreenSide, appbar: Box<dyn Widget>, child: Box<dyn Widget>) -> Box<Scaffold> {
        Box::new(Scaffold {
            appbar_side,
            appbar,
            child,
        })
    }

    /// returns the width and offset of the appbar, and the width and offset of the child.
    fn get_childs_width_and_offset(&self, available_space: NonZeroU32) -> ((u32, u32), (u32, u32)) {
        match self.appbar_side {
            ScreenSide::Top | ScreenSide::Bottom => ((available_space.get(), 0), (available_space.get(), 0)),
            ScreenSide::Left => {
                let [appbar_width, child_width] = WidgetSizeRequirement::distribute_available_size(
                    [
                        self.appbar.min_space_requirements().0,
                        self.child.min_space_requirements().0,
                    ],
                    available_space
                );
                ((appbar_width, 0), (child_width, appbar_width))
            },
            ScreenSide::Right => {
                let [child_width, appbar_width] = WidgetSizeRequirement::distribute_available_size(
                    [
                        self.child.min_space_requirements().0,
                        self.appbar.min_space_requirements().0,
                    ],
                    available_space
                );
                ((appbar_width, child_width), (child_width, 0))
            },
        }
    }

    /// returns the height and offset of the appbar, and the height and offset of the child.
    fn get_childs_height_and_offset(&self, available_space: NonZeroU32) -> ((u32, u32), (u32, u32)) {
        match self.appbar_side {
            ScreenSide::Left | ScreenSide::Right => ((available_space.get(), 0), (available_space.get(), 0)),
            ScreenSide::Top => {
                let [appbar_height, child_height] = WidgetSizeRequirement::distribute_available_size(
                    [
                        self.appbar.min_space_requirements().1,
                        self.child.min_space_requirements().1,
                    ],
                    available_space
                );
                ((appbar_height, 0), (child_height, appbar_height))
            },
            ScreenSide::Bottom => {
                let [child_height, appbar_height] = WidgetSizeRequirement::distribute_available_size(
                    [
                        self.child.min_space_requirements().1,
                        self.appbar.min_space_requirements().1,
                    ],
                    available_space
                );
                ((appbar_height, child_height), (child_height, 0))
            },
        }
    }



}

impl Widget for Scaffold {
    fn draw(&self, canvas: &mut crate::drawing::canvas::Canvas, rect: softbuffer::Rect) {
        let ((appbar_width, appbar_x_offset), (child_width, child_x_offset)) = self.get_childs_width_and_offset(rect.width);
        let ((appbar_height, appbar_y_offset), (child_height, child_y_offset)) = self.get_childs_height_and_offset(rect.height);
        match (NonZeroU32::new(appbar_width), NonZeroU32::new(appbar_height)) {
            (Some(width), Some(height)) => self.appbar.draw(canvas, softbuffer::Rect {
                x: rect.x + appbar_x_offset,
                y: rect.y + appbar_y_offset,
                width,
                height,
            }),
            _ => {}, // either width or height is 0, so we don't draw the appbar
        };
        match (NonZeroU32::new(child_width), NonZeroU32::new(child_height)) {
            (Some(width), Some(height)) => self.child.draw(canvas, softbuffer::Rect {
                x: rect.x + child_x_offset,
                y: rect.y + child_y_offset,
                width,
                height,
            }),
            _ => {}, // either width or height is 0, so we don't draw the child
        };
    }

    fn min_space_requirements(&self) -> (WidgetSizeRequirement, WidgetSizeRequirement) {
        match self.appbar_side {
            ScreenSide::Top | ScreenSide::Bottom => {
                let (appbar_width_req, appbar_height_req) = self.appbar.min_space_requirements();
                let (child_width_req, child_height_req) = self.child.min_space_requirements();
                (
                    appbar_width_req | child_width_req,
                    appbar_height_req & child_height_req,
                )
            },
            ScreenSide::Left | ScreenSide::Right => {
                let (appbar_width_req, appbar_height_req) = self.appbar.min_space_requirements();
                let (child_width_req, child_height_req) = self.child.min_space_requirements();
                (
                    appbar_width_req & child_width_req,
                    appbar_height_req | child_height_req,
                )
            },
        }
    }
}
