

pub enum HorizontalAlignment {
    Left,
    Center,
    Right,
}

pub enum VerticalAlignment {
    Top,
    Center,
    Bottom,
}

pub struct Alignment(HorizontalAlignment, VerticalAlignment);

impl Alignment {
    pub const TOP_LEFT: Alignment = Alignment(HorizontalAlignment::Left, VerticalAlignment::Top);
    pub const TOP_CENTER: Alignment = Alignment(HorizontalAlignment::Center, VerticalAlignment::Top);
    pub const TOP_RIGHT: Alignment = Alignment(HorizontalAlignment::Right, VerticalAlignment::Top);
    pub const CENTER_LEFT: Alignment = Alignment(HorizontalAlignment::Left, VerticalAlignment::Center);
    pub const CENTER: Alignment = Alignment(HorizontalAlignment::Center, VerticalAlignment::Center);
    pub const CENTER_RIGHT: Alignment = Alignment(HorizontalAlignment::Right, VerticalAlignment::Center);
    pub const BOTTOM_LEFT: Alignment = Alignment(HorizontalAlignment::Left, VerticalAlignment::Bottom);
    pub const BOTTOM_CENTER: Alignment = Alignment(HorizontalAlignment::Center, VerticalAlignment::Bottom);
    pub const BOTTOM_RIGHT: Alignment = Alignment(HorizontalAlignment::Right, VerticalAlignment::Bottom);


    pub fn get_left_space(&self, available_space: u32) -> u32 {
        match self.0 {
            HorizontalAlignment::Left => 0,
            HorizontalAlignment::Center => available_space / 2,
            HorizontalAlignment::Right => available_space,
        }
    }

    pub fn get_top_space(&self, available_space: u32) -> u32 {
        match self.1 {
            VerticalAlignment::Top => 0,
            VerticalAlignment::Center => available_space / 2,
            VerticalAlignment::Bottom => available_space,
        }
    }
}