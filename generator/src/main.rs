use std::fs::OpenOptions;
use std::path::Path;
use std::io::Write as IOWrite;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Write;

extern crate rand;
use rand::{Rng, SeedableRng, XorShiftRng};

//TODO dedup these constants with player crate
//in pixels
pub const SCREEN_WIDTH: usize = 256;
pub const SCREEN_HEIGHT: usize = 240;
pub const SCREEN_LENGTH: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

// reportedly colourblind friendly colours
// https://twitter.com/ea_accessible/status/968595073184092160
pub const BLUE: u32 = 0xFFE15233;
pub const GREEN: u32 = 0xFF6EB030;
pub const RED: u32 = 0xFF4949DE;
pub const YELLOW: u32 = 0xFF37B9FF;
pub const PURPLE: u32 = 0xFF543353;
pub const GREY: u32 = 0xFF8B7D5A;
pub const GRAY: u32 = GREY;
pub const WHITE: u32 = 0xFFEEEEEE;
pub const BLACK: u32 = 0xFF222222;

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

    let code = match generate_update_function(&mut rng) {
        Err(error) => {
            println!("{}", error);
            return;
        }
        Ok(c) => c,
    };

    let filename = "../player/src/game.rs";
    let path = Path::new(filename);
    println!("Overwriting {:?}.", path.as_os_str());

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)
        .unwrap();

    let bytes = code.as_bytes();

    if let Err(error) = IOWrite::write_all(&mut file, bytes) {
        println!("{}", error);
    } else {
        let len = bytes.len();
        println!("Overwrote {} successfully with {} bytes.", filename, len);
    }
}

#[derive(Debug)]
struct Error {
    line: u32,
    file: &'static str,
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
struct GameSpec {
    grid_dimensions: Option<(u8, u8)>,
    goal: Goal,
    ontology: Ontology,
}

fn generate_update_function<R: Rng + Sized>(rng: &mut R) -> Result<String> {
    generate_spec(rng).and_then(|spec: GameSpec| {
        println!("{:?}", spec);
        //TODO Separate seed for these RNG calls?
        render_spec(rng, spec)
    })
}

fn generate_spec<R: Rng + Sized>(rng: &mut R) -> Result<GameSpec> {
    let grid_dimensions = (
        rng.gen_range(0, SCREEN_WIDTH) as u8,
        rng.gen_range(0, SCREEN_HEIGHT) as u8,
    );

    let goal = generate_goal(rng);

    let ontology = generate_ontology(rng, goal);

    Ok(GameSpec {
        grid_dimensions: Some(grid_dimensions),
        goal,
        ontology,
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

fn generate_goal<R: Rng + Sized>(rng: &mut R) -> Goal {
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
    entity_move_kinds: Vec<EntityMoveKind>,
}

fn generate_ontology<R: Rng + Sized>(rng: &mut R, goal: Goal) -> Ontology {
    let minimum_entity_types_needed = goal.minimum_entity_types_needed();

    let entity_move_kinds_len =
        rng.gen_range(minimum_entity_types_needed, minimum_entity_types_needed * 2);

    debug_assert!(entity_move_kinds_len >= 1);

    let mut entity_move_kinds = Vec::with_capacity(entity_move_kinds_len as _);

    entity_move_kinds.push(PlayerControlled);

    for _ in 1..entity_move_kinds_len {
        entity_move_kinds.push(generate_entity_move_kind(rng));
    }

    Ontology { entity_move_kinds }
}

#[derive(Debug)]
enum EntityMoveKind {
    PlayerControlled,
    Stationary,
    Mobile,
}
use EntityMoveKind::*;

const ENTITY_MOVE_KIND_COUNT: u8 = 3;

fn generate_entity_move_kind<R: Rng + Sized>(rng: &mut R) -> EntityMoveKind {
    match rng.gen_range(0, ENTITY_MOVE_KIND_COUNT) {
        0 => PlayerControlled,
        1 => Stationary,
        _ => Mobile,
    }
}

const BUTTON_COUNT: usize = 8;

struct RenderableGame {
    button_responses: [&'static str; BUTTON_COUNT],
}

impl RenderableGame {
    fn render(self) -> String {
        let RenderableGame { button_responses } = self;

        format!(
            "
    use common::*;

    #[inline]
    pub fn update_and_render(state: &mut Framebuffer, input: Input) {{
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
            button_responses[0],
            button_responses[1],
            button_responses[2],
            button_responses[3],
            button_responses[4],
            button_responses[5],
            button_responses[6],
            button_responses[7],
        )
    }
}

fn render_spec<R: Rng + Sized>(rng: &mut R, spec: GameSpec) -> Result<String> {
    let result = if let Some(grid_dimensions) = spec.grid_dimensions {
        render_grid_game(rng, spec, grid_dimensions)
    } else {
        render_guess_game(rng)
    };

    result.map(RenderableGame::render)
}

fn render_guess_game<R: Rng + Sized>(rng: &mut R) -> Result<RenderableGame> {
    let mut button_responses = [""; BUTTON_COUNT];

    let winning_index = rng.gen_range(0, BUTTON_COUNT);

    button_responses[winning_index] = "draw_winning_screen(state);";

    Ok(RenderableGame { button_responses })
}

fn render_grid_game<R: Rng + Sized>(
    _rng: &mut R,
    _spec: GameSpec,
    _grid_dimensions: (u8, u8),
) -> Result<RenderableGame> {
    err!(NotImplemented)
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
        }}
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

#[derive(Clone, Copy)]
enum Colour {
    Blue,
    Green,
    Red,
    Yellow,
    Purple,
    Grey,
    White,
    Black,
}
use Colour::*;

impl From<Colour> for u32 {
    fn from(c: Colour) -> Self {
        match c {
            Blue => BLUE,
            Green => GREEN,
            Red => RED,
            Yellow => YELLOW,
            Purple => PURPLE,
            Grey => GREY,
            White => WHITE,
            Black => BLACK,
        }
    }
}

impl From<u32> for Colour {
    fn from(n: u32) -> Self {
        match n {
            BLUE => Blue,
            GREEN => Green,
            RED => Red,
            YELLOW => Yellow,
            PURPLE => Purple,
            GREY => Grey,
            WHITE => White,
            BLACK => Black,
            _ => Grey,
        }
    }
}

impl From<Colour> for usize {
    fn from(c: Colour) -> Self {
        match c {
            Blue => 0,
            Green => 1,
            Red => 2,
            Yellow => 3,
            Purple => 4,
            Grey => 5,
            White => 6,
            Black => 7,
        }
    }
}

impl From<usize> for Colour {
    fn from(n: usize) -> Self {
        match n {
            0 => Blue,
            1 => Green,
            2 => Red,
            3 => Yellow,
            4 => Purple,
            5 => Grey,
            6 => White,
            7 => Black,
            _ => Grey,
        }
    }
}

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
