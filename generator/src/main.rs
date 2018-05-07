use std::fs::OpenOptions;
use std::path::Path;
use std::io::Write as IOWrite;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Write;

extern crate rand;
use rand::{Rng, SeedableRng, XorShiftRng};

extern crate project_common;
use project_common::*;

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

    println!("{:?}\n", rng.next_u32());

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

#[derive(Debug)]
struct Error {
    line: u32,
    file: String,
    kind: ErrorKind,
}

macro_rules! err {
    ($kind: expr) => {
        Err(Error {
            line: line!(),
            file: file!(),
            kind: $kind,
        })
    };
}

#[derive(Debug)]
enum ErrorKind {
    NotImplemented,
}
use ErrorKind::*;

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} error occurred at line {} in {}",
            self.kind, self.line, self.file
        )
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        "https://github.com/rust-lang/rfcs/pull/2230"
    }
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
enum GameSpec {
    Guess,
    Grid(GridGameSpec),
    Solitaire(()),
}
use GameSpec::*;

impl Default for GameSpec {
    fn default() -> GameSpec {
        Guess
    }
}

#[derive(Debug)]
struct GridGameSpec {
    grid_dimensions: (u8, u8),
    goal: Goal,
    entity_animacies: Vec<EntityAnimacy>,
    entity_controls: Vec<Option<EntityControl>>,
}

fn generate_game<R: Rng + Sized>(rng: &mut R) -> Result<RenderedGame> {
    generate_spec(rng).and_then(|spec: GameSpec| {
        println!("{:#?}", spec);
        //TODO Separate seed for these RNG calls?
        render_spec(rng, spec)
    })
}

enum GameType {
    Guess,
    GridBased,
    Solitaire,
}

fn generate_game_type<R: Rng + Sized>(rng: &mut R) -> GameType {
    match rng.gen_range(0, 3) {
        0 => GameType::Guess,
        1 => GameType::GridBased,
        _ => GameType::Solitaire,
    }
}

fn generate_spec<R: Rng + Sized>(rng: &mut R) -> Result<GameSpec> {
    let game_type = generate_game_type(rng);

    match game_type {
        GameType::Guess => Ok(Default::default()),
        GameType::GridBased => generate_grid_spec(rng).map(Grid),
        GameType::Solitaire => Ok(()).map(Solitaire),
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

#[derive(Clone, Copy, Debug)]
enum Goal {
    AddSomeNumberOfXsToTheGrid,
    MoveAllXsToALocation,
    FreeAllTrappedXs,
    TransformAllXsIntoYs,
    MakeAllXsTouchTheGroupofXs,
    MakeAllXsTouchAtLeastOneY,
}
use Goal::*;

impl Goal {
    fn minimum_entity_types_needed(&self) -> u8 {
        //We always need at least one player controlled entity
        match *self {
            AddSomeNumberOfXsToTheGrid => 1,
            MoveAllXsToALocation => 1,
            FreeAllTrappedXs => 1,
            //X _cannot_ be the same as Y
            TransformAllXsIntoYs => 2,
            MakeAllXsTouchTheGroupofXs => 1,
            //X _can_ be the same as Y
            MakeAllXsTouchAtLeastOneY => 2,
        }
    }
}
const GOAL_COUNT: u8 = 4;

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

#[derive(Copy, Clone, Debug)]
struct EntityControl {
    movement: MoveControl,
    a: Action,
    b: Action,
    select: Action,
}

#[derive(Copy, Clone, Debug)]
enum MoveControl {
    Orthogonal,
    Diagonal,
}
use MoveControl::*;

const MOVE_CONTROL_COUNT: u8 = 2;

fn generate_move_control<R: Rng + Sized>(rng: &mut R) -> MoveControl {
    match rng.gen_range(0, MOVE_CONTROL_COUNT) {
        0 => Orthogonal,
        _ => Diagonal,
    }
}

//Moreso than any other game element, this set of possibilities will need to be expaneded
#[derive(Copy, Clone, Debug)]
enum Action {
    SwapPlayerControlToNext(u8),
    CopySelf,
}
use Action::*;

const ACTION_COUNT: u8 = 2;

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

#[derive(Debug)]
enum EntityAnimacy {
    PlayerControlled,
    Inanimate,
    Animate,
}
use EntityAnimacy::*;

impl fmt::Display for EntityAnimacy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = match *self {
            PlayerControlled => "Component::Player",
            Inanimate => "Component::empty()",
            Animate => "Component::Animate",
        };

        write!(f, "{}", result,)?;

        Ok(())
    }
}

