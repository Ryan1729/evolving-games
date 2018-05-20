
        use inner_common::*;

        impl GameState {
            pub const ENTITY_COUNT: usize = 256;
            pub const ENTITY_PIECE_COUNT: usize = 32;
            pub const GRID_DIMENSIONS : ( u8 , u8 ) = ( 7 , 4 ) ;

            pub fn new() -> GameState {
                let mut entities = [Component::Ty::empty(); GameState::ENTITY_COUNT];

                let mut positions = [[(0, 0); GameState::ENTITY_PIECE_COUNT]; GameState::ENTITY_COUNT];
                let mut appearances =
                    [[Appearance::default(); GameState::ENTITY_PIECE_COUNT]; GameState::ENTITY_COUNT];
                let mut sizes = [[(0, 0); GameState::ENTITY_PIECE_COUNT]; GameState::ENTITY_COUNT];

                let mut varieties = [Variety::default(); GameState::ENTITY_COUNT];

                let player_controlling_variety = Variety::default();

                    entities[0] = Component::Animate;
    positions[0] = [(0, 0), (3, 2), (11, 3), (11, 9), (3, 14), (0, 0), (0, 0), (3, 8), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[0] = [(25, 35), (8, 1), (1, 5), (1, 5), (8, 1), (0, 0), (0, 0), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[0] = [Appearance(14), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[0] = 0;
    entities[1] = Component::Animate;
    positions[1] = [(29, 0), (0, 0), (40, 3), (40, 9), (0, 0), (0, 0), (31, 3), (32, 8), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[1] = [(25, 35), (0, 0), (1, 5), (1, 5), (0, 0), (0, 0), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[1] = [Appearance(13), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[1] = 1;
    entities[2] = Component::Animate;
    positions[2] = [(58, 0), (0, 0), (0, 0), (69, 9), (61, 14), (60, 9), (60, 3), (61, 8), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[2] = [(25, 35), (0, 0), (0, 0), (1, 5), (8, 1), (1, 5), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[2] = [Appearance(10), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[2] = 2;
    entities[3] = Component::Animate;
    positions[3] = [(87, 0), (0, 0), (0, 0), (98, 9), (90, 14), (89, 9), (89, 3), (90, 8), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[3] = [(25, 35), (0, 0), (0, 0), (1, 5), (8, 1), (1, 5), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[3] = [Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[3] = 3;
    entities[4] = Component::Animate;
    positions[4] = [(116, 0), (0, 0), (127, 3), (127, 9), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[4] = [(25, 35), (0, 0), (1, 5), (1, 5), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[4] = [Appearance(9), Appearance(12), Appearance(12), Appearance(12), Appearance(12), Appearance(12), Appearance(12), Appearance(12), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[4] = 4;
    entities[5] = Component::Animate;
    positions[5] = [(145, 0), (148, 2), (0, 0), (0, 0), (0, 0), (147, 9), (147, 3), (148, 8), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[5] = [(25, 35), (8, 1), (0, 0), (0, 0), (0, 0), (1, 5), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[5] = [Appearance(8), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[5] = 5;
    entities[6] = Component::Animate;
    positions[6] = [(174, 0), (177, 2), (0, 0), (0, 0), (177, 14), (176, 9), (176, 3), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[6] = [(25, 35), (8, 1), (0, 0), (0, 0), (8, 1), (1, 5), (1, 5), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[6] = [Appearance(10), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[6] = 6;
    entities[7] = Component::Animate;
    positions[7] = [(0, 17), (3, 19), (11, 20), (11, 26), (3, 31), (0, 0), (0, 0), (3, 25), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[7] = [(25, 35), (8, 1), (1, 5), (1, 5), (8, 1), (0, 0), (0, 0), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[7] = [Appearance(11), Appearance(8), Appearance(8), Appearance(8), Appearance(8), Appearance(8), Appearance(8), Appearance(8), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[7] = 7;
    entities[8] = Component::Animate;
    positions[8] = [(29, 17), (0, 0), (0, 0), (40, 26), (32, 31), (31, 26), (31, 20), (32, 25), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[8] = [(25, 35), (0, 0), (0, 0), (1, 5), (8, 1), (1, 5), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[8] = [Appearance(10), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[8] = 8;
    entities[9] = Component::Animate;
    positions[9] = [(58, 17), (61, 19), (69, 20), (69, 26), (61, 31), (60, 26), (60, 20), (61, 25), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[9] = [(25, 35), (8, 1), (1, 5), (1, 5), (8, 1), (1, 5), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[9] = [Appearance(10), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[9] = 9;
    entities[10] = Component::Animate;
    positions[10] = [(87, 17), (90, 19), (0, 0), (0, 0), (0, 0), (89, 26), (89, 20), (90, 25), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[10] = [(25, 35), (8, 1), (0, 0), (0, 0), (0, 0), (1, 5), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[10] = [Appearance(8), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[10] = 10;
    entities[11] = Component::Animate;
    positions[11] = [(116, 17), (119, 19), (0, 0), (0, 0), (0, 0), (118, 26), (118, 20), (119, 25), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[11] = [(25, 35), (8, 1), (0, 0), (0, 0), (0, 0), (1, 5), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[11] = [Appearance(13), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[11] = 11;
    entities[12] = Component::Animate;
    positions[12] = [(145, 17), (148, 19), (0, 0), (156, 26), (148, 31), (0, 0), (147, 20), (148, 25), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[12] = [(25, 35), (8, 1), (0, 0), (1, 5), (8, 1), (0, 0), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[12] = [Appearance(9), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[12] = 12;
    entities[13] = Component::Animate;
    positions[13] = [(174, 17), (177, 19), (185, 20), (185, 26), (0, 0), (176, 26), (176, 20), (177, 25), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[13] = [(25, 35), (8, 1), (1, 5), (1, 5), (0, 0), (1, 5), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[13] = [Appearance(10), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[13] = 13;
    entities[14] = Component::Animate;
    positions[14] = [(0, 34), (3, 36), (11, 37), (11, 43), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[14] = [(25, 35), (8, 1), (1, 5), (1, 5), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[14] = [Appearance(11), Appearance(10), Appearance(10), Appearance(10), Appearance(10), Appearance(10), Appearance(10), Appearance(10), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[14] = 14;
    entities[15] = Component::Animate;
    positions[15] = [(29, 34), (0, 0), (40, 37), (40, 43), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[15] = [(25, 35), (0, 0), (1, 5), (1, 5), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[15] = [Appearance(11), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[15] = 15;
    entities[16] = Component::Animate;
    positions[16] = [(58, 34), (61, 36), (69, 37), (69, 43), (0, 0), (60, 43), (60, 37), (61, 42), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[16] = [(25, 35), (8, 1), (1, 5), (1, 5), (0, 0), (1, 5), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[16] = [Appearance(11), Appearance(12), Appearance(12), Appearance(12), Appearance(12), Appearance(12), Appearance(12), Appearance(12), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[16] = 16;
    entities[17] = Component::Animate;
    positions[17] = [(87, 34), (90, 36), (98, 37), (98, 43), (90, 48), (0, 0), (89, 37), (90, 42), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[17] = [(25, 35), (8, 1), (1, 5), (1, 5), (8, 1), (0, 0), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[17] = [Appearance(12), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[17] = 17;
    entities[18] = Component::Animate;
    positions[18] = [(116, 34), (119, 36), (0, 0), (127, 43), (119, 48), (0, 0), (118, 37), (119, 42), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[18] = [(25, 35), (8, 1), (0, 0), (1, 5), (8, 1), (0, 0), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[18] = [Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[18] = 18;
    entities[19] = Component::Animate;
    positions[19] = [(145, 34), (148, 36), (156, 37), (0, 0), (148, 48), (147, 43), (0, 0), (148, 42), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[19] = [(25, 35), (8, 1), (1, 5), (0, 0), (8, 1), (1, 5), (0, 0), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[19] = [Appearance(9), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[19] = 19;
    entities[20] = Component::Animate;
    positions[20] = [(174, 34), (177, 36), (0, 0), (0, 0), (0, 0), (176, 43), (176, 37), (177, 42), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[20] = [(25, 35), (8, 1), (0, 0), (0, 0), (0, 0), (1, 5), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[20] = [Appearance(14), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[20] = 20;
    entities[21] = Component::Animate;
    positions[21] = [(0, 51), (3, 53), (0, 0), (11, 60), (3, 65), (2, 60), (2, 54), (3, 59), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[21] = [(25, 35), (8, 1), (0, 0), (1, 5), (8, 1), (1, 5), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[21] = [Appearance(11), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[21] = 21;
    entities[22] = Component::Animate;
    positions[22] = [(29, 51), (32, 53), (40, 54), (40, 60), (0, 0), (31, 60), (31, 54), (32, 59), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[22] = [(25, 35), (8, 1), (1, 5), (1, 5), (0, 0), (1, 5), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[22] = [Appearance(12), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[22] = 22;
    entities[23] = Component::Animate;
    positions[23] = [(58, 51), (61, 53), (0, 0), (0, 0), (61, 65), (60, 60), (60, 54), (61, 59), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[23] = [(25, 35), (8, 1), (0, 0), (0, 0), (8, 1), (1, 5), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[23] = [Appearance(15), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[23] = 23;
    entities[24] = Component::Animate;
    positions[24] = [(87, 51), (90, 53), (0, 0), (98, 60), (90, 65), (0, 0), (89, 54), (90, 59), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[24] = [(25, 35), (8, 1), (0, 0), (1, 5), (8, 1), (0, 0), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[24] = [Appearance(9), Appearance(9), Appearance(9), Appearance(9), Appearance(9), Appearance(9), Appearance(9), Appearance(9), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[24] = 24;
    entities[25] = Component::Animate;
    positions[25] = [(116, 51), (119, 53), (0, 0), (0, 0), (119, 65), (118, 60), (118, 54), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[25] = [(25, 35), (8, 1), (0, 0), (0, 0), (8, 1), (1, 5), (1, 5), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[25] = [Appearance(9), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[25] = 25;
    entities[26] = Component::Animate;
    positions[26] = [(145, 51), (148, 53), (0, 0), (0, 0), (148, 65), (147, 60), (147, 54), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[26] = [(25, 35), (8, 1), (0, 0), (0, 0), (8, 1), (1, 5), (1, 5), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[26] = [Appearance(8), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[26] = 26;
    entities[27] = Component::Player;
    positions[27] = [(116, 205), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[27] = [(25, 35), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[27] = [Appearance(3), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[27] = 0;


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

            pub fn move_left ( & mut self , id : usize ) {
let positions = & mut self . positions [ id ] ; for i in 0 .. positions . len
(  ) {
let ( x , y ) = screen_to_grid ( positions [ i ] ) ; positions [ i ] =
grid_to_screen ( ( x - 1 , y ) ) ; } } pub fn move_right (
& mut self , id : usize ) {
let positions = & mut self . positions [ id ] ; for i in 0 .. positions . len
(  ) {
let ( x , y ) = screen_to_grid ( positions [ i ] ) ; positions [ i ] =
grid_to_screen ( ( x + 1 , y ) ) ; } } pub fn move_up (
& mut self , id : usize ) {
let positions = & mut self . positions [ id ] ; for i in 0 .. positions . len
(  ) {
let ( x , y ) = screen_to_grid ( positions [ i ] ) ; positions [ i ] =
grid_to_screen ( ( x , y - 1 ) ) ; } } pub fn move_down (
& mut self , id : usize ) {
let positions = & mut self . positions [ id ] ; for i in 0 .. positions . len
(  ) {
let ( x , y ) = screen_to_grid ( positions [ i ] ) ; positions [ i ] =
grid_to_screen ( ( x , y + 1 ) ) ; } }
        }

        type GridPos = ( GridX , GridY ) ; struct GridX ( u8 ) ; impl GridX {
fn new ( n : u8 ) -> Self {
if n > GameState :: GRID_DIMENSIONS . 0 {
GridX ( GameState :: GRID_DIMENSIONS . 0 ) } else { GridX ( n ) } } } struct
GridY ( u8 ) ; impl GridY {
fn new ( n : u8 ) -> Self {
if n > GameState :: GRID_DIMENSIONS . 1 {
GridY ( GameState :: GRID_DIMENSIONS . 1 ) } else { GridY ( n ) } } } use std
:: ops :: Add ; use std :: ops :: Sub ; macro_rules ! add_sub_impl {
( $ ( $ type : ty ) , * ) => {
$ (
impl Add < $ type > for $ type {
type Output = $ type ; fn add ( self , other : $ type ) -> $ type {
let result = self . 0 . saturating_add ( other . 0 ) ; < $ type > :: new (
result ) } } impl Add < u8 > for $ type {
type Output = $ type ; fn add ( self , other : u8 ) -> $ type {
let result = self . 0 . saturating_add ( other ) ; < $ type > :: new ( result
) } } impl Add < $ type > for u8 {
type Output = $ type ; fn add ( self , other : $ type ) -> $ type {
let result = self . saturating_add ( other . 0 ) ; < $ type > :: new ( result
) } } impl Sub < $ type > for $ type {
type Output = $ type ; fn sub ( self , other : $ type ) -> $ type {
let result = self . 0 . saturating_sub ( other . 0 ) ; < $ type > :: new (
result ) } } impl Sub < u8 > for $ type {
type Output = $ type ; fn sub ( self , other : u8 ) -> $ type {
let result = self . 0 . saturating_sub ( other ) ; < $ type > :: new ( result
) } } impl Sub < $ type > for u8 {
type Output = $ type ; fn sub ( self , other : $ type ) -> $ type {
let result = self . saturating_sub ( other . 0 ) ; < $ type > :: new ( result
) } } ) * } } add_sub_impl ! { GridX , GridY } fn screen_to_grid (
screen_pos : ( u8 , u8 ) ) -> GridPos {
let ( x , y ) = card :: screen_to_grid ( screen_pos ) ; (
GridX :: new ( x ) , GridY :: new ( y ) ) } fn grid_to_screen (
( x , y ) : GridPos ) -> ( u8 , u8 ) {
card :: grid_to_screen ( ( x . 0 , y . 0 ) ) }
        