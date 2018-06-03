use rand::Rng;

use common::*;
use error::Result;

pub fn render_game<R: Rng + Sized>(rng: &mut R, spec: SolitaireSpec) -> Result<RenderableGame> {
    let mut button_responses = ButtonResponses::default();

    button_responses.left = code_string!(
        state.move_left(id);
    );
    button_responses.right = code_string!{
        state.move_right(id);
    };
    button_responses.up = code_string!{
        state.move_up(id);
    };
    button_responses.down = code_string!{
        state.move_down(id);
    };

    let responder = InputResponder {
        button_responses,
        variety: Default::default(),
    };

    let minimum_card_count = spec.deck.get_minimum_card_count();

    let capacity = minimum_card_count as usize;

    let mut animacies: Vec<EntityAnimacy> = Vec::with_capacity(capacity);
    let mut positions: Vec<Vec<(u8, u8)>> = Vec::with_capacity(capacity);
    let mut appearances: Vec<Vec<Appearance>> = Vec::with_capacity(capacity);
    let mut sizes: Vec<Vec<(u8, u8)>> = Vec::with_capacity(capacity);
    let mut varieties: Vec<u8> = Vec::with_capacity(capacity);

    let (w, h) = spec.grid_dimensions;

    for i in 0..minimum_card_count {
        animacies.push(Animate);

        let grid_pos = (i % w, i / w);

        let base_pos = card::grid_to_screen(grid_pos);

        let card_appearance = generate_solitaire_card_appearance(rng, base_pos);

        positions.push(card_appearance.positions.to_vec());
        sizes.push(card_appearance.sizes.to_vec());
        appearances.push(card_appearance.appearances.to_vec());

        varieties.push(i);
    }

    {
        animacies.push(Default::default());
        let mut cursor_positions: [(u8, u8); SOLITAIRE_ENTITY_PIECE_COUNT] = Default::default();
        let mut cursor_sizes: [(u8, u8); SOLITAIRE_ENTITY_PIECE_COUNT] = Default::default();
        let mut cursor_appearances: [Appearance; SOLITAIRE_ENTITY_PIECE_COUNT] = Default::default();

        cursor_positions[0] = (
            (SCREEN_WIDTH / 2) as u8 - (card::WIDTH / 2),
            SCREEN_HEIGHT as u8 - card::HEIGHT,
        );
        cursor_sizes[0] = (card::WIDTH, card::HEIGHT);
        cursor_appearances[0] = Yellow | Rectangle;

        positions.push(cursor_positions.to_vec());
        sizes.push(cursor_sizes.to_vec());
        appearances.push(cursor_appearances.to_vec());

        varieties.push(0);
    }

    let initial_state = InitialState {
        animacies,
        positions,
        sizes,
        appearances,
        varieties,
    };

    let custom_code = code_string!{
        impl GameState {
            pub fn move_left(&mut self, id: usize) {
                GameState::move_in_direction(self, id, Direction::Left);
            }

            pub fn move_right(&mut self, id: usize) {
                GameState::move_in_direction(self, id, Direction::Right);
            }

            pub fn move_up(&mut self, id: usize) {
                GameState::move_in_direction(self, id, Direction::Up);
            }

            pub fn move_down(&mut self, id: usize) {
                GameState::move_in_direction(self, id, Direction::Down);
            }

            fn move_in_direction(&mut self, id: usize, dir: Direction) {
                let grid_pos = self.get_cursor_pos(id);

                let new_pos = match dir {
                    Direction::Left => {
                        match grid_pos {
                            GridPos::Main(x, y) => {
                                 GridPos::Main(x - 1, y)
                            }
                            GridPos::FoundationLeft => GridPos::FoundationLeft,
                            GridPos::FoundationMiddle => GridPos::FoundationLeft,
                            GridPos::FoundationRight => GridPos::FoundationMiddle,
                        }
                    },
                    Direction::Right => {
                        match grid_pos {
                            GridPos::Main(x, y) => {
                                 GridPos::Main(x + 1, y)
                            }
                            GridPos::FoundationLeft => GridPos::FoundationMiddle,
                            GridPos::FoundationMiddle => GridPos::FoundationRight,
                            GridPos::FoundationRight => GridPos::FoundationRight,
                        }
                    },
                    Direction::Up => {
                        match grid_pos {
                            GridPos::Main(x, y) => {
                                 GridPos::Main(x, y - 1)
                            }
                            GridPos::FoundationLeft => GridPos::Main(GridX::new(2), GridY::new(GameState::GRID_DIMENSIONS.1 - 1)),
                            GridPos::FoundationMiddle => GridPos::Main(GridX::new(3), GridY::new(GameState::GRID_DIMENSIONS.1 - 1)),
                            GridPos::FoundationRight => GridPos::Main(GridX::new(4), GridY::new(GameState::GRID_DIMENSIONS.1 - 1)),
                        }
                    },
                    Direction::Down => {
                        match grid_pos {
                            GridPos::Main(x, y) => {
                                let newY = y + 1;

                                if newY == y {
                                    if x >= GridX::new(4) {
                                        GridPos::FoundationRight
                                    } else if x <= GridX::new(2) {
                                        GridPos::FoundationLeft
                                    } else {
                                        GridPos::FoundationMiddle
                                    }
                                } else {
                                    GridPos::Main(x, newY)
                                }
                            }
                            GridPos::FoundationLeft => GridPos::FoundationLeft,
                            GridPos::FoundationMiddle => GridPos::FoundationMiddle,
                            GridPos::FoundationRight => GridPos::FoundationRight,
                        }
                    },
                };

                self.set_cursor_pos(id, new_pos);
            }

            fn get_cursor_pos(&self, id: usize) -> GridPos {
                let positions = &self.positions[id];

                if let Some(pos) = foundation::from_screen(positions[0]) {
                    return pos;
                }

                screen_to_grid(positions[0])
            }

            fn set_cursor_pos(&mut self, id: usize, grid_pos: GridPos) {
                let positions = &mut self.positions[id];

                match grid_pos {
                    GridPos::Main(x, y) => {
                        positions[0] = grid_to_screen((x, y));
                    }
                    GridPos::FoundationLeft => {
                        positions[0] = foundation::LEFT;
                    }
                    GridPos::FoundationMiddle => {
                        positions[0] = foundation::MIDDLE;
                    }
                    GridPos::FoundationRight => {
                        positions[0] = foundation::RIGHT;
                    }
                }
            }
        }

        mod foundation {
            use super::*;
            pub const LEFT_EDGE: u8 = (2 * (card::WIDTH + card::SPACING) + card::SPACING);
            pub const TOP_EDGE: u8 = card::HEIGHT * GameState::GRID_DIMENSIONS.1;

            pub const LEFT: (u8,u8) = (
                LEFT_EDGE,
                TOP_EDGE
            );
            pub const MIDDLE: (u8,u8) = (
                LEFT_EDGE + ((card::WIDTH + card::SPACING) + card::SPACING),
                TOP_EDGE
            );
            pub const RIGHT: (u8,u8) = (
                LEFT_EDGE + (2 * (card::WIDTH + card::SPACING) + card::SPACING),
                TOP_EDGE
            );

            pub fn from_screen(screen_pos: (u8,u8)) -> Option<GridPos> {
                if screen_pos == LEFT {
                    Some(GridPos::FoundationLeft)
                } else if screen_pos == MIDDLE {
                    Some(GridPos::FoundationMiddle)
                } else if screen_pos == RIGHT {
                    Some(GridPos::FoundationRight)
                } else {
                    None
                }
            }
        }

        pub enum Direction {
            Left,
            Right,
            Down,
            Up,
        }

        pub enum GridPos {
            Main(GridX, GridY),
            FoundationLeft,
            FoundationMiddle,
            FoundationRight,
        }

        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub struct GridX(u8);
        impl GridX {
            fn new(n: u8) -> Self {
                if n >= GameState::GRID_DIMENSIONS.0 - 1 {
                    GridX(GameState::GRID_DIMENSIONS.0 - 1)
                } else {
                    GridX(n)
                }
            }
        }

        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub struct GridY(u8);
        impl GridY {
            fn new(n: u8) -> Self {
                if n >= GameState::GRID_DIMENSIONS.1 - 1 {
                    GridY(GameState::GRID_DIMENSIONS.1 - 1)
                } else {
                    GridY(n)
                }
            }
        }

        use std::ops::Add;
        use std::ops::Sub;

        macro_rules! add_sub_impl {
            ($($type:ty),*) => {
                $(
                    impl Add<$type> for $type {
                        type Output = $type;

                        fn add(self, other: $type) -> $type {
                            let result = self.0.saturating_add(other.0);

                            <$type>::new(result)
                        }
                    }

                    impl Add<u8> for $type {
                        type Output = $type;

                        fn add(self, other: u8) -> $type {
                            let result = self.0.saturating_add(other);

                            <$type>::new(result)
                        }
                    }

                    impl Add<$type> for u8 {
                        type Output = $type;

                        fn add(self, other: $type) -> $type {
                            let result = self.saturating_add(other.0);

                            <$type>::new(result)
                        }
                    }

                    impl Sub<$type> for $type {
                        type Output = $type;

                        fn sub(self, other: $type) -> $type {
                            let result = self.0.saturating_sub(other.0);

                            <$type>::new(result)
                        }
                    }

                    impl Sub<u8> for $type {
                        type Output = $type;

                        fn sub(self, other: u8) -> $type {
                            let result = self.0.saturating_sub(other);

                            <$type>::new(result)
                        }
                    }

                    impl Sub<$type> for u8 {
                        type Output = $type;

                        fn sub(self, other: $type) -> $type {
                            let result = self.saturating_sub(other.0);

                            <$type>::new(result)
                        }
                    }
                )*
            }
        }

        add_sub_impl!{GridX, GridY}

        fn screen_to_grid(screen_pos: (u8,u8)) -> GridPos {
            let (x, y) = card::screen_to_grid(screen_pos);

            GridPos::Main(GridX::new(x), GridY::new(y))
        }

        fn grid_to_screen((x, y): (GridX, GridY)) -> (u8,u8) {
            card::grid_to_screen((x.0, y.0))
        }
    };

    let game_state_impl = GameStateImpl {
        entity_count: 256,
        entity_piece_count: SOLITAIRE_ENTITY_PIECE_COUNT,
        custom_consts: format!(
            stringify!{
                pub const GRID_DIMENSIONS: (u8, u8) = ({}, {});
            },
            w,
            h
        ),
        initial_state,
        custom_code,
        ..Default::default()
    };

    Ok(RenderableGame {
        game_type: Solitaire,
        input_responders: vec![responder],
        game_state_impl,
        grid_dimensions: spec.grid_dimensions,
    })
}

