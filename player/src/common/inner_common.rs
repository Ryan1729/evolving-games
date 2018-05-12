pub use common::project_common::*;

pub struct GameState {
    pub entities: [Component::Ty; GameState::ENTITY_COUNT],

    pub positions: [Position; GameState::ENTITY_COUNT],
    pub appearances: [Appearance; GameState::ENTITY_COUNT],
    pub varieties: [Variety; GameState::ENTITY_COUNT],

    pub player_controlling_variety: Variety,
}

pub mod Component {
    bitflags! {
        pub flags Ty: u16 {
            const Animate          = 1 << 0,
            const PlayerControlled = 1 << 1,
            const Player = Animate.bits
             | PlayerControlled.bits,
        }
    }
}

pub type Position = (u8, u8);

#[derive(Clone, Copy, Debug)]
pub struct Appearance(pub u8);

impl Default for Appearance {
    fn default() -> Self {
        Appearance(EMPTY_APPEARANCE)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Shape {
    Rectangle,
    FilledRectangle,
    Circle,
    FilledCircle,
}
pub use Shape::*;
