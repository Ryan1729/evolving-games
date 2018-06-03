use std::fmt;

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
