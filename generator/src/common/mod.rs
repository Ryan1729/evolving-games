use std::fmt;

use rand::{
    distributions::{Distribution, Standard}, Rng,
};

pub use project_common::*;

macro_rules! code_string {
    ($($token_trees:tt)*) => {
        stringify!($($token_trees)*).to_owned()
    }
}

#[derive(Debug)]
pub enum GameSpec {
    Solitaire(SolitaireSpec),
    Grid(GridGameSpec),
    Guess,
}

impl Default for GameSpec {
    fn default() -> GameSpec {
        GameSpec::Guess
    }
}

#[derive(Debug)]
pub struct SolitaireSpec {
    pub grid_dimensions: (u8, u8),
    pub deck: DeckType,
}

#[derive(Debug)]
pub enum DeckType {
    ThreeColour(u8),
}
pub use DeckType::*;

#[derive(Default)]
pub struct ButtonResponses {
    pub up: String,
    pub down: String,
    pub left: String,
    pub right: String,
    pub a: String,
    pub b: String,
    pub start: String,
    pub select: String,
}

pub const BUTTON_COUNT: usize = 8;

#[derive(Debug)]
pub enum GameType {
    ErrorTest,
    Solitaire,
    GridBased,
    Guess,
}
pub use GameType::*;

impl Default for GameType {
    fn default() -> GameType {
        Guess
    }
}

#[derive(Default)]
pub struct RenderableGame {
    pub game_type: GameType,
    pub input_responders: Vec<InputResponder>,
    pub grid_dimensions: (u8, u8),
    pub game_state_impl: GameStateImpl,
}

#[derive(Default)]
pub struct InitialState {
    pub animacies: Vec<EntityAnimacy>,
    pub positions: Vec<Vec<(u8, u8)>>,
    pub sizes: Vec<Vec<(u8, u8)>>,
    pub appearances: Vec<Vec<Appearance>>,
    pub varieties: Vec<u8>,
}

#[derive(Default)]
pub struct GameStateImpl {
    pub entity_count: usize,
    pub entity_piece_count: usize,
    pub custom_consts: String,
    pub initial_state: InitialState,
    pub custom_code: String,
}

#[derive(Default)]
pub struct InputResponders(pub Vec<InputResponder>);

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

#[derive(Default)]
pub struct InputResponder {
    pub button_responses: ButtonResponses,
    pub variety: Variety,
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

impl Distribution<GridGameSpec> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> GridGameSpec {
        let grid_dimensions = (
            rng.gen_range(0, SCREEN_WIDTH) as u8,
            rng.gen_range(0, SCREEN_HEIGHT) as u8,
        );

        let goal = rng.gen();

        let entity_animacies = generate_entity_animacies(rng, goal);

        let entity_controls = generate_entity_controls(rng, &entity_animacies);

        GridGameSpec {
            grid_dimensions,
            goal,
            entity_animacies,
            entity_controls,
        }
    }
}

impl Distribution<Goal> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Goal {
        match rng.gen_range(0, GOAL_COUNT) {
            0 => AddSomeNumberOfXsToTheGrid,
            1 => MoveAllXsToALocation,
            2 => FreeAllTrappedXs,
            3 => TransformAllXsIntoYs,
            4 => MakeAllXsTouchTheGroupofXs,
            _ => MakeAllXsTouchAtLeastOneY,
        }
    }
}

fn generate_entity_animacies<R: Rng + ?Sized>(rng: &mut R, goal: Goal) -> Vec<EntityAnimacy> {
    let minimum_entity_types_needed = goal.minimum_entity_types_needed();

    let entity_animacies_len =
        rng.gen_range(minimum_entity_types_needed, minimum_entity_types_needed * 2);

    debug_assert!(entity_animacies_len >= 1);

    let mut entity_animacies = Vec::with_capacity(entity_animacies_len as _);

    entity_animacies.push(PlayerControlled);

    for _ in 1..entity_animacies_len {
        entity_animacies.push(rng.gen());
    }

    entity_animacies
}

impl Distribution<EntityAnimacy> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EntityAnimacy {
        match rng.gen_range(0, ENTITY_ANIMACY_COUNT) {
            0 => PlayerControlled,
            1 => Inanimate,
            _ => Animate,
        }
    }
}

#[derive(Debug)]
pub struct GridGameSpec {
    pub grid_dimensions: (u8, u8),
    pub goal: Goal,
    pub entity_animacies: Vec<EntityAnimacy>,
    pub entity_controls: Vec<Option<EntityControl>>,
}

#[derive(Clone, Copy, Debug)]
pub enum Goal {
    AddSomeNumberOfXsToTheGrid,
    MoveAllXsToALocation,
    FreeAllTrappedXs,
    TransformAllXsIntoYs,
    MakeAllXsTouchTheGroupofXs,
    MakeAllXsTouchAtLeastOneY,
}
pub use Goal::*;

pub const GOAL_COUNT: u8 = 4;

impl Goal {
    pub fn minimum_entity_types_needed(&self) -> u8 {
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

#[derive(Debug)]
pub enum EntityAnimacy {
    PlayerControlled,
    Inanimate,
    Animate,
}
pub use EntityAnimacy::*;

pub const ENTITY_ANIMACY_COUNT: u8 = 3;

impl Distribution<EntityControl> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EntityControl {
        EntityControl {
            movement: rng.gen(),
            a: rng.gen(),
            b: rng.gen(),
            select: rng.gen(),
        }
    }
}

impl Default for EntityAnimacy {
    fn default() -> EntityAnimacy {
        PlayerControlled
    }
}

impl fmt::Display for EntityAnimacy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = match *self {
            PlayerControlled => "Component::Player",
            Inanimate => "Component::Ty::empty()",
            Animate => "Component::Animate",
        };

        write!(f, "{}", result,)?;

        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
pub struct EntityControl {
    pub movement: MoveControl,
    pub a: Action,
    pub b: Action,
    pub select: Action,
}

#[derive(Copy, Clone, Debug)]
pub enum MoveControl {
    Orthogonal,
    Diagonal,
}
pub use MoveControl::*;

pub const MOVE_CONTROL_COUNT: u8 = 2;

impl Distribution<MoveControl> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MoveControl {
        match rng.gen_range(0, MOVE_CONTROL_COUNT) {
            0 => Orthogonal,
            _ => Diagonal,
        }
    }
}

//Moreso than any other game element, this set of possibilities will need to be expaneded
#[derive(Copy, Clone, Debug)]
pub enum Action {
    SwapPlayerControlToNext(u8),
    CopySelf,
}
pub use Action::*;

impl Distribution<Action> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Action {
        match rng.gen_range(0, MOVE_CONTROL_COUNT) {
            0 => SwapPlayerControlToNext(rng.gen()),
            _ => CopySelf,
        }
    }
}

fn generate_entity_controls<R: Rng + ?Sized>(
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
            _ => Some(rng.gen()),
        };

        controls.push(contol);
    }

    controls
}
