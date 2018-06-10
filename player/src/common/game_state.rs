
        use inner_common::*;

        impl GameState {
            pub const ENTITY_COUNT: usize = 256;
            pub const ENTITY_PIECE_COUNT: usize = 32;
            pub const GRID_DIMENSIONS : ( u8 , u8 ) = ( 4 , 6 ) ;

            pub fn new() -> GameState {
                let mut entities = [Component::Ty::empty(); GameState::ENTITY_COUNT];

                let mut positions = [[(0, 0); GameState::ENTITY_PIECE_COUNT]; GameState::ENTITY_COUNT];
                let mut appearances =
                    [[Appearance::default(); GameState::ENTITY_PIECE_COUNT]; GameState::ENTITY_COUNT];
                let mut sizes = [[(0, 0); GameState::ENTITY_PIECE_COUNT]; GameState::ENTITY_COUNT];

                let mut varieties = [Variety::default(); GameState::ENTITY_COUNT];

                let player_controlling_variety = Variety::default();

                    entities[0] = Component::Animate;
    positions[0] = [(0, 0), (3, 2), (11, 3), (11, 9), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[0] = [(25, 35), (8, 1), (1, 5), (1, 5), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[0] = [Appearance(11), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[0] = 0;
    entities[1] = Component::Animate;
    positions[1] = [(29, 0), (32, 2), (40, 3), (40, 9), (32, 14), (31, 9), (31, 3), (32, 8), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[1] = [(25, 35), (8, 1), (1, 5), (1, 5), (8, 1), (1, 5), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[1] = [Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[1] = 1;
    entities[2] = Component::Animate;
    positions[2] = [(58, 0), (61, 2), (0, 0), (69, 9), (61, 14), (0, 0), (60, 3), (61, 8), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[2] = [(25, 35), (8, 1), (0, 0), (1, 5), (8, 1), (0, 0), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[2] = [Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[2] = 2;
    entities[3] = Component::Animate;
    positions[3] = [(87, 0), (90, 2), (0, 0), (0, 0), (0, 0), (89, 9), (89, 3), (90, 8), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[3] = [(25, 35), (8, 1), (0, 0), (0, 0), (0, 0), (1, 5), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[3] = [Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[3] = 3;
    entities[4] = Component::Animate;
    positions[4] = [(0, 17), (3, 19), (0, 0), (0, 0), (3, 31), (2, 26), (2, 20), (3, 25), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[4] = [(25, 35), (8, 1), (0, 0), (0, 0), (8, 1), (1, 5), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[4] = [Appearance(11), Appearance(8), Appearance(8), Appearance(8), Appearance(8), Appearance(8), Appearance(8), Appearance(8), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[4] = 4;
    entities[5] = Component::Animate;
    positions[5] = [(29, 17), (32, 19), (0, 0), (0, 0), (0, 0), (31, 26), (31, 20), (32, 25), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[5] = [(25, 35), (8, 1), (0, 0), (0, 0), (0, 0), (1, 5), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[5] = [Appearance(13), Appearance(9), Appearance(9), Appearance(9), Appearance(9), Appearance(9), Appearance(9), Appearance(9), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[5] = 5;
    entities[6] = Component::Animate;
    positions[6] = [(58, 17), (61, 19), (69, 20), (69, 26), (0, 0), (60, 26), (60, 20), (61, 25), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[6] = [(25, 35), (8, 1), (1, 5), (1, 5), (0, 0), (1, 5), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[6] = [Appearance(10), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[6] = 6;
    entities[7] = Component::Animate;
    positions[7] = [(87, 17), (90, 19), (0, 0), (0, 0), (0, 0), (89, 26), (89, 20), (90, 25), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[7] = [(25, 35), (8, 1), (0, 0), (0, 0), (0, 0), (1, 5), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[7] = [Appearance(14), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[7] = 7;
    entities[8] = Component::Animate;
    positions[8] = [(0, 34), (3, 36), (11, 37), (11, 43), (0, 0), (2, 43), (2, 37), (3, 42), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[8] = [(25, 35), (8, 1), (1, 5), (1, 5), (0, 0), (1, 5), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[8] = [Appearance(8), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[8] = 8;
    entities[9] = Component::Animate;
    positions[9] = [(29, 34), (0, 0), (40, 37), (40, 43), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[9] = [(25, 35), (0, 0), (1, 5), (1, 5), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[9] = [Appearance(13), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[9] = 9;
    entities[10] = Component::Animate;
    positions[10] = [(58, 34), (61, 36), (0, 0), (0, 0), (0, 0), (60, 43), (60, 37), (61, 42), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[10] = [(25, 35), (8, 1), (0, 0), (0, 0), (0, 0), (1, 5), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[10] = [Appearance(13), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[10] = 10;
    entities[11] = Component::Animate;
    positions[11] = [(87, 34), (0, 0), (98, 37), (98, 43), (0, 0), (0, 0), (89, 37), (90, 42), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[11] = [(25, 35), (0, 0), (1, 5), (1, 5), (0, 0), (0, 0), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[11] = [Appearance(11), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[11] = 11;
    entities[12] = Component::Animate;
    positions[12] = [(0, 51), (0, 0), (0, 0), (11, 60), (3, 65), (2, 60), (2, 54), (3, 59), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[12] = [(25, 35), (0, 0), (0, 0), (1, 5), (8, 1), (1, 5), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[12] = [Appearance(9), Appearance(8), Appearance(8), Appearance(8), Appearance(8), Appearance(8), Appearance(8), Appearance(8), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[12] = 12;
    entities[13] = Component::Animate;
    positions[13] = [(29, 51), (32, 53), (0, 0), (0, 0), (32, 65), (31, 60), (31, 54), (32, 59), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[13] = [(25, 35), (8, 1), (0, 0), (0, 0), (8, 1), (1, 5), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[13] = [Appearance(9), Appearance(10), Appearance(10), Appearance(10), Appearance(10), Appearance(10), Appearance(10), Appearance(10), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[13] = 13;
    entities[14] = Component::Animate;
    positions[14] = [(58, 51), (61, 53), (69, 54), (69, 60), (61, 65), (60, 60), (60, 54), (61, 59), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[14] = [(25, 35), (8, 1), (1, 5), (1, 5), (8, 1), (1, 5), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[14] = [Appearance(15), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[14] = 14;
    entities[15] = Component::Animate;
    positions[15] = [(87, 51), (90, 53), (98, 54), (0, 0), (90, 65), (89, 60), (0, 0), (90, 59), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[15] = [(25, 35), (8, 1), (1, 5), (0, 0), (8, 1), (1, 5), (0, 0), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[15] = [Appearance(14), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[15] = 15;
    entities[16] = Component::Animate;
    positions[16] = [(0, 68), (0, 0), (11, 71), (11, 77), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[16] = [(25, 35), (0, 0), (1, 5), (1, 5), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[16] = [Appearance(11), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[16] = 16;
    entities[17] = Component::Animate;
    positions[17] = [(29, 68), (32, 70), (0, 0), (40, 77), (32, 82), (0, 0), (31, 71), (32, 76), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[17] = [(25, 35), (8, 1), (0, 0), (1, 5), (8, 1), (0, 0), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[17] = [Appearance(12), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[17] = 17;
    entities[18] = Component::Animate;
    positions[18] = [(58, 68), (61, 70), (0, 0), (69, 77), (61, 82), (0, 0), (60, 71), (61, 76), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[18] = [(25, 35), (8, 1), (0, 0), (1, 5), (8, 1), (0, 0), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[18] = [Appearance(11), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[18] = 18;
    entities[19] = Component::Animate;
    positions[19] = [(87, 68), (0, 0), (98, 71), (98, 77), (0, 0), (0, 0), (89, 71), (90, 76), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[19] = [(25, 35), (0, 0), (1, 5), (1, 5), (0, 0), (0, 0), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[19] = [Appearance(14), Appearance(10), Appearance(10), Appearance(10), Appearance(10), Appearance(10), Appearance(10), Appearance(10), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[19] = 19;
    entities[20] = Component::Animate;
    positions[20] = [(0, 85), (3, 87), (11, 88), (0, 0), (3, 99), (2, 94), (0, 0), (3, 93), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[20] = [(25, 35), (8, 1), (1, 5), (0, 0), (8, 1), (1, 5), (0, 0), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[20] = [Appearance(8), Appearance(12), Appearance(12), Appearance(12), Appearance(12), Appearance(12), Appearance(12), Appearance(12), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[20] = 20;
    entities[21] = Component::Animate;
    positions[21] = [(29, 85), (32, 87), (40, 88), (40, 94), (32, 99), (0, 0), (31, 88), (32, 93), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[21] = [(25, 35), (8, 1), (1, 5), (1, 5), (8, 1), (0, 0), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[21] = [Appearance(10), Appearance(8), Appearance(8), Appearance(8), Appearance(8), Appearance(8), Appearance(8), Appearance(8), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[21] = 21;
    entities[22] = Component::Animate;
    positions[22] = [(58, 85), (61, 87), (0, 0), (69, 94), (61, 99), (60, 94), (60, 88), (61, 93), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[22] = [(25, 35), (8, 1), (0, 0), (1, 5), (8, 1), (1, 5), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[22] = [Appearance(15), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[22] = 22;
    entities[23] = Component::Animate;
    positions[23] = [(87, 85), (0, 0), (98, 88), (98, 94), (0, 0), (0, 0), (89, 88), (90, 93), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[23] = [(25, 35), (0, 0), (1, 5), (1, 5), (0, 0), (0, 0), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[23] = [Appearance(12), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[23] = 23;
    entities[24] = Component::Player;
    positions[24] = [(116, 205), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[24] = [(25, 35), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[24] = [Appearance(3), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[24] = 0;


                GameState {
                    entities,
                    positions,
                    sizes,
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

        const SOLITAIRE_ENTITY_PIECE_COUNT : usize = 32 ; pub const BUTTON_COLUMN : u8
= 3 ; pub const FLOWER_FOUNDATION : u8 = 4 ; pub const START_OF_FOUNDATIONS :
u8 = 5 ; pub const END_OF_FOUNDATIONS : u8 = 7 ; pub const START_OF_TABLEAU :
u8 = 8 ; pub const CELLS_MAX_INDEX : u8 = 15 ; struct SevenSegmentAppearance {
positions : [ ( u8 , u8 ) ; 7 ] , sizes : [ ( u8 , u8 ) ; 7 ] , appearances :
[ Appearance ; 7 ] , } fn digit_to_segment_spec ( digit : u8 ) -> u8 {
match digit & 0b1111 {
0 => 0b0111111 , 1 => 0b0000110 , 2 => 0b1011011 , 3 => 0b1001111 , 4 =>
0b1100110 , 5 => 0b1101101 , 6 => 0b1111101 , 7 => 0b0000111 , 8 => 0b1111111
, 9 => 0b1101111 , 10 => 0b1110111 , 11 => 0b1111100 , 12 => 0b0111001 , 13 =>
0b1011110 , 14 => 0b1111001 , 15 => 0b1110001 , _ => 0 , } } fn
render_seven_segment (
digit : u8 , ( x , y ) : ( u8 , u8 ) , ( w , h ) : ( u8 , u8 ) , colour :
Colour , ) -> SevenSegmentAppearance {
let mut positions = [ Default :: default (  ) ; 7 ] ; let mut sizes = [
Default :: default (  ) ; 7 ] ; let appearances = [
colour | FilledRectangle ; 7 ] ; let spec : u8 = digit_to_segment_spec ( digit
) ; let short_width = 1 ; let long_width = w - ( short_width * 2 ) ; let
short_height = 1 ; let long_height = ( h / 2 ) - ( short_height * 2 ) ; if
spec & ( 1 << 0 ) == 1 << 0 {
positions [ 0 ] = ( x + short_width , y ) ; sizes [ 0 ] = (
long_width , short_height ) ; } if spec & ( 1 << 1 ) == 1 << 1 {
positions [ 1 ] = ( x + long_width + short_width , y + short_height ) ; sizes
[ 1 ] = ( short_width , long_height ) ; } if spec & ( 1 << 2 ) == 1 << 2 {
positions [ 2 ] = (
x + long_width + short_width , y + ( short_height * 2 ) + long_height , ) ;
sizes [ 2 ] = ( short_width , long_height ) ; } if spec & ( 1 << 3 ) == 1 << 3
{
positions [ 3 ] = (
x + short_width , y + ( short_height * 2 ) + ( long_height * 2 ) ) ; sizes [ 3
] = ( long_width , short_height ) ; } if spec & ( 1 << 4 ) == 1 << 4 {
positions [ 4 ] = ( x , y + ( short_height * 2 ) + long_height ) ; sizes [ 4 ]
= ( short_width , long_height ) ; } if spec & ( 1 << 5 ) == 1 << 5 {
positions [ 5 ] = ( x , y + short_height ) ; sizes [ 5 ] = (
short_width , long_height ) ; } if spec & ( 1 << 6 ) == 1 << 6 {
positions [ 6 ] = ( x + short_width , y + short_height + long_height ) ; sizes
[ 6 ] = ( long_width , short_height ) ; } SevenSegmentAppearance {
positions , sizes , appearances , } } struct CardAppearance {
positions : [ ( u8 , u8 ) ; SOLITAIRE_ENTITY_PIECE_COUNT ] , sizes : [
( u8 , u8 ) ; SOLITAIRE_ENTITY_PIECE_COUNT ] , appearances : [
Appearance ; SOLITAIRE_ENTITY_PIECE_COUNT ] , } fn get_card_appearance (
variety : u8 , ( x , y ) : ( u8 , u8 ) , ) -> CardAppearance {
let variety = variety as u32 ; let mut positions : [
( u8 , u8 ) ; SOLITAIRE_ENTITY_PIECE_COUNT ] = Default :: default (  ) ; let
mut sizes : [ ( u8 , u8 ) ; SOLITAIRE_ENTITY_PIECE_COUNT ] = Default ::
default (  ) ; let mut appearances : [
Appearance ; SOLITAIRE_ENTITY_PIECE_COUNT ] = Default :: default (  ) ; const
SPACING : u8 = 2 ; let mut i = 0 ; positions [ i ] = ( x , y ) ; sizes [ i ] =
( card :: WIDTH , card :: HEIGHT ) ; appearances [ i ] = Colour :: from (
variety % COLOUR_COUNT as u32 ) | FilledRectangle ; i += 1 ; let digit = 4 ;
let ssa : SevenSegmentAppearance = render_seven_segment (
digit , ( x + SPACING , y + SPACING ) , (
( card :: WIDTH - ( SPACING * 2 ) ) / 2 , ( card :: HEIGHT - ( SPACING * 2 ) )
/ 2 , ) , Colour :: from ( ( variety >> 3 ) % COLOUR_COUNT as u32 ) , ) ; for
ssa_index in 0 .. 7 {
positions [ i ] = ssa . positions [ ssa_index ] ; sizes [ i ] = ssa . sizes [
ssa_index ] ; appearances [ i ] = ssa . appearances [ ssa_index ] ; i += 1 ; }
CardAppearance { positions , sizes , appearances , } } pub const
TOP_ROW_ENTITY_Y : u8 = 0 ; pub const SECOND_ROW_ENTITY_Y : u8 = 24 ; pub
const CARD_HORIZONTAL_OFFSET : u8 = 16 ; pub const CARD_VERTICAL_OFFSET : u8 =
8 ; pub const FLOWER_CARD_X : u8 = 56 ; fn get_card_pos (
( x , y ) : ( u8 , u8 ) ) -> ( u8 , u8 ) {
let ( mut pos_x , mut pos_y ) = if x > END_OF_FOUNDATIONS {
( x - START_OF_TABLEAU , SECOND_ROW_ENTITY_Y ) } else {
( x , TOP_ROW_ENTITY_Y ) } ; pos_x = if pos_y == 0 && pos_x ==
FLOWER_FOUNDATION { FLOWER_CARD_X } else { pos_x * CARD_HORIZONTAL_OFFSET } ;
pos_y += y * CARD_VERTICAL_OFFSET ; ( pos_x , pos_y ) } fn
card_entity_base_pos_to_grid_pos ( ( x , y ) : ( u8 , u8 ) ) -> ( u8 , u8 ) {
let mut grid_x = if y == 0 && x == FLOWER_CARD_X { FLOWER_FOUNDATION } else {
x / CARD_HORIZONTAL_OFFSET } ; if y >= SECOND_ROW_ENTITY_Y {
grid_x += START_OF_TABLEAU ; } let mut grid_y = if grid_x > END_OF_FOUNDATIONS
{ y - SECOND_ROW_ENTITY_Y } else { y } ; grid_y /= CARD_VERTICAL_OFFSET ; (
grid_x , grid_y ) }use std :: cmp :: { max , min } ; use common :: * ; macro_rules !
last_unchecked { ( $ vec : expr ) => { $ vec [ $ vec . len (  ) - 1 ] } ; }
pub const MOVE_TIMER_MAX : u8 = 3 ; pub const MAX_SUIT_NUM : u8 = 9 ; pub
const FIRST_GREEN_CARD : u8 = 10 ; pub const FIRST_BLACK_CARD : u8 = 20 ; pub
const FLOWER_CARD : u8 = 30 ; pub const CARD_BACK : u8 = 31 ; pub const CURSOR
: u8 = 32 ; pub const CURSOR_GHOST : u8 = 33 ; pub const BUTTON_COLUMN_VARIETY
: u8 = 34 ; pub fn update ( state : & mut CustomState , input : Input ) {
if state . movetimer > 0 { state . movetimer -= 1 ; } if state . movetimer ==
0 {
if automove ( state ) { state . movetimer = MOVE_TIMER_MAX ; } else {
if input . pressed_this_frame ( Button :: Left ) {
state . selectpos = if state . selectpos == 0 { START_OF_TABLEAU - 1 } else if
state . selectpos == START_OF_TABLEAU { CELLS_MAX_INDEX } else {
state . selectpos - 1 } ; state . selectdepth = if state . selectdrop { 0 }
else {
let len = state . cells [ state . selectpos as usize ] . len (  ) as u8 ; min
( max ( 0 , state . selectdepth ) , len - 1 ) } ; } else if input .
pressed_this_frame ( Button :: Right ) {
state . selectpos = if state . selectpos == START_OF_TABLEAU - 1 { 0 } else if
state . selectpos >= CELLS_MAX_INDEX { START_OF_TABLEAU } else {
state . selectpos + 1 } ; state . selectdepth = if state . selectdrop { 0 }
else {
let len = state . cells [ state . selectpos as usize ] . len (  ) as u8 ; min
( max ( 0 , state . selectdepth ) , len - 1 ) } ; } else if input .
pressed_this_frame ( Button :: Up ) {
let changepos = if state . selectpos == BUTTON_COLUMN {
state . selectdepth >= 2 } else {
let len = state . cells [ state . selectpos as usize ] . len (  ) ; len == 0
|| state . selectdepth >= len as u8 - 1 || state . selectdrop } ; if changepos
{
state . selectpos = if state . selectpos > END_OF_FOUNDATIONS {
state . selectpos - START_OF_TABLEAU } else {
state . selectpos + START_OF_TABLEAU } ; state . selectdepth = 0 ; } else {
state . selectdepth += 1 ; } } else if input . pressed_this_frame (
Button :: Down ) {
if state . selectdepth == 0 {
state . selectpos = if state . selectpos > END_OF_FOUNDATIONS {
state . selectpos - START_OF_TABLEAU } else {
state . selectpos + START_OF_TABLEAU } ; let len = state . cells [
state . selectpos as usize ] . len (  ) ; state . selectdepth = if len > 0 &&
! state . selectdrop { len as u8 - 1 } else if state . selectpos ==
BUTTON_COLUMN { 2 } else { 0 } ; } else {
state . selectdepth = state . selectdepth - 1 ; } } else if input .
pressed_this_frame ( Button :: A ) {
if state . selectpos == BUTTON_COLUMN {
if canmovedragons ( state , state . selectdepth ) {
movedragons ( state ) ; state . selectdrop = false ; state . movetimer =
MOVE_TIMER_MAX ; } } else {
if state . selectdrop {
let grabpos : u8 = state . grabpos ; let grabdepth : u8 = state . grabdepth ;
let selectpos : u8 = state . selectpos ; if candrop (
& state . cells , grabpos , grabdepth , selectpos , ) {
movecards ( state , grabpos , grabdepth , selectpos ) ; state . selectdrop =
false ; state . movetimer = MOVE_TIMER_MAX ; } } else if cangrab (
& state . cells , state . selectpos , state . selectdepth ) {
state . grabpos = state . selectpos ; state . grabdepth = state . selectdepth
; state . selectdrop = true ; } } } else if input . pressed_this_frame (
Button :: B ) { state . selectdrop = false ; } } } if haswon ( state ) {
if state . win_done {
if input . pressed_this_frame ( Button :: Start ) {
let wins = state . wins ; * state = GameState :: new (  ) . get_custom_state (
 ) ; state . wins = wins ; } } else {
state . wins += 1 ; state . win_done = true ; } } } fn getselection (
cells : & Cells , pos : u8 , depth : u8 ) -> Vec < u8 > {
let pos = pos as usize ; let depth = depth as usize ; let mut output = Vec ::
with_capacity ( depth ) ; for i in 1 ..= depth + 1 {
let index = cells [ pos ] . len (  ) - ( depth + 1 ) + i - 1 ; output . push (
cells [ pos ] [ index ] ) ; } return output ; } fn cangrab (
cells : & Cells , pos : u8 , depth : u8 ) -> bool {
let selection = getselection ( cells , pos , depth ) ; if selection . len (  )
== 0 || ( pos >= FLOWER_FOUNDATION && pos < START_OF_TABLEAU ) {
return false ; } let mut lastsuit = 255 ; let mut lastnum = 255 ; let mut
first = true ; for & card in selection . iter (  ) {
if card == CARD_BACK { return false ; } let suit = getsuit ( card ) ; let num
= getcardnum ( card ) ; if ! first {
if suit == lastsuit || num == 0 || num != lastnum - 1 { return false ; } }
lastsuit = suit ; lastnum = num ; first = false ; } return true ; } fn candrop
( cells : & Cells , grabpos : u8 , grabdepth : u8 , droppos : u8 ) -> bool {
let grabpos = grabpos as usize ; let grabdepth = grabdepth as usize ; let
grabcard = {
let len = cells [ grabpos ] . len (  ) ; if len < grabdepth { return false ; }
cells [ grabpos ] [ len - 1 - grabdepth ] } ; if droppos < BUTTON_COLUMN {
return cells [ droppos as usize ] . len (  ) == 0 && grabdepth == 0 ; } else
if droppos >= BUTTON_COLUMN && droppos <= FLOWER_FOUNDATION { return false ; }
else if droppos >= START_OF_FOUNDATIONS && droppos < START_OF_TABLEAU {
let droppos = droppos as usize ; if grabdepth == 0 {
if cells [ droppos ] . len (  ) == 0 {
if getcardnum ( grabcard ) == 1 { return true ; } } else {
let dropcard = last_unchecked ! ( cells [ droppos ] ) ; if getsuit ( grabcard
) == getsuit ( dropcard ) && getcardnum ( grabcard ) != 0 && getcardnum (
grabcard ) == getcardnum ( dropcard ) + 1 { return true ; } } } return false ;
} else {
let droppos = droppos as usize ; if cells [ droppos ] . len (  ) == 0 {
return true ; } else {
let dropcard = last_unchecked ! ( cells [ droppos ] ) ; if getsuit ( grabcard
) != getsuit ( dropcard ) && getcardnum ( grabcard ) != 0 && getcardnum (
grabcard ) == getcardnum ( dropcard ) - 1 { return true ; } } return false ; }
} fn getsuit ( card : u8 ) -> u8 {
if card >= FLOWER_CARD { 3 } else if card >= FIRST_BLACK_CARD { 2 } else if
card >= FIRST_GREEN_CARD { 1 } else { 0 } } fn getcardnum ( card : u8 ) -> u8
{ card - ( getsuit ( card ) * 10 ) } fn movecards (
state : & mut CustomState , grabpos : u8 , grabdepth : u8 , droppos : u8 ) {
let grabpos = grabpos as usize ; let grabdepth = grabdepth as usize ; let
droppos = droppos as usize ; if droppos <= END_OF_FOUNDATIONS as usize {
if let Some ( last ) = state . cells [ grabpos ] . pop (  ) {
if state . cells [ droppos ] . len (  ) > 0 {
state . cells [ droppos ] [ 0 ] = last ; } else {
state . cells [ droppos ] . push ( last ) ; } } } else {
let len = state . cells [ grabpos ] . len (  ) ; let temp : Vec < _ > = state
. cells [ grabpos ] . drain ( len - 1 - grabdepth .. ) . collect (  ) ; state
. cells [ droppos ] . extend ( temp . into_iter (  ) ) ; } } fn canmovedragons
( state : & CustomState , suit : u8 ) -> bool {
let mut count = 0 ; for i in 0 ..= CELLS_MAX_INDEX {
let i = i as usize ; if state . cells [ i ] . len (  ) > 0 && last_unchecked !
( state . cells [ i ] ) == suit * 10 { count += 1 ; } } if count < 4 {
return false ; } for i in 0 .. BUTTON_COLUMN {
let i = i as usize ; if state . cells [ i ] . len (  ) == 0 || last_unchecked
! ( state . cells [ i ] ) == suit * 10 { return true ; } } return false ; } fn
movedragons ( state : & mut CustomState ) {
let suit = state . selectdepth ; let mut moveto = None ; for i in 0 ..
BUTTON_COLUMN {
let i = i as usize ; if state . cells [ i ] . len (  ) != 0 && last_unchecked
! ( state . cells [ i ] ) == suit * 10 && moveto . is_none (  ) {
moveto = Some ( i ) ; } } if moveto . is_none (  ) {
for i in 0 .. BUTTON_COLUMN {
let i = i as usize ; if state . cells [ i ] . len (  ) == 0 {
moveto = Some ( i ) ; break ; } } } for i in 0 ..= CELLS_MAX_INDEX {
let i = i as usize ; if state . cells [ i ] . len (  ) != 0 && last_unchecked
! ( state . cells [ i ] ) == suit * 10 { state . cells [ i ] . pop (  ) ; } }
if let Some ( moveto ) = moveto {
let moveto = moveto as usize ; state . cells [ moveto ] . push ( CARD_BACK ) ;
} } fn haswon ( state : & CustomState ) -> bool {
for i in START_OF_TABLEAU ..= CELLS_MAX_INDEX {
let i = i as usize ; if state . cells [ i ] . len (  ) > 0 { return false ; }
} return true ; } fn automove ( state : & mut CustomState ) -> bool {
let min_free_card_num = {
let mut min_foundation_card_num = None ; for i in START_OF_FOUNDATIONS ..
START_OF_TABLEAU {
let i = i as usize ; let val = if state . cells [ i ] . len (  ) > 0 {
let card = last_unchecked ! ( state . cells [ i ] ) ; getcardnum ( card ) }
else { 0 } ; if min_foundation_card_num . map ( | v | val < v ) . unwrap_or (
true ) { min_foundation_card_num = Some ( val ) ; } } min_foundation_card_num
. unwrap_or ( 255 ) . wrapping_add ( 1 ) } ; for i in 0 ..= CELLS_MAX_INDEX {
if ( i < BUTTON_COLUMN || i >= START_OF_TABLEAU ) && state . cells [
i as usize ] . len (  ) > 0 {
let card = last_unchecked ! ( state . cells [ i as usize ] ) ; if card ==
FLOWER_CARD { movecards ( state , i , 0 , FLOWER_FOUNDATION ) ; return true ;
} else if getcardnum ( card ) == min_free_card_num && card != CARD_BACK {
let suit = getsuit ( card ) ; for i2 in START_OF_FOUNDATIONS ..
START_OF_TABLEAU {
if state . cells [ i2 as usize ] . len (  ) > 0 {
let card2 = last_unchecked ! ( state . cells [ i2 as usize ] ) ; if getsuit (
card2 ) == suit { movecards ( state , i , 0 , i2 ) ; return true ; } } } for
i2 in START_OF_FOUNDATIONS .. START_OF_TABLEAU {
if state . cells [ i2 as usize ] . len (  ) == 0 {
movecards ( state , i , 0 , i2 ) ; return true ; } } } } } return false ; }
pub type Cells = [ Vec < u8 > ; CELLS_MAX_INDEX as usize + 1 ] ; pub struct
CustomState {
pub cells : Cells , pub wins : u8 , pub win_done : bool , pub selectdrop :
bool , pub selectpos : u8 , pub selectdepth : u8 , pub grabpos : u8 , pub
grabdepth : u8 , pub movetimer : u8 , } const
FIRST_UNUSED_FOR_EXTRA_DATA_INDEX : usize = 8 ; impl GameState {
pub fn get_custom_state ( & self ) -> CustomState {
let mut grid_positions : Vec < ( ( u8 , u8 ) , u8 ) > = Vec :: new (  ) ; for
i in FIRST_UNUSED_FOR_EXTRA_DATA_INDEX .. GameState :: ENTITY_COUNT {
if self . entities [ i ] . is_empty (  ) { continue ; } match self . varieties
[ i ] {
value @ 0 ... FLOWER_CARD => {
let grid_position = card_entity_pos_to_grid_pos ( self . positions [ i ] ) ;
grid_positions . push ( ( grid_position , value ) ) ; } , CURSOR => {  } ,
CURSOR_GHOST => {  } , BUTTON_COLUMN_VARIETY => {  } , _ => {  } , } }
grid_positions . sort_by_key (
| & ( position , _ ) : & ( ( u8 , u8 ) , u8 ) | position ) ; let mut cells :
Cells = Default :: default (  ) ; for & ( ( x , y ) , value ) in
grid_positions . iter (  ) {
let x = x as usize ; cells [ x ] . push ( value ) ; } CustomState {
cells , wins : self . varieties [ 0 ] , win_done : self . varieties [ 1 ] != 0
, selectdrop : self . varieties [ 2 ] != 0 , selectpos : self . varieties [ 3
] , selectdepth : self . varieties [ 4 ] , grabpos : self . varieties [ 5 ] ,
grabdepth : self . varieties [ 6 ] , movetimer : self . varieties [ 7 ] , } }
pub fn set_custom_state ( & mut self , custom_state : CustomState ) {
let mut id = FIRST_UNUSED_FOR_EXTRA_DATA_INDEX ; for x in 0 .. CELLS_MAX_INDEX
as usize + 1 {
let column = & custom_state . cells [ x ] ; for y in 0 .. column . len (  ) {
let variety = column [ y ] ; let full_entity = get_card_full_entity (
variety , get_card_pos ( ( x as u8 , y as u8 ) ) ) ; self . set_full_entity (
id , full_entity ) ; id += 1 ; } } fn get_button_full_entity (
state : & CustomState , suit : u8 , pos : Position ) -> FullEntity {
let colour = if canmovedragons ( state , suit ) { Yellow } else { White } ;
let symbol_colour = match suit {
0 => Red , 1 => Green , 2 => Black , _ => Grey , } ; const BUTTON_WIDTH : u8 =
8 ; const BUTTON_HEIGHT : u8 = 8 ; const BUTTON_MARGIN : u8 = 1 ; let
seven_segment_appearance = render_seven_segment (
suit + 10 , ( pos . 0 + BUTTON_MARGIN , pos . 1 + BUTTON_MARGIN ) , (
BUTTON_WIDTH - 2 * BUTTON_MARGIN , BUTTON_HEIGHT - 2 * BUTTON_MARGIN ) ,
symbol_colour ) ; let mut position : [
Position ; GameState :: ENTITY_PIECE_COUNT ] = Default :: default (  ) ;
position [ 0 ] = pos ; {
let ( left , right ) = position . split_at_mut ( 1 ) ; let ( middle , right )
= right . split_at_mut ( seven_segment_appearance . positions . len (  ) ) ;
middle . copy_from_slice ( & seven_segment_appearance . positions [ .. ] ) ; }
let mut appearance : [ Appearance ; GameState :: ENTITY_PIECE_COUNT ] =
Default :: default (  ) ; appearance [ 0 ] = colour | FilledRectangle ; {
let ( left , right ) = appearance . split_at_mut ( 1 ) ; let ( middle , right
) = right . split_at_mut ( seven_segment_appearance . appearances . len (  ) )
; middle . copy_from_slice ( & seven_segment_appearance . appearances [ .. ] )
; } let mut size : [ Position ; GameState :: ENTITY_PIECE_COUNT ] = Default ::
default (  ) ; size [ 0 ] = ( BUTTON_WIDTH , BUTTON_HEIGHT ) ; {
let ( left , right ) = size . split_at_mut ( 1 ) ; let ( middle , right ) =
right . split_at_mut ( seven_segment_appearance . sizes . len (  ) ) ; middle
. copy_from_slice ( & seven_segment_appearance . sizes [ .. ] ) ; } FullEntity
{
entity : Component :: Animate , position , appearance , size , variety :
BUTTON_COLUMN_VARIETY , } } {
let suit = 0 ; let pos = ( 48 , 16 ) ; self . set_full_entity (
id , get_button_full_entity ( & custom_state , suit , pos ) ) ; id += 1 ; } {
let suit = 1 ; let pos = ( 48 , 8 ) ; self . set_full_entity (
id , get_button_full_entity ( & custom_state , suit , pos ) ) ; id += 1 ; } {
let suit = 2 ; let pos = ( 48 , 0 ) ; self . set_full_entity (
id , get_button_full_entity ( & custom_state , suit , pos ) ) ; id += 1 ; }
self . varieties [ 0 ] = custom_state . wins ; self . varieties [ 1 ] = if
custom_state . win_done { 1 } else { 0 } ; self . varieties [ 2 ] = if
custom_state . selectdrop { 1 } else { 0 } ; self . varieties [ 3 ] =
custom_state . selectpos ; self . varieties [ 4 ] = custom_state . selectdepth
; self . varieties [ 5 ] = custom_state . grabpos ; self . varieties [ 6 ] =
custom_state . grabdepth ; self . varieties [ 7 ] = custom_state . movetimer ;
} } fn get_card_full_entity ( variety : Variety , pos : ( u8 , u8 ) , ) ->
FullEntity {
let card_appearance = get_card_appearance ( variety , pos ) ; FullEntity {
entity : Component :: Animate , position : card_appearance . positions ,
appearance : card_appearance . appearances , size : card_appearance . sizes ,
variety , } } fn card_entity_pos_to_grid_pos (
pos : [ Position ; GameState :: ENTITY_PIECE_COUNT ] ) -> ( u8 , u8 ) {
card_entity_base_pos_to_grid_pos ( pos [ 0 ] ) }
        