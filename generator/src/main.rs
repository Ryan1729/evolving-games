use std::fmt;
use std::fs::OpenOptions;
use std::io::Write as IOWrite;
use std::path::Path;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

extern crate rand;
use rand::{
    distributions::{Distribution, Standard}, Rng, SeedableRng, XorShiftRng,
};

extern crate project_common;

#[macro_use]
mod common;
use common::*;

mod solitaire;
use solitaire::render_game as render_solitaire_game;

mod grid;
use grid::render_game as render_grid_game;

mod guess;
use guess::render_game as render_guess_game;

#[macro_use]
mod error;
use error::{Error, ErrorKind, Result};

fn main() {
    let seed: [u8; 16] = {
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
                result
            })
            .unwrap_or_else(|| {
                let since_the_epoch = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_else(|_| Duration::new(42, 42));

                let seconds: [u32; 2] = unsafe { std::mem::transmute(since_the_epoch.as_secs()) };

                let result = [seconds[0], seconds[1], seconds[0], seconds[1]];

                unsafe { std::mem::transmute(result) }
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

fn generate_game<R: Rng + ?Sized>(rng: &mut R) -> Result<RenderedGame> {
    generate_spec(rng).and_then(|spec: GameSpec| {
        println!("{:#?}", spec);
        //TODO Separate seed for these RNG calls?
        render_spec(rng, spec)
    })
}

impl Distribution<GameType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> GameType {
        match rng.gen_range(0, 4) {
            0 => ErrorTest,
            1 => Solitaire,
            2 => GridBased,
            _ => Guess,
        }
    }
}

fn generate_spec<R: Rng + ?Sized>(rng: &mut R) -> Result<GameSpec> {
    match rng.gen() {
        ErrorTest => err!(ErrorKind::NotImplemented),
        Solitaire => Ok(GameSpec::Solitaire(rng.gen())),
        GridBased => Ok(GameSpec::Grid(rng.gen())),
        Guess => Ok(Default::default()),
    }
}

impl Distribution<SolitaireSpec> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SolitaireSpec {
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

        SolitaireSpec {
            grid_dimensions,
            deck,
        }
    }
}

impl DeckType {
    fn get_minimum_card_count(&self) -> u8 {
        match *self {
            ThreeColour(highest_card) => highest_card.saturating_mul(3),
        }
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

fn render_spec<R: Rng + ?Sized>(rng: &mut R, spec: GameSpec) -> Result<RenderedGame> {
    let result = match spec {
        GameSpec::Guess => render_guess_game(rng),
        GameSpec::Grid(ggs) => render_grid_game(rng, ggs),
        GameSpec::Solitaire(sgs) => render_solitaire_game(rng, sgs),
    };

    result.map(RenderableGame::render)
}
