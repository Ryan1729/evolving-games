pub use common::project_common::*;

pub struct GameState {
    pub entities: [Component::Ty; GameState::ENTITY_COUNT],

    pub positions: [[Position; GameState::ENTITY_PIECE_COUNT]; GameState::ENTITY_COUNT],
    pub appearances: [[Appearance; GameState::ENTITY_PIECE_COUNT]; GameState::ENTITY_COUNT],
    pub sizes: [[Position; GameState::ENTITY_PIECE_COUNT]; GameState::ENTITY_COUNT],

    pub varieties: [Variety; GameState::ENTITY_COUNT],

    pub player_controlling_variety: Variety,
}

pub struct FullEntity {
    pub entity: Component::Ty,

    pub position: [Position; GameState::ENTITY_PIECE_COUNT],
    pub appearance: [Appearance; GameState::ENTITY_PIECE_COUNT],
    pub size: [Position; GameState::ENTITY_PIECE_COUNT],

    pub variety: Variety,
}

impl GameState {
    pub fn set_full_entity(&mut self, id: usize, full_entity: FullEntity) {
        self.entities[id] = full_entity.entity;

        self.positions[id] = full_entity.position;
        self.appearances[id] = full_entity.appearance;
        self.sizes[id] = full_entity.size;

        self.varieties[id] = full_entity.variety;
    }
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
