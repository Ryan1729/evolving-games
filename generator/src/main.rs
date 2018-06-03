use std::fmt;
use std::fs::OpenOptions;
use std::io::Write as IOWrite;
use std::path::Path;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

extern crate rand;
use rand::{Rng, SeedableRng, XorShiftRng};

extern crate project_common;

#[macro_use]
mod common;
use common::*;

mod grid;
use grid::render_game as render_grid_game;

mod guess;
use guess::render_game as render_guess_game;

#[macro_use]
mod error;
use error::{Error, ErrorKind, Result};

fn main() {
    let seed: [u32; 4] = {
        let mut args = std::env::args();
        //exe name
        args.next();

        args.next()
            .map(|s| {
                let mut result = [0u8; 16];
                let bytes = s.as_bytes();
                for i in 0..16 {
                    result[i] = bytes[i % bytes.len()];
                }
                unsafe { std::mem::transmute(result) }
            })
            .unwrap_or_else(|| {
                let since_the_epoch = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_else(|_| Duration::new(42, 42));

                let seconds: [u32; 2] = unsafe { std::mem::transmute(since_the_epoch.as_secs()) };

                [seconds[0], seconds[1], seconds[0], seconds[1]]
            })
    };

    println!("\nUsing {:?} as a seed.\n", seed);

    let mut rng = XorShiftRng::from_seed(seed);

    let code = match generate_game(&mut rng) {
        Err(error) => {
            println!("{}", error);
            return;
        }
        Ok(c) => c,
    };

    overwrite_filename("../player/src/game.rs", code.update_and_render).unwrap();

    overwrite_filename("../player/src/common/game_state.rs", code.game_state_impl).unwrap();
}

fn overwrite_filename(filename: &str, data: String) -> std::io::Result<()> {
    let path = Path::new(filename);
    println!("Overwriting {:?}.", path.as_os_str());

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)
        .unwrap();

    let bytes = data.as_bytes();

    let result = IOWrite::write_all(&mut file, bytes);

    if let Err(error) = result {
        println!("{}", error);
        Err(error)
    } else {
        let len = bytes.len();
        println!("Overwrote {} successfully with {} bytes.", filename, len);
        Ok(())
    }
}

fn generate_game<R: Rng + Sized>(rng: &mut R) -> Result<RenderedGame> {
    generate_spec(rng).and_then(|spec: GameSpec| {
        println!("{:#?}", spec);
        //TODO Separate seed for these RNG calls?
        render_spec(rng, spec)
    })
}

fn generate_game_type<R: Rng + Sized>(rng: &mut R) -> GameType {
    match rng.gen_range(0, 4) {
        0 => ErrorTest,
        1 => Solitaire,
        2 => GridBased,
        _ => Guess,
    }
}

fn generate_spec<R: Rng + Sized>(rng: &mut R) -> Result<GameSpec> {
    let game_type = generate_game_type(rng);

    match game_type {
        ErrorTest => err!(ErrorKind::NotImplemented),
        Solitaire => generate_solitaire_spec(rng).map(GameSpec::Solitaire),
        GridBased => generate_grid_spec(rng).map(GameSpec::Grid),
        Guess => Ok(Default::default()),
    }
}

fn generate_solitaire_spec<R: Rng + Sized>(rng: &mut R) -> Result<SolitaireSpec> {
    let deck = ThreeColour(rng.gen_range(6, 12));

    let grid_dimensions = match deck {
        ThreeColour(highest_card) => {
            let minimum_card_count = deck.get_minimum_card_count();

            let w = rng.gen_range(highest_card / 2, highest_card) as u8;
            //Ceiling division
            let h = ((minimum_card_count - 1) / w) + 1;

            (w, h)
        }
    };

    Ok(SolitaireSpec {
        grid_dimensions,
        deck,
    })
}

