use std::error;
use std::fmt;
use std::result;

pub use project_common::*;

macro_rules! code_string {
    ($($token_trees:tt)*) => {
        stringify!($($token_trees)*).to_owned()
    }
}

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    pub line: u32,
    pub file: &'static str,
    pub kind: ErrorKind,
}

macro_rules! err {
    ($kind:expr) => {
        Err(Error {
            line: line!(),
            file: file!(),
            kind: $kind,
        })
    };
}

#[derive(Debug)]
pub enum ErrorKind {
    NotImplemented,
}
pub use ErrorKind::*;

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

impl error::Error for Error {
    fn description(&self) -> &str {
        "https://github.com/rust-lang/rfcs/pull/2230"
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

pub const ENTITY_ANIMACY_COUNT: u8 = 3;

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

//Moreso than any other game element, this set of possibilities will need to be expaneded
#[derive(Copy, Clone, Debug)]
pub enum Action {
    SwapPlayerControlToNext(u8),
    CopySelf,
}
pub use Action::*;

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
