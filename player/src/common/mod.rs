pub mod rendering;
pub use rendering::Framebuffer;
pub use rendering::draw_winning_screen;

pub mod constants;
pub use constants::*;

pub struct State {
    pub framebuffer: Framebuffer,
    pub input: Input,
}

impl State {
    pub fn new() -> State {
        let framebuffer = Framebuffer::new();

        State {
            framebuffer,
            input: Input::new(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Input {
    pub gamepad: Button::Ty,
    pub previous_gamepad: Button::Ty,
}

impl Input {
    pub fn new() -> Self {
        Input {
            gamepad: Button::Ty::empty(),
            previous_gamepad: Button::Ty::empty(),
        }
    }

    pub fn pressed_this_frame(&self, buttons: Button::Ty) -> bool {
        !self.previous_gamepad.contains(buttons) && self.gamepad.contains(buttons)
    }
}

// These values are deliberately picked to be the same as the ones in NES' input registers.
pub mod Button {
    bitflags! {
        pub flags Ty: u8 {
            const A          = 1 << 0,
            const B          = 1 << 1,
            const Select     = 1 << 2,
            const Start      = 1 << 3,
            const Up         = 1 << 4,
            const Down       = 1 << 5,
            const Left       = 1 << 6,
            const Right      = 1 << 7
        }
    }
}