impl DeckType {
    fn get_minimum_card_count(&self) -> u8 {
        match *self {
            ThreeColour(highest_card) => highest_card.saturating_mul(3),
        }
    }
}

fn generate_grid_spec<R: Rng + Sized>(rng: &mut R) -> Result<GridGameSpec> {
    let grid_dimensions = (
        rng.gen_range(0, SCREEN_WIDTH) as u8,
        rng.gen_range(0, SCREEN_HEIGHT) as u8,
    );

    let goal = generate_grid_goal(rng);

    let Ontology {
        entity_animacies,
        entity_controls,
    } = generate_ontology(rng, goal);

    Ok(GridGameSpec {
        grid_dimensions,
        goal,
        entity_animacies,
        entity_controls,
    })
}

fn generate_grid_goal<R: Rng + Sized>(rng: &mut R) -> Goal {
    match rng.gen_range(0, GOAL_COUNT) {
        0 => AddSomeNumberOfXsToTheGrid,
        1 => MoveAllXsToALocation,
        2 => FreeAllTrappedXs,
        3 => TransformAllXsIntoYs,
        4 => MakeAllXsTouchTheGroupofXs,
        _ => MakeAllXsTouchAtLeastOneY,
    }
}

//We are using the "what kind of things are there" definition of ontology.
#[derive(Debug)]
struct Ontology {
    entity_animacies: Vec<EntityAnimacy>,
    entity_controls: Vec<Option<EntityControl>>,
}

fn generate_ontology<R: Rng + Sized>(rng: &mut R, goal: Goal) -> Ontology {
    let entity_animacies = generate_entity_animacies(rng, goal);

    let entity_controls = generate_entity_controls(rng, &entity_animacies);

    Ontology {
        entity_animacies,
        entity_controls,
    }
}

fn generate_entity_controls<R: Rng + Sized>(
    rng: &mut R,
    entity_animacies: &Vec<EntityAnimacy>,
) -> Vec<Option<EntityControl>> {
    let controls_len = entity_animacies.len();

    debug_assert!(controls_len >= 1);

    let mut controls = Vec::with_capacity(controls_len);

    for e in entity_animacies.iter() {
        let contol = match *e {
            Inanimate => None,
            //I could try to encode this relationship between Animacy and Controls in the type by
            //merging those two types, but that smells like unnecessary coupling.
            _ => Some(generate_entity_control(rng)),
        };

        controls.push(contol);
    }

    controls
}

fn generate_entity_control<R: Rng + Sized>(rng: &mut R) -> EntityControl {
    EntityControl {
        movement: generate_move_control(rng),
        a: generate_action(rng),
        b: generate_action(rng),
        select: generate_action(rng),
    }
}

fn generate_move_control<R: Rng + Sized>(rng: &mut R) -> MoveControl {
    match rng.gen_range(0, MOVE_CONTROL_COUNT) {
        0 => Orthogonal,
        _ => Diagonal,
    }
}

fn generate_action<R: Rng + Sized>(rng: &mut R) -> Action {
    match rng.gen_range(0, MOVE_CONTROL_COUNT) {
        0 => SwapPlayerControlToNext(rng.gen()),
        _ => CopySelf,
    }
}

fn generate_entity_animacies<R: Rng + Sized>(rng: &mut R, goal: Goal) -> Vec<EntityAnimacy> {
    let minimum_entity_types_needed = goal.minimum_entity_types_needed();

    let entity_animacies_len =
        rng.gen_range(minimum_entity_types_needed, minimum_entity_types_needed * 2);

    debug_assert!(entity_animacies_len >= 1);

    let mut entity_animacies = Vec::with_capacity(entity_animacies_len as _);

    entity_animacies.push(PlayerControlled);

    for _ in 1..entity_animacies_len {
        entity_animacies.push(generate_entity_animacy(rng));
    }

    entity_animacies
}

