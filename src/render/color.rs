#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const AMBER: Color = Color {
        r: 255,
        g: 200,
        b: 0,
    };
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };
    pub const DARK_GREY: Color = Color {
        r: 30,
        g: 30,
        b: 30,
    };
    pub const GREY: Color = Color {
        r: 180,
        g: 180,
        b: 180,
    };
    pub const LIGHT_PURPLE: Color = Color {
        r: 180,
        g: 180,
        b: 255,
    };
    pub const RED: Color = Color { r: 255, g: 0, b: 0 };
    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
    };

    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn to_u32(self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }
}
