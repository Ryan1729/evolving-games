//in pixels
pub const SCREEN_WIDTH: usize = 256;
pub const SCREEN_HEIGHT: usize = 240;
pub const SCREEN_LENGTH: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

// reportedly colourblind friendly colours
// https://twitter.com/ea_accessible/status/968595073184092160
pub mod colours {
    pub const BLUE: u32 = 0xFFE15233;
    pub const GREEN: u32 = 0xFF6EB030;
    pub const RED: u32 = 0xFF4949DE;
    pub const YELLOW: u32 = 0xFF37B9FF;
    pub const PURPLE: u32 = 0xFF543353;
    pub const GREY: u32 = 0xFF8B7D5A;
    pub const GRAY: u32 = GREY;
    pub const WHITE: u32 = 0xFFEEEEEE;
    pub const BLACK: u32 = 0xFF222222;
}
pub use colours::*;

#[derive(Clone, Copy)]
pub enum Colour {
    Blue,
    Green,
    Red,
    Yellow,
    Purple,
    Grey,
    White,
    Black,
}
pub use Colour::*;

impl From<Colour> for u32 {
    fn from(c: Colour) -> Self {
        match c {
            Blue => BLUE,
            Green => GREEN,
            Red => RED,
            Yellow => YELLOW,
            Purple => PURPLE,
            Grey => GREY,
            White => WHITE,
            Black => BLACK,
        }
    }
}

impl From<u32> for Colour {
    fn from(n: u32) -> Self {
        match n {
            BLUE => Blue,
            GREEN => Green,
            RED => Red,
            YELLOW => Yellow,
            PURPLE => Purple,
            GREY => Grey,
            WHITE => White,
            BLACK => Black,
            _ => Grey,
        }
    }
}

impl From<Colour> for usize {
    fn from(c: Colour) -> Self {
        match c {
            Blue => 0,
            Green => 1,
            Red => 2,
            Yellow => 3,
            Purple => 4,
            Grey => 5,
            White => 6,
            Black => 7,
        }
    }
}

impl From<usize> for Colour {
    fn from(n: usize) -> Self {
        match n {
            0 => Blue,
            1 => Green,
            2 => Red,
            3 => Yellow,
            4 => Purple,
            5 => Grey,
            6 => White,
            7 => Black,
            _ => Grey,
        }
    }
}