struct SevenSegmentAppearance {
    positions: [(u8, u8); 7],
    sizes: [(u8, u8); 7],
    appearances: [Appearance; 7],
}

//seven segment spec format is 0b_gfedcba
//   -a-
//   f b
//   -g-
//   e c
//   -d-

fn digit_to_segment_spec(digit: u8) -> u8 {
    match digit & 0b1111 {
        0 => 0b0111111,
        1 => 0b0000110,
        2 => 0b1011011,
        3 => 0b1001111,
        4 => 0b1100110,
        5 => 0b1101101,
        6 => 0b1111101,
        7 => 0b0000111,
        8 => 0b1111111,
        9 => 0b1101111,
        10 => 0b1110111,
        11 => 0b1111100,
        12 => 0b0111001,
        13 => 0b1011110,
        14 => 0b1111001,
        15 => 0b1110001,
        _ => 0,
    }
}

fn render_seven_segment(
    digit: u8,
    (x, y): (u8, u8),
    (w, h): (u8, u8),
    colour: Colour,
) -> SevenSegmentAppearance {
    let mut positions = [Default::default(); 7];
    let mut sizes = [Default::default(); 7];
    let appearances = [colour | FilledRectangle; 7];

    let spec: u8 = digit_to_segment_spec(digit);

    let short_width = 1;
    let long_width = w - (short_width * 2);

    let short_height = 1;
    let long_height = (h / 2) - (short_height * 2);

    if spec & (1 << 0) == 1 << 0 {
        positions[0] = (x + short_width, y);
        sizes[0] = (long_width, short_height);
    }

    if spec & (1 << 1) == 1 << 1 {
        positions[1] = (x + long_width + short_width, y + short_height);
        sizes[1] = (short_width, long_height);
    }

    if spec & (1 << 2) == 1 << 2 {
        positions[2] = (
            x + long_width + short_width,
            y + (short_height * 2) + long_height,
        );
        sizes[2] = (short_width, long_height);
    }

    if spec & (1 << 3) == 1 << 3 {
        positions[3] = (x + short_width, y + (short_height * 2) + (long_height * 2));
        sizes[3] = (long_width, short_height);
    }

    if spec & (1 << 4) == 1 << 4 {
        positions[4] = (x, y + (short_height * 2) + long_height);
        sizes[4] = (short_width, long_height);
    }

    if spec & (1 << 5) == 1 << 5 {
        positions[5] = (x, y + short_height);
        sizes[5] = (short_width, long_height);
    }

    if spec & (1 << 6) == 1 << 6 {
        positions[6] = (x + short_width, y + short_height + long_height);
        sizes[6] = (long_width, short_height);
    }

    SevenSegmentAppearance {
        positions,
        sizes,
        appearances,
    }
}

struct CardAppearance {
    positions: [(u8, u8); SOLITAIRE_ENTITY_PIECE_COUNT],
    sizes: [(u8, u8); SOLITAIRE_ENTITY_PIECE_COUNT],
    appearances: [Appearance; SOLITAIRE_ENTITY_PIECE_COUNT],
}

fn generate_solitaire_card_appearance<R: Rng + Sized>(
    rng: &mut R,
    (x, y): (u8, u8),
) -> CardAppearance {
    //TODO design more cards

    let mut positions: [(u8, u8); SOLITAIRE_ENTITY_PIECE_COUNT] = Default::default();
    let mut sizes: [(u8, u8); SOLITAIRE_ENTITY_PIECE_COUNT] = Default::default();
    let mut appearances: [Appearance; SOLITAIRE_ENTITY_PIECE_COUNT] = Default::default();

    const SPACING: u8 = 2;

    let mut i = 0;

    positions[i] = (x, y);
    sizes[i] = (card::WIDTH, card::HEIGHT);
    appearances[i] = Colour::from(rng.gen_range(0, COLOUR_COUNT)) | FilledRectangle;

    i += 1;

    let digit = rng.gen_range(0, 16);

    let ssa: SevenSegmentAppearance = render_seven_segment(
        digit,
        (x + SPACING, y + SPACING),
        (
            (card::WIDTH - (SPACING * 2)) / 2,
            (card::HEIGHT - (SPACING * 2)) / 2,
        ),
        Colour::from(rng.gen_range(0, COLOUR_COUNT)),
    );

    for ssa_index in 0..7 {
        positions[i] = ssa.positions[ssa_index];
        sizes[i] = ssa.sizes[ssa_index];
        appearances[i] = ssa.appearances[ssa_index];

        i += 1;
    }

    CardAppearance {
        positions,
        sizes,
        appearances,
    }
}

const SOLITAIRE_ENTITY_PIECE_COUNT: usize = 32;
