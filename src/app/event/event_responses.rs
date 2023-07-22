
/// The event response is the returned value f the handle_event function.
/// It is like an enum, implemented as an u64 to combine values. 
/// Widgets that want to provide info to their parents about what happen should use this.
/// It is important to throw this back to our own parent.
/// All of the default values are bits at the extreme left of the u64, so the user can add it's own events at the right.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct EventResponse(u64);

impl EventResponse {
    /// No responses.
    pub const NONE: EventResponse = EventResponse(0);
    /// The widget request a redraw.
    pub const REDRAW_REQUEST: EventResponse = EventResponse(1 << 63);
    /// A callback have been triggered on the widget. Usually, the press of a button.
    pub const CALLBACK: EventResponse = EventResponse(1 << 62);
    /// The widget request an animation frame.
    /// Requesting an animation frame requires the engine to redraw, then resend an animation event.
    pub const REQUEST_ANIMATION_FRAME: EventResponse = EventResponse(1 << 61);
}

impl std::ops::BitAnd for EventResponse {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        EventResponse(self.0 & rhs.0)
    }
}

impl std::ops::BitOr for EventResponse {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        EventResponse(self.0 | rhs.0)
    }
}

impl std::ops::Not for EventResponse {
    type Output = Self;

    fn not(self) -> Self::Output {
        EventResponse(!self.0)
    }
}

impl EventResponse {
    pub fn contains(self, other: EventResponse) -> bool {
        self.0 & other.0 > 0
    }
}



