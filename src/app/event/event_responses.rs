
/// The event response is the returned value f the handle_event function.
/// It is like an enum, implemented as an u64 to combine values. 
/// Widgets that want to provide info to their parents about what happen should use this.
/// It is important to transmit this back o our own parent.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct EventResponse(u64);

impl EventResponse {
    pub const NONE: EventResponse = EventResponse(0);
    pub const REDRAW_REQUEST: EventResponse = EventResponse(1 << 63);
    pub const CALLBACK: EventResponse = EventResponse(1 << 62);
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



