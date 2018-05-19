
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

            entities[0] = Component::Player;
    positions[0] = [(116, 205), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[0] = [(25, 35), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[0] = [Appearance(3), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[0] = 0;
    entities[1] = Component::Animate;
    positions[1] = [(29, 0), (32, 2), (40, 3), (40, 9), (32, 14), (0, 0), (0, 0), (32, 8), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[1] = [(25, 35), (8, 1), (1, 5), (1, 5), (8, 1), (0, 0), (0, 0), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[1] = [Appearance(14), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[1] = 1;
    entities[2] = Component::Animate;
    positions[2] = [(56, 0), (0, 0), (67, 3), (67, 9), (0, 0), (0, 0), (58, 3), (59, 8), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[2] = [(25, 35), (0, 0), (1, 5), (1, 5), (0, 0), (0, 0), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[2] = [Appearance(13), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[2] = 2;
    entities[3] = Component::Animate;
    positions[3] = [(83, 0), (0, 0), (0, 0), (94, 9), (86, 14), (85, 9), (85, 3), (86, 8), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[3] = [(25, 35), (0, 0), (0, 0), (1, 5), (8, 1), (1, 5), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[3] = [Appearance(10), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[3] = 3;
    entities[4] = Component::Animate;
    positions[4] = [(110, 0), (0, 0), (0, 0), (121, 9), (113, 14), (112, 9), (112, 3), (113, 8), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[4] = [(25, 35), (0, 0), (0, 0), (1, 5), (8, 1), (1, 5), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[4] = [Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[4] = 4;
    entities[5] = Component::Animate;
    positions[5] = [(137, 0), (0, 0), (148, 3), (148, 9), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[5] = [(25, 35), (0, 0), (1, 5), (1, 5), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[5] = [Appearance(9), Appearance(12), Appearance(12), Appearance(12), Appearance(12), Appearance(12), Appearance(12), Appearance(12), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[5] = 5;
    entities[6] = Component::Animate;
    positions[6] = [(164, 0), (167, 2), (0, 0), (0, 0), (0, 0), (166, 9), (166, 3), (167, 8), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[6] = [(25, 35), (8, 1), (0, 0), (0, 0), (0, 0), (1, 5), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[6] = [Appearance(8), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[6] = 6;
    entities[7] = Component::Animate;
    positions[7] = [(2, 17), (5, 19), (0, 0), (0, 0), (5, 31), (4, 26), (4, 20), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[7] = [(25, 35), (8, 1), (0, 0), (0, 0), (8, 1), (1, 5), (1, 5), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[7] = [Appearance(10), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[7] = 7;
    entities[8] = Component::Animate;
    positions[8] = [(29, 17), (32, 19), (40, 20), (40, 26), (32, 31), (0, 0), (0, 0), (32, 25), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[8] = [(25, 35), (8, 1), (1, 5), (1, 5), (8, 1), (0, 0), (0, 0), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[8] = [Appearance(11), Appearance(8), Appearance(8), Appearance(8), Appearance(8), Appearance(8), Appearance(8), Appearance(8), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[8] = 8;
    entities[9] = Component::Animate;
    positions[9] = [(56, 17), (0, 0), (0, 0), (67, 26), (59, 31), (58, 26), (58, 20), (59, 25), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[9] = [(25, 35), (0, 0), (0, 0), (1, 5), (8, 1), (1, 5), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[9] = [Appearance(10), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[9] = 9;
    entities[10] = Component::Animate;
    positions[10] = [(83, 17), (86, 19), (94, 20), (94, 26), (86, 31), (85, 26), (85, 20), (86, 25), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[10] = [(25, 35), (8, 1), (1, 5), (1, 5), (8, 1), (1, 5), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[10] = [Appearance(10), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[10] = 10;
    entities[11] = Component::Animate;
    positions[11] = [(110, 17), (113, 19), (0, 0), (0, 0), (0, 0), (112, 26), (112, 20), (113, 25), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[11] = [(25, 35), (8, 1), (0, 0), (0, 0), (0, 0), (1, 5), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[11] = [Appearance(8), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[11] = 11;
    entities[12] = Component::Animate;
    positions[12] = [(137, 17), (140, 19), (0, 0), (0, 0), (0, 0), (139, 26), (139, 20), (140, 25), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[12] = [(25, 35), (8, 1), (0, 0), (0, 0), (0, 0), (1, 5), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[12] = [Appearance(13), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[12] = 12;
    entities[13] = Component::Animate;
    positions[13] = [(164, 17), (167, 19), (0, 0), (175, 26), (167, 31), (0, 0), (166, 20), (167, 25), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[13] = [(25, 35), (8, 1), (0, 0), (1, 5), (8, 1), (0, 0), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[13] = [Appearance(9), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[13] = 13;
    entities[14] = Component::Animate;
    positions[14] = [(2, 34), (5, 36), (13, 37), (13, 43), (0, 0), (4, 43), (4, 37), (5, 42), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[14] = [(25, 35), (8, 1), (1, 5), (1, 5), (0, 0), (1, 5), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[14] = [Appearance(10), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(14), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[14] = 14;
    entities[15] = Component::Animate;
    positions[15] = [(29, 34), (32, 36), (40, 37), (40, 43), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[15] = [(25, 35), (8, 1), (1, 5), (1, 5), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[15] = [Appearance(11), Appearance(10), Appearance(10), Appearance(10), Appearance(10), Appearance(10), Appearance(10), Appearance(10), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[15] = 15;
    entities[16] = Component::Animate;
    positions[16] = [(56, 34), (0, 0), (67, 37), (67, 43), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[16] = [(25, 35), (0, 0), (1, 5), (1, 5), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[16] = [Appearance(11), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[16] = 16;
    entities[17] = Component::Animate;
    positions[17] = [(83, 34), (86, 36), (94, 37), (94, 43), (0, 0), (85, 43), (85, 37), (86, 42), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[17] = [(25, 35), (8, 1), (1, 5), (1, 5), (0, 0), (1, 5), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[17] = [Appearance(11), Appearance(12), Appearance(12), Appearance(12), Appearance(12), Appearance(12), Appearance(12), Appearance(12), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[17] = 17;
    entities[18] = Component::Animate;
    positions[18] = [(110, 34), (113, 36), (121, 37), (121, 43), (113, 48), (0, 0), (112, 37), (113, 42), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[18] = [(25, 35), (8, 1), (1, 5), (1, 5), (8, 1), (0, 0), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[18] = [Appearance(12), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[18] = 18;
    entities[19] = Component::Animate;
    positions[19] = [(137, 34), (140, 36), (0, 0), (148, 43), (140, 48), (0, 0), (139, 37), (140, 42), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[19] = [(25, 35), (8, 1), (0, 0), (1, 5), (8, 1), (0, 0), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[19] = [Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[19] = 19;
    entities[20] = Component::Animate;
    positions[20] = [(164, 34), (167, 36), (175, 37), (0, 0), (167, 48), (166, 43), (0, 0), (167, 42), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[20] = [(25, 35), (8, 1), (1, 5), (0, 0), (8, 1), (1, 5), (0, 0), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[20] = [Appearance(9), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[20] = 20;
    entities[21] = Component::Animate;
    positions[21] = [(2, 51), (5, 53), (0, 0), (0, 0), (0, 0), (4, 60), (4, 54), (5, 59), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[21] = [(25, 35), (8, 1), (0, 0), (0, 0), (0, 0), (1, 5), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[21] = [Appearance(14), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[21] = 21;
    entities[22] = Component::Animate;
    positions[22] = [(29, 51), (32, 53), (0, 0), (40, 60), (32, 65), (31, 60), (31, 54), (32, 59), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[22] = [(25, 35), (8, 1), (0, 0), (1, 5), (8, 1), (1, 5), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[22] = [Appearance(11), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[22] = 22;
    entities[23] = Component::Animate;
    positions[23] = [(56, 51), (59, 53), (67, 54), (67, 60), (0, 0), (58, 60), (58, 54), (59, 59), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[23] = [(25, 35), (8, 1), (1, 5), (1, 5), (0, 0), (1, 5), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[23] = [Appearance(12), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(13), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[23] = 23;
    entities[24] = Component::Animate;
    positions[24] = [(83, 51), (86, 53), (0, 0), (0, 0), (86, 65), (85, 60), (85, 54), (86, 59), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[24] = [(25, 35), (8, 1), (0, 0), (0, 0), (8, 1), (1, 5), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[24] = [Appearance(15), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(11), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[24] = 24;
    entities[25] = Component::Animate;
    positions[25] = [(110, 51), (113, 53), (0, 0), (121, 60), (113, 65), (0, 0), (112, 54), (113, 59), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[25] = [(25, 35), (8, 1), (0, 0), (1, 5), (8, 1), (0, 0), (1, 5), (8, 1), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[25] = [Appearance(9), Appearance(9), Appearance(9), Appearance(9), Appearance(9), Appearance(9), Appearance(9), Appearance(9), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[25] = 25;
    entities[26] = Component::Animate;
    positions[26] = [(137, 51), (140, 53), (0, 0), (0, 0), (140, 65), (139, 60), (139, 54), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    sizes[26] = [(25, 35), (8, 1), (0, 0), (0, 0), (8, 1), (1, 5), (1, 5), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), ];
    appearances[26] = [Appearance(9), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(15), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), Appearance(255), ];
    varieties[26] = 26;


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

type GridPos = ( GridX , GridY ) ; type GridX = u8 ; type GridY = u8 ; fn
screen_to_grid ( ( x , y ) : ( u8 , u8 ) ) -> GridPos { ( x , y ) } fn
grid_to_screen ( ( x , y ) : GridPos ) -> ( u8 , u8 ) { ( x , y ) }
