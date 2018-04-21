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

    let filename = "../player/src/game.rs";
    let path = Path::new(filename);
    println!("Overwriting {:?}.", path.as_os_str());

    let code = generate_update_function(&mut rng);

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

fn generate_update_function<R: Rng + Sized>(rng: &mut R) -> String {
    format!(
        "
use common::*;

fn add_one_to_buffer(buffer: &mut [u32], mut i: usize) {{
    loop {{
        buffer[i] = match buffer[i] {{
            BLUE => GREEN,
            GREEN => RED,
            RED => YELLOW,
            YELLOW => PURPLE,
            PURPLE => GREY,
            GREY => WHITE,
            WHITE => BLACK,
            BLACK => BLUE,
            other => other,
        }};

        if buffer[i] != BLACK || i == 0 {{
            break;
        }}

        i -= 1;
    }}
}}

fn add_n_to_buffer(buffer: &mut [u32], mut n: u32) {{
    let len = buffer.len();
    for i in (0..len).rev() {{
        for _ in 0..(n & 0b111) {{
            add_one_to_buffer(buffer, i);
        }}

        n >>= 3;

        if n == 0 {{
            break;
        }}
    }}
}}

fn nor(b1: bool, b2: bool) -> bool {{
    !(b1 || b2)
}}

#[inline]
pub fn update_and_render(state: &mut Framebuffer, input: Input) {{
    let buffer = &mut state.buffer;
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

    for y in 1..SCREEN_HEIGHT {{
        for x in 0..SCREEN_WIDTH {{
            buffer[y * SCREEN_WIDTH + x] = buffer[x];
        }}
    }}
}}
",
        gen_button_response(rng),
        gen_button_response(rng),
        gen_button_response(rng),
        gen_button_response(rng),
        gen_button_response(rng),
        gen_button_response(rng),
        gen_button_response(rng),
        gen_button_response(rng),
    )
}

const STATE_PREDICATE_LENGTH_UPPER_BOUND: usize =
    (SCREEN_WIDTH / 2) //Average amount of nodes pe expression
    * (
        5 //"nor()".len()
        + SECTION_PREDICATE_LENGTH_UPPER_BOUND
    )
    + 14 //ELSE_IF.len() - 6
    ;

macro_rules! IF {
    () => {
        "if {} {{\n\t{}\n}}"
    };
}

macro_rules! ELSE_IF {
    () => {
        "else if {} {{\n\t{}\n}}"
    };
}

macro_rules! ELSE {
    () => {
        "else {{\n\t{}\n}}\n"
    };
}

fn gen_button_response<R: Rng + Sized>(rng: &mut R) -> String {
    let pred_count = rng.gen_range(1, 4);

    let mut predicates = Vec::with_capacity(pred_count);

    for _ in 0..pred_count {
        predicates.push(generate_state_predicate(rng));
    }

    let mut result = String::with_capacity(
        pred_count
        * STATE_PREDICATE_LENGTH_UPPER_BOUND
        + 11  //ELSE.len() - 4
        + (pred_count + 1)
        * STATE_MUTATION_LENGTH_UPPER_BOUND,
    );

    write!(result, IF!(), predicates[0], generate_state_mutation(rng)).unwrap();

    for i in 1..pred_count {
        write!(
            result,
            ELSE_IF!(),
            predicates[i],
            generate_state_mutation(rng)
        ).unwrap();
    }

    write!(result, ELSE!(), generate_state_mutation(rng)).unwrap();

    result
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
            "//for i in ({}..).rev() {{
                let n = match buffer[{}] {{
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
