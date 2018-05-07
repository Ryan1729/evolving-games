use inner_common::*;

    impl GameState {
        pub const ENTITY_COUNT: usize = 256;

        pub fn new() -> GameState {
            let mut entities = [Component::Ty::empty(); GameState::ENTITY_COUNT];

            let mut positions = [(0, 0); GameState::ENTITY_COUNT];
            let mut appearances = [Appearance::default(); GameState::ENTITY_COUNT];
            let mut varieties = [Variety::default(); GameState::ENTITY_COUNT];

            let player_controlling_variety = Variety::default();

            GameState {
                entities,
                positions,
                appearances,
                varieties,
                player_controlling_variety,
            }
        }

        pub fn mark_won(&mut self) {
            self.positions[0] = (1,1);
        }

        pub fn has_won(&self) -> bool {
            self.positions[0].0 == 1
        }
    }
