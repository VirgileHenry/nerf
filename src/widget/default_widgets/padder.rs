use std::num::NonZeroU32;

use crate::{
    app::event::AppEvent, geometry::size_requirements::WidgetSizeRequirement, Canvas, Rect, Widget,
};
use crate::utils::nonable::Nonable;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct PaddType(u8);

impl PaddType {
    pub const TOP: PaddType = PaddType(0b0000_0001);
    pub const BOTTOM: PaddType = PaddType(0b0000_0010);
    pub const LEFT: PaddType = PaddType(0b0000_0100);
    pub const RIGHT: PaddType = PaddType(0b0000_1000);
    pub const VERTICAL: PaddType = PaddType(0b0000_0011);
    pub const HORIZONTAL: PaddType = PaddType(0b0000_1100);
    pub const ALL: PaddType = PaddType(0b0000_1111);
    pub const NONE: PaddType = PaddType(0b0000_0000);
}

impl std::ops::BitAnd for PaddType {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        PaddType(self.0 & rhs.0)
    }
}

impl std::ops::BitOr for PaddType {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        PaddType(self.0 | rhs.0)
    }
}

impl std::ops::Not for PaddType {
    type Output = Self;
    fn not(self) -> Self::Output {
        PaddType(!self.0)
    }
}

impl From<PaddType> for u32 {
    fn from(padd_type: PaddType) -> Self {
        u32::from(padd_type.0)
    }
}

/// The Padder widget will draw its child with a padding around it.
/// The padding can be specified for each side of the child. The padding value is the same for all sides.
/// If you want different padding values for each side, you can nest multiple Padder widgets.
struct Padding {
    pub padd_type: PaddType,
    pub padd_amount: u32,
}

impl Padding {
    pub fn top(&self) -> u32 {
        (u32::from(self.padd_type & PaddType::TOP) >> 0) * self.padd_amount
    }

    pub fn bottom(&self) -> u32 {
        (u32::from(self.padd_type & PaddType::BOTTOM) >> 1) * self.padd_amount
    }

    pub fn left(&self) -> u32 {
        (u32::from(self.padd_type & PaddType::LEFT) >> 2) * self.padd_amount
    }

    pub fn right(&self) -> u32 {
        (u32::from(self.padd_type & PaddType::RIGHT) >> 3) * self.padd_amount
    }

    pub fn vertical(&self) -> u32 {
        self.top() + self.bottom()
    }

    pub fn horizontal(&self) -> u32 {
        self.left() + self.right()
    }
}

pub struct Padder<UserEvent, Child: Widget<UserEvent>> {
    _m: core::marker::PhantomData<UserEvent>,
    child: Child,
    padding: Padding,
}

// todo : this whole section is terrible and needs rewrite
impl<UserEvent, Child: Widget<UserEvent>> Padder<UserEvent, Child> {
    pub fn new(padd_type: PaddType, padd_amount: u32, child: Child) -> Self {
        Padder {
            _m: core::marker::PhantomData,
            child,
            padding: Padding { padd_type, padd_amount },
        }
    }

    fn get_child_remaining_width(&self, available_space: NonZeroU32) -> u32 {
        available_space.get().checked_sub(self.padding.horizontal()).unwrap_or(0)
    }

    fn get_child_remaining_height(&self, available_space: NonZeroU32) -> u32 {
        available_space.get().max(self.padding.vertical()) - self.padding.vertical()
    }

    fn get_left_pad(&self, available_space: NonZeroU32, child_space: NonZeroU32) -> u32 {
        if self.padding.left() == 0 {
            return 0;
        }
        else {
            let padd_space = available_space.get() - child_space.get();
            if self.padding.right() == 0 {
                padd_space
            }
            else {
                padd_space / 2
            }
        }
    }

    fn get_top_pad(&self, available_space: NonZeroU32, child_space: NonZeroU32) -> u32 {
        if self.padding.top() == 0 {
            return 0;
        }
        else {
            let padd_space = available_space.get() - child_space.get();
            if self.padding.bottom() == 0 {
                padd_space
            }
            else {
                padd_space / 2
            }
        }
    }

    fn compute_child_rect(&self, from_rect: softbuffer::Rect) -> Option<softbuffer::Rect> {
        let child_width = NonZeroU32::new(self.get_child_remaining_width(from_rect.width))?;
        let child_height = NonZeroU32::new(self.get_child_remaining_height(from_rect.height))?;
        let left_pad = self.get_left_pad(from_rect.width, child_width);
        let top_pad = self.get_top_pad(from_rect.height, child_height);
        Some(softbuffer::Rect {
            x: from_rect.x + left_pad,
            y: from_rect.y + top_pad,
            width: child_width,
            height: child_height,
        })
    }
}

impl<UserEvent, Child: Widget<UserEvent>> Widget<UserEvent> for Padder<UserEvent, Child> {
    type EventResponse = Child::EventResponse;
    fn draw(&self, buffer: &mut Canvas, rect: Rect) {
        match self.compute_child_rect(rect) {
            Some(rect) => self.child.draw(buffer, rect),
            None => {}, // no space left for child, don't draw at all
        };
    }

    fn min_space_requirements(&self) -> (WidgetSizeRequirement, WidgetSizeRequirement) {
        let (child_width_requirement, child_height_requirement) = self.child.min_space_requirements();
        (
            child_width_requirement + self.padding.horizontal(),
            child_height_requirement + self.padding.vertical(),
        )
    }

    fn handle_event(&mut self, event: &AppEvent<UserEvent>, rect: Rect) -> Self::EventResponse {
        match self.compute_child_rect(rect) {
            Some(rect) => self.child.handle_event(event, rect),
            None => Self::EventResponse::none(),
        }
    }
}