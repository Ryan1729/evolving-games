use inner_common::*;

        impl GameState {
            pub const ENTITY_COUNT: usize = 256;
            pub const GRID_DIMENSIONS: (u8, u8) = (7, 4);

            pub fn new() -> GameState {
                let mut entities = [Component::Ty::empty(); GameState::ENTITY_COUNT];

                let mut positions = [(0, 0); GameState::ENTITY_COUNT];
                let mut appearances = [Appearance::default(); GameState::ENTITY_COUNT];
                let mut varieties = [Variety::default(); GameState::ENTITY_COUNT];

                let player_controlling_variety = Variety::default();

                    entities[0] = Component::Player;
    positions[0] = (2, 0);
    appearances[0] = Appearance(0);
    varieties[0] = 0;
    entities[1] = Component::Player;
    positions[1] = (24, 0);
    appearances[1] = Appearance(1);
    varieties[1] = 1;
    entities[2] = Component::Player;
    positions[2] = (46, 0);
    appearances[2] = Appearance(2);
    varieties[2] = 2;
    entities[3] = Component::Player;
    positions[3] = (68, 0);
    appearances[3] = Appearance(3);
    varieties[3] = 3;
    entities[4] = Component::Player;
    positions[4] = (90, 0);
    appearances[4] = Appearance(4);
    varieties[4] = 4;
    entities[5] = Component::Player;
    positions[5] = (112, 0);
    appearances[5] = Appearance(5);
    varieties[5] = 5;
    entities[6] = Component::Player;
    positions[6] = (134, 0);
    appearances[6] = Appearance(6);
    varieties[6] = 6;
    entities[7] = Component::Player;
    positions[7] = (2, 14);
    appearances[7] = Appearance(7);
    varieties[7] = 7;
    entities[8] = Component::Player;
    positions[8] = (24, 14);
    appearances[8] = Appearance(8);
    varieties[8] = 8;
    entities[9] = Component::Player;
    positions[9] = (46, 14);
    appearances[9] = Appearance(9);
    varieties[9] = 9;
    entities[10] = Component::Player;
    positions[10] = (68, 14);
    appearances[10] = Appearance(10);
    varieties[10] = 10;
    entities[11] = Component::Player;
    positions[11] = (90, 14);
    appearances[11] = Appearance(11);
    varieties[11] = 11;
    entities[12] = Component::Player;
    positions[12] = (112, 14);
    appearances[12] = Appearance(12);
    varieties[12] = 12;
    entities[13] = Component::Player;
    positions[13] = (134, 14);
    appearances[13] = Appearance(13);
    varieties[13] = 13;
    entities[14] = Component::Player;
    positions[14] = (2, 28);
    appearances[14] = Appearance(14);
    varieties[14] = 14;
    entities[15] = Component::Player;
    positions[15] = (24, 28);
    appearances[15] = Appearance(15);
    varieties[15] = 15;
    entities[16] = Component::Player;
    positions[16] = (46, 28);
    appearances[16] = Appearance(16);
    varieties[16] = 16;
    entities[17] = Component::Player;
    positions[17] = (68, 28);
    appearances[17] = Appearance(17);
    varieties[17] = 17;
    entities[18] = Component::Player;
    positions[18] = (90, 28);
    appearances[18] = Appearance(18);
    varieties[18] = 18;
    entities[19] = Component::Player;
    positions[19] = (112, 28);
    appearances[19] = Appearance(19);
    varieties[19] = 19;
    entities[20] = Component::Player;
    positions[20] = (134, 28);
    appearances[20] = Appearance(20);
    varieties[20] = 20;
    entities[21] = Component::Player;
    positions[21] = (2, 42);
    appearances[21] = Appearance(21);
    varieties[21] = 21;
    entities[22] = Component::Player;
    positions[22] = (24, 42);
    appearances[22] = Appearance(22);
    varieties[22] = 22;
    entities[23] = Component::Player;
    positions[23] = (46, 42);
    appearances[23] = Appearance(23);
    varieties[23] = 23;
    entities[24] = Component::Player;
    positions[24] = (68, 42);
    appearances[24] = Appearance(24);
    varieties[24] = 24;
    entities[25] = Component::Player;
    positions[25] = (90, 42);
    appearances[25] = Appearance(25);
    varieties[25] = 25;
    entities[26] = Component::Player;
    positions[26] = (112, 42);
    appearances[26] = Appearance(26);
    varieties[26] = 26;


                GameState {
                    entities,
                    positions,
                    appearances,
                    varieties,
                    player_controlling_variety,
                }
            }

            pub fn get_free_id(&self) -> Option<usize> {
                for (i, e) in self.entities.iter().enumerate() {
                    if e.is_empty() {
                        return Some(i);
                    }
                }

                None
            }
        }
