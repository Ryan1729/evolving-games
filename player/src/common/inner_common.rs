pub use common::project_common::*;

pub struct GameState {
    pub entities: [Component::Ty; GameState::ENTITY_COUNT],

    pub positions: [[Position; GameState::ENTITY_PIECE_COUNT]; GameState::ENTITY_COUNT],
    pub appearances: [[Appearance; GameState::ENTITY_PIECE_COUNT]; GameState::ENTITY_COUNT],
    pub sizes: [[Position; GameState::ENTITY_PIECE_COUNT]; GameState::ENTITY_COUNT],

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