fn generate_entity_animacy<R: Rng + Sized>(rng: &mut R) -> EntityAnimacy {
    match rng.gen_range(0, ENTITY_ANIMACY_COUNT) {
        0 => PlayerControlled,
        1 => Inanimate,
        _ => Animate,
    }
}

struct RenderedGame {
    pub update_and_render: String,
    pub game_state_impl: String,
}

fn format<T: fmt::Display>(t: T) -> String {
    format!("{}", t)
}

fn format_as_tuple((x, y): (u8, u8)) -> String {
    format!("({}, {})", x, y)
}

fn format_as_array<T: Copy, F>(pairs: &Vec<T>, mut format: F) -> String
where
    F: FnMut(T) -> String,
{
    let mut result = String::new();

    result.push('[');

    for pair in pairs.iter() {
        let string = format(*pair);
        result.push_str(&string);

        result.push(',');
        result.push(' ');
    }

    result.push(']');

    result
}

impl fmt::Display for InitialState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let InitialState {
            ref animacies,
            ref positions,
            ref sizes,
            ref appearances,
            ref varieties,
        } = *self;

        debug_assert!(
            animacies.len() == positions.len()
                && positions.len() == sizes.len()
                && sizes.len() == appearances.len()
                && appearances.len() == varieties.len()
        );

        for i in 0..positions.len() {
            write!(
                f,
                "    entities[{}] = {};
    positions[{0}] = {};
    sizes[{0}] = {};
    appearances[{0}] = {};
    varieties[{0}] = {};
",
                i,
                animacies[i],
                format_as_array(&positions[i], format_as_tuple),
                format_as_array(&sizes[i], format_as_tuple),
                format_as_array(&appearances[i], format),
                varieties[i],
            )?;
        }

        Ok(())
    }
}

impl fmt::Display for GameStateImpl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
                "
        use inner_common::*;

        impl GameState {{
            pub const ENTITY_COUNT: usize = {};
            pub const ENTITY_PIECE_COUNT: usize = {};
            {}

            pub fn new() -> GameState {{
                let mut entities = [Component::Ty::empty(); GameState::ENTITY_COUNT];

                let mut positions = [[(0, 0); GameState::ENTITY_PIECE_COUNT]; GameState::ENTITY_COUNT];
                let mut appearances =
                    [[Appearance::default(); GameState::ENTITY_PIECE_COUNT]; GameState::ENTITY_COUNT];
                let mut sizes = [[(0, 0); GameState::ENTITY_PIECE_COUNT]; GameState::ENTITY_COUNT];

                let mut varieties = [Variety::default(); GameState::ENTITY_COUNT];

                let player_controlling_variety = Variety::default();

                {}

                GameState {{
                    entities,
                    positions,
                    sizes,
                    appearances,
                    varieties,
                    player_controlling_variety,
                }}
            }}

            pub fn get_free_id(&self) -> Option<usize> {{
                for (i, e) in self.entities.iter().enumerate() {{
                    if e.is_empty() {{
                        return Some(i);
                    }}
                }}

                None
            }}
        }}

        {}
        ",
                self.entity_count,
                self.entity_piece_count,
                self.custom_consts,
                self.initial_state,
                self.custom_code,
            )?;

        Ok(())
    }
}

fn get_cell_dimensions(gw: u8, gh: u8) -> (u8, u8) {
    (
        (SCREEN_WIDTH / gw as usize) as _,
        (SCREEN_HEIGHT / gh as usize) as _,
    )
}

impl RenderableGame {
    fn render(self) -> RenderedGame {
        let RenderableGame {
            game_type,
            input_responders,
            game_state_impl,
            grid_dimensions,
        } = self;

        match game_type {
            Guess | ErrorTest => RenderableGame::guess_game(input_responders, game_state_impl),
            GridBased => {
                RenderableGame::grid_game(input_responders, game_state_impl, grid_dimensions)
            }
            Solitaire => RenderableGame::solitaire_game(input_responders, game_state_impl),
        }
    }