const ENTITY_ANIMACY_COUNT: u8 = 3;

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

const BUTTON_COUNT: usize = 8;

#[derive(Default)]
struct RenderableGame {
    input_responders: Vec<InputResponder>,
    initial_state: InitialState,
    grid_dimensions: Option<(u8, u8)>,
}

#[derive(Default)]
struct InitialState {
    animacies: Vec<EntityAnimacy>,
    positions: Vec<(u8, u8)>,
    appearances: Vec<u8>,
    varieties: Vec<u8>,
}

impl fmt::Display for InitialState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let InitialState {
            ref animacies,
            ref positions,
            ref appearances,
            ref varieties,
        } = *self;

        debug_assert!(
            animacies.len() == positions.len() && positions.len() == appearances.len()
                && appearances.len() == varieties.len()
        );

        for i in 0..positions.len() {
            write!(
                f,
                "    entities[{}] = {};
    positions[{0}] = ({}, {});
    appearances[{0}] = Appearance({});
    varieties[{0}] = {};
",
                i, animacies[i], positions[i].0, positions[i].1, appearances[i], varieties[i],
            )?;
        }

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
            input_responders,
            initial_state,
            grid_dimensions,
        } = self;

        match grid_dimensions {
            Some(gd) => RenderableGame::grid_game(input_responders, initial_state, gd),
            None => RenderableGame::guess_game(input_responders, initial_state),
        }
    }

    fn guess_game(
        input_responders: Vec<InputResponder>,
        initial_state: InitialState,
    ) -> RenderedGame {
        let update_and_render = format!(
            "
    use common::*;

    {}

    #[inline]
    pub fn update_and_render(framebuffer: &mut Framebuffer, state: &mut GameState, input: Input) {{
        respond_to_input(state, input, 0, Variety::default());
    }}
    ",
            InputResponders(input_responders),
        );

        let game_state_impl = "use inner_common::*;

    impl GameState {{
        pub const ENTITY_COUNT: usize = 256;
        pub const GRID_DIMENSIONS: (u8, u8) = ({}, {});

        pub fn new() -> GameState {{
            let mut entities = [Component::Ty::empty(); GameState::ENTITY_COUNT];

            let mut positions = [(0, 0); GameState::ENTITY_COUNT];
            let mut appearances = [Appearance::default(); GameState::ENTITY_COUNT];
            let mut varieties = [Variety::default(); GameState::ENTITY_COUNT];

            let player_controlling_variety = Variety::default();

            GameState {{
                entities,
                positions,
                appearances,
                varieties,
                player_controlling_variety,
            }}
        }}
    }}
"
            .to_string();

        RenderedGame {
            update_and_render,
            game_state_impl,
        }
    }

    fn grid_game(
        input_responders: Vec<InputResponder>,
        initial_state: InitialState,
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

            appearance.render(framebuffer, (x as usize, y as usize), ({}, {}));
        }}
    }}
    ",
            InputResponders(input_responders),
            w,
            h,
        );

        let game_state_impl = format!(
            "use inner_common::*;

        impl GameState {{
            pub const ENTITY_COUNT: usize = 256;
            pub const GRID_DIMENSIONS: (u8, u8) = ({}, {});

            pub fn new() -> GameState {{
                let mut entities = [Component::Ty::empty(); GameState::ENTITY_COUNT];

                let mut positions = [(0, 0); GameState::ENTITY_COUNT];
                let mut appearances = [Appearance::default(); GameState::ENTITY_COUNT];
                let mut varieties = [Variety::default(); GameState::ENTITY_COUNT];

                let player_controlling_variety = Variety::default();

                {}

                GameState {{
                    entities,
                    positions,
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

            pub fn find_nearest_empty_pos(
                &self,
                start_pos: Position,
            ) -> Option<Position> {{
                use std::collections::{{VecDeque, HashSet}};
                //PERF would it be faster to preallocate this?
                //I expect the common case not to use anything close to the maximum.
                let mut queue = VecDeque::new();

                let mut full = HashSet::with_capacity(GameState::ENTITY_COUNT);
                let mut visited = HashSet::with_capacity(GameState::ENTITY_COUNT);

                //PERF it might make sense to just keep track of which slots are free all the time
                for i in 0..GameState::ENTITY_COUNT {{
                    if !self.entities[i].is_empty() {{
                        full.insert(self.positions[i]);
                    }}
                }}

                queue.push_back(start_pos);

                while let Some(pos) = queue.pop_front() {{
                    if !full.contains(&pos) {{
                        return Some(pos)
                    }}

                    if visited.contains(&pos) {{
                        continue;
                    }}

                    visited.insert(pos);

                    //TODO we might want to figure out a heuristic on which direction to look in
                    //first, given the start_pos. It would also prevent the empty ones always
                    //being in a certain direction.

                    if pos.0 > 0 {{
                        queue.push_back((pos.0 - 1, pos.1));
                    }}

                    if pos.0 < GameState::GRID_DIMENSIONS.0 - 1 {{
                        queue.push_back((pos.0 + 1, pos.1));
                    }}

                    if pos.1 > 0 {{
                        queue.push_back((pos.0, pos.1 - 1));
                    }}

                    if pos.1 < GameState::GRID_DIMENSIONS.1 - 1 {{
                        queue.push_back((pos.0, pos.1 + 1));
                    }}
                }}

                None
            }}
        }}
",
            gw, gh, initial_state
        );

        RenderedGame {
            update_and_render,
            game_state_impl,
        }
    }
}

#[derive(Default)]
struct InputResponders(Vec<InputResponder>);

impl fmt::Display for InputResponders {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for input_responder in self.0.iter() {
            write!(f, "{}\n", input_responder)?;
        }

        write!(
            f,
"fn respond_to_input(state: &mut GameState, input: Input, id: usize, variety: Variety) {{
    match variety {{\n"
        )?;

        let mut varieties: Vec<_> = self.0.iter().map(|ir| ir.variety).collect();

        varieties.sort();
        varieties.dedup();

        for variety in varieties.iter() {
            write!(
                f,
                "           {0} => input_responder_{}(state, input, id),\n",
                variety
            )?;
        }

        write!(
            f,
            "           _ => {{}},
        }}
    }}\n"
        )?;

        Ok(())
    }
}

struct InputResponder {
    button_responses: ButtonResponses,
    variety: Variety,
}

impl fmt::Display for InputResponder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let InputResponder {
            ref button_responses,
            ref variety,
        } = *self;

        write!(
            f,
            "fn input_responder_{}(state: &mut GameState, input: Input, id: usize) {{
                if input.pressed_this_frame(Button::Left) {{
                    {}
                }}

                if input.pressed_this_frame(Button::Right) {{
                    {}
                }}

                if input.pressed_this_frame(Button::Up) {{
                    {}
                }}

                if input.pressed_this_frame(Button::Down) {{
                    {}
                }}

                if input.pressed_this_frame(Button::Select) {{
                    {}
                }}

                if input.pressed_this_frame(Button::Start) {{
                    {}
                }}

                if input.pressed_this_frame(Button::A) {{
                    {}
                }}

                if input.pressed_this_frame(Button::B) {{
                    {}
                }}
            }}
        ",
            variety,
            button_responses.left,
            button_responses.right,
            button_responses.up,
            button_responses.down,
            button_responses.select,
            button_responses.start,
            button_responses.a,
            button_responses.b,
        )?;

        Ok(())
    }
}

fn render_spec<R: Rng + Sized>(rng: &mut R, spec: GameSpec) -> Result<RenderedGame> {
    let result = match spec {
        Guess => render_guess_game(rng),
        Grid(ggs) => render_grid_game(rng, ggs),
        Solitaire(sgs) => render_guess_game(rng), //TODO render_solitaire_game
    };

    result.map(RenderableGame::render)
}

fn render_guess_game<R: Rng + Sized>(rng: &mut R) -> Result<RenderableGame> {
    let mut button_responses = ButtonResponses::default();

    let winning_index = rng.gen_range(0, BUTTON_COUNT);

    let winner = "draw_winning_screen(state);".to_string();

    match winning_index {
        0 => {
            button_responses.up = winner;
        }
        1 => {
            button_responses.down = winner;
        }
        2 => {
            button_responses.left = winner;
        }
        3 => {
            button_responses.right = winner;
        }
        4 => {
            button_responses.a = winner;
        }
        5 => {
            button_responses.b = winner;
        }
        6 => {
            button_responses.start = winner;
        }
        7 => {
            button_responses.select = winner;
        }
        _ => {}
    }

    let responder = InputResponder {
        button_responses,
        variety: Default::default(),
    };

    Ok(RenderableGame {
        input_responders: vec![responder],
        initial_state: Default::default(),
        grid_dimensions: Default::default(),
    })
}

fn render_grid_game<R: Rng + Sized>(rng: &mut R, spec: GridGameSpec) -> Result<RenderableGame> {
    let grid_cell_size = (
        next_power_of_2(spec.grid_dimensions.0 as _),
        next_power_of_2(spec.grid_dimensions.1 as _),
    );

    debug_assert!(spec.entity_animacies.len() == spec.entity_controls.len());

    let entity_type_count = spec.entity_animacies.len();

    let mut appearances = Vec::with_capacity(entity_type_count);
    let mut positions = Vec::with_capacity(entity_type_count);
    let mut varieties = Vec::with_capacity(entity_type_count);

    let mut input_responders = Vec::with_capacity(entity_type_count);

    for i in 0..entity_type_count {
        appearances.push(rng.gen());

        positions.push((
            rng.gen_range(0, spec.grid_dimensions.0),
            rng.gen_range(0, spec.grid_dimensions.1),
        ));

        varieties.push(i as Variety);

        input_responders.push(InputResponder {
            button_responses: spec.entity_controls[i]
                .map(controls_to_button_responses)
                .unwrap_or_else(|| Default::default()),
            variety: i as Variety,
        });
    }

    let initial_state = InitialState {
        animacies: spec.entity_animacies,
        positions,
        appearances,
        varieties,
    };

    Ok(RenderableGame {
        input_responders,
        initial_state,
        grid_dimensions: Some(spec.grid_dimensions),
    })
}

#[derive(Default)]
struct ButtonResponses {
    up: String,
    down: String,
    left: String,
    right: String,
    a: String,
    b: String,
    start: String,
    select: String,
}

fn controls_to_button_responses(controls: EntityControl) -> ButtonResponses {
    let up;
    let down;
    let left;
    let right;

    match controls.movement {
        Orthogonal => {
            up = "let pos = &mut state.positions[id];
pos.1 = pos.1.saturating_sub(1);\n"
                .to_string();
            down = "let pos = &mut state.positions[id];
pos.1 = pos.1.saturating_add(1);\n"
                .to_string();
            left = "let pos = &mut state.positions[id];
pos.0 = pos.0.saturating_sub(1);\n"
                .to_string();
            right = "let pos = &mut state.positions[id];
pos.0 = pos.0.saturating_add(1);\n"
                .to_string();
        }
        Diagonal => {
            up = "let pos = &mut state.positions[id];
pos.0 = pos.0.saturating_add(1);
pos.1 = pos.1.saturating_sub(1);\n"
                .to_string();
            down = "let pos = &mut state.positions[id];
pos.0 = pos.0.saturating_sub(1);
pos.1 = pos.1.saturating_add(1);\n"
                .to_string();
            left = "let pos = &mut state.positions[id];
pos.0 = pos.0.saturating_sub(1);
pos.1 = pos.1.saturating_sub(1);\n"
                .to_string();
            right = "let pos = &mut state.positions[id];
pos.0 = pos.0.saturating_add(1);
pos.1 = pos.1.saturating_add(1);\n"
                .to_string();
        }
    }

    let a = action_to_button_responses(controls.a);
    let b = action_to_button_responses(controls.b);
    let select = action_to_button_responses(controls.select);

    let start = "".to_string();

    ButtonResponses {
        up,
        down,
        left,
        right,
        a,
        b,
        start,
        select,
    }
}