    fn solitaire_game(
        input_responders: Vec<InputResponder>,
        game_state_impl: GameStateImpl,
    ) -> RenderedGame {
        let update_and_render = format!(
            "
    use common::*;

    {}

    #[inline]
    pub fn update_and_render(framebuffer: &mut Framebuffer, state: &mut GameState, input: Input) {{
        for id in 0..GameState::ENTITY_COUNT {{
            if state.entities[id].contains(Component::PlayerControlled) {{
                respond_to_input(state, input, id, Variety::default());
            }}
        }}

        framebuffer.clear();

        for entity in 0..GameState::ENTITY_COUNT {{
            if state.entities[entity].is_empty() {{
                continue;
            }}

            for i in 0..GameState::ENTITY_PIECE_COUNT {{
                let (x, y) = state.positions[entity][i];
                let appearance = &state.appearances[entity][i];
                let (w, h) = state.sizes[entity][i];

                render(
                    appearance,
                    framebuffer,
                    (x as usize, y as usize),
                    (w as usize, h as usize)
                );
            }}
        }}
    }}
    ",
            InputResponders(input_responders),
        );

        RenderedGame {
            update_and_render,
            game_state_impl: format!("{}", game_state_impl),
        }
    }

    fn grid_game(
        input_responders: Vec<InputResponder>,
        game_state_impl: GameStateImpl,
        (gw, gh): (u8, u8),
    ) -> RenderedGame {
        let (w, h) = get_cell_dimensions(gw, gh);

        let update_and_render = format!(
            "
    use common::*;

    {}

    #[inline]
    pub fn update_and_render(framebuffer: &mut Framebuffer, state: &mut GameState, input: Input) {{
        for id in 0..GameState::ENTITY_COUNT {{
            if !state.entities[id].contains(Component::Animate) {{
                continue;
            }}

            let current_variety = state.varieties[id];
            if state.entities[id].contains(Component::PlayerControlled) {{
                if current_variety == state.player_controlling_variety {{
                    respond_to_input(state, input, id, current_variety);
                }}
            }} else {{
                let artifical_input = Input::default(); //TODO AI

                respond_to_input(state, artifical_input, id, current_variety);
            }}
        }}

        framebuffer.clear();

        for i in 0..GameState::ENTITY_COUNT {{
            let (x, y) = state.positions[i];
            let appearance = &mut state.appearances[i];

            render(appearance, framebuffer, (x as usize, y as usize), ({}, {}));
        }}
    }}
    ",
            InputResponders(input_responders),
            w,
            h,
        );

        RenderedGame {
            update_and_render,
            game_state_impl: format!("{}", game_state_impl),
        }
    }

    fn guess_game(
        input_responders: Vec<InputResponder>,
        game_state_impl: GameStateImpl,
    ) -> RenderedGame {
        let update_and_render = format!(
            "
    use common::*;

    {}

    #[inline]
    pub fn update_and_render(framebuffer: &mut Framebuffer, state: &mut GameState, input: Input) {{
        respond_to_input(state, input, 0, Variety::default());

        if state.has_won() {{
            draw_winning_screen(framebuffer);
        }}
    }}
    ",
            InputResponders(input_responders),
        );

        RenderedGame {
            update_and_render,
            game_state_impl: format!("{}", game_state_impl),
        }
    }
}

fn render_spec<R: Rng + Sized>(rng: &mut R, spec: GameSpec) -> Result<RenderedGame> {
    let result = match spec {
        GameSpec::Guess => render_guess_game(rng),
        GameSpec::Grid(ggs) => render_grid_game(rng, ggs),
        GameSpec::Solitaire(sgs) => render_solitaire_game(rng, sgs),
    };

    result.map(RenderableGame::render)
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

fn render_solitaire_game<R: Rng + Sized>(
    rng: &mut R,
    spec: SolitaireSpec,
) -> Result<RenderableGame> {
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