fn action_to_button_responses(action: Action) -> String {
    match action {
        SwapPlayerControlToNext(offset) => format!(
            "state.player_controlling_variety = state.player_controlling_variety.wrapping_add({});",
            offset
        ),
        CopySelf => "if let Some(clone_id) = state.get_free_id() {
            if let Some(clone_pos) = state.find_nearest_empty_pos(state.positions[id]) {
                state.positions[clone_id] = clone_pos;
                state.entities[clone_id] = state.entities[id];
                state.appearances[clone_id] = state.appearances[id];
                state.varieties[clone_id] = state.varieties[id];
            }
        }
"
            .to_string(),
    }
}
const STATE_PREDICATE_LENGTH_UPPER_BOUND: usize =
    (SCREEN_WIDTH / 2) //Average amount of nodes pe expression
    * (
        5 //"nor()".len()
        + SECTION_PREDICATE_LENGTH_UPPER_BOUND
    )
    + 14 //ELSE_IF.len() - 6
    ;

#[allow(unused_macros)]
macro_rules! IF {
    () => {
        "if {} {{\n\t{}\n}}"
    };
}

#[allow(unused_macros)]
macro_rules! ELSE_IF {
    () => {
        "else if {} {{\n\t{}\n}}"
    };
}

#[allow(unused_macros)]
macro_rules! ELSE {
    () => {
        "else {{\n\t{}\n}}\n"
    };
}

//TODO could be tighter
const STATE_MUTATION_LENGTH_UPPER_BOUND: usize = SCREEN_WIDTH * 512;

struct Mutation {
    pub start: usize,
    pub one_past_end: usize,
    pub transform: PixelTransform,
}

impl fmt::Display for Mutation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Mutation {
            start,
            one_past_end,
            transform,
        } = *self;

        write!(
            f,
            "for i in ({}..{}).rev() {{
                let n = match buffer[i] {{
                BLUE => {},
                GREEN => {},
                RED => {},
                YELLOW => {},
                PURPLE => {},
                GREY => {},
                WHITE => {},
                BLACK => {},
                _ => 0
            }};

            add_n_to_buffer(&mut buffer[{0}..{1}], n);
        //}}
        ",
            start,
            one_past_end,
            u32::from(transform[usize::from(Blue)]),
            u32::from(transform[usize::from(Green)]),
            u32::from(transform[usize::from(Red)]),
            u32::from(transform[usize::from(Yellow)]),
            u32::from(transform[usize::from(Purple)]),
            u32::from(transform[usize::from(Grey)]),
            u32::from(transform[usize::from(White)]),
            u32::from(transform[usize::from(Black)]),
        )?;

        Ok(())
    }
}

const MAXIMUM_U32_CHAR_LENGTH: usize = 10;

const COLOUR_COUNT: usize = 8;

type PixelTransform = [u32; COLOUR_COUNT];

fn generate_state_mutation<R: Rng + Sized>(rng: &mut R) -> Mutation {
    let start = rng.gen_range(0, SCREEN_WIDTH);
    let one_past_end = rng.gen_range(start, SCREEN_WIDTH);

    Mutation {
        start,
        one_past_end,
        transform: generate_pixel_transform(rng),
    }
}

fn generate_pixel_transform<R: Rng + Sized>(rng: &mut R) -> PixelTransform {
    let mut result = [0; COLOUR_COUNT];

    for i in 0..COLOUR_COUNT {
        result[i] = rng.gen_range(0, SCREEN_WIDTH as u32);
    }

    result
}

struct StatePredicate(Vec<Option<SectionPredicate>>);

fn generate_state_predicate<R: Rng + Sized>(rng: &mut R) -> StatePredicate {
    let section_predicates = generate_section_predicates(rng);

    // Insert up to twice the current nodes worth of blanks,
    // so all possible combinations are possible
    let max_nodes = next_power_of_2(section_predicates.len()) * 2;

    let mut result = vec![None; max_nodes];

    debug_assert!(section_predicates.len() <= max_nodes);

    for s in section_predicates.into_iter() {
        let mut index = rng.gen_range(0, max_nodes);

        loop {
            if result[index].is_none() {
                result[index] = Some(s);
                break;
            }

            index += 1;
            index &= max_nodes - 1;
        }
    }

    StatePredicate(result)
}

impl fmt::Display for StatePredicate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut strings: Vec<String> = Vec::with_capacity(self.0.len() / 2);
        for nodes in self.0.chunks(2) {
            //TODO evaluate whether the default to false here skews things.
            strings.push(match (nodes[0], nodes[1]) {
                (Some(p1), Some(p2)) => format!("nor({}, {})", p1, p2),
                (Some(p), None) | (None, Some(p)) => format!("{}", p),
                (None, None) => "false".to_string(),
            });
        }

        while strings.len() > 1 {
            let new_len = strings.len() / 2;
            for i in 0..new_len {
                strings[i] = format!("nor({}, {})", strings[2 * i], strings[2 * i + 1]);
            }

            unsafe { strings.set_len(new_len) };
        }

        let result = strings.pop();

        match result {
            Some(p) => write!(f, "{}", p).unwrap(),
            None => write!(f, "false").unwrap(),
        };

        Ok(())
    }
}

// https://graphics.stanford.edu/~seander/bithacks.html#RoundUpPowerOf2
fn next_power_of_2(mut x: usize) -> usize {
    //The basic idea here is fill in all the bits below the highest set bit
    //and then add one, making a power of two. We do this by taking the
    //highest set bit, (the only one we know we have) and progressively
    //ORing with the lower bits of the number. Once we do the first OR,
    //then we know there are two set bits at the top, so we can set the
    //next two below it at once. Then we know the top 4 are set and so on.
    x = x.wrapping_sub(1); //This subtraction makes, for instance, 8 map to 8 instead of 16.
    x |= x >> 1;
    x |= x >> 2;
    x |= x >> 4;
    x |= x >> 8;
    x |= x >> 16;
    x |= x >> 32;
    x.wrapping_add(1)
}

#[derive(Copy, Clone)]
struct SectionPredicate(usize, [bool; COLOUR_COUNT]);

const SECTION_PREDICATE_LENGTH_UPPER_BOUND: usize =
    15 // "match buffer[{}] {{ ".len() - 4
    + MAXIMUM_U32_CHAR_LENGTH
    + 5 //"false".len()
    + 6 // "{} => {}, ".len() - 4
    + 13//"_ => false, }}".len() - 1
    ;
impl fmt::Display for SectionPredicate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let &SectionPredicate(index, results) = self;
        write!(f, "match buffer[{}] {{ ", index)?;

        for i in 0..COLOUR_COUNT {
            write!(
                f,
                "{} => {}, ",
                u32::from(Colour::from(i)),
                //Theoretically Display for bools could change
                if results[i] { "true" } else { "false" }
            )?;
        }

        write!(f, "_ => false, }}")?;

        Ok(())
    }
}

fn generate_section_predicates<R: Rng + Sized>(rng: &mut R) -> Vec<SectionPredicate> {
    let state_subset_size = rng.gen_range(0, SCREEN_WIDTH / 8);
    //we don't want duplicates.
    let mut map = HashMap::with_capacity(state_subset_size);

    //Apparently due to a Robert Floyd.
    //see https://stackoverflow.com/a/2394292/4496839
    for j in 1..state_subset_size {
        let current_index = rng.gen_range(0, j);
        if map.contains_key(&current_index) {
            map.insert(j, generate_section_predicate(rng, j));
        } else {
            map.insert(
                current_index,
                generate_section_predicate(rng, current_index),
            );
        }
    }

    let mut result: Vec<SectionPredicate> = map.drain().map(|(_, p)| p).collect();
    //We want a consistent order so the result is determined only by the RNG.
    result.sort_by(|&SectionPredicate(i1, _), &SectionPredicate(i2, _)| i1.cmp(&i2));

    //But we don't want the order to be the same every time.
    rng.shuffle(&mut result);

    result
}

fn generate_section_predicate<R: Rng + Sized>(rng: &mut R, state_index: usize) -> SectionPredicate {
    SectionPredicate(
        state_index,
        [
            rng.gen(),
            rng.gen(),
            rng.gen(),
            rng.gen(),
            rng.gen(),
            rng.gen(),
            rng.gen(),
            rng.gen(),
        ],
    )
}
