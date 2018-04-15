use std::fs::OpenOptions;
use std::path::Path;
use std::io::Write;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::collections::HashMap;

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

    if let Err(error) = file.write_all(bytes) {
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
        generate_state_mutation(rng),
        generate_state_mutation(rng),
        generate_state_mutation(rng),
        generate_state_mutation(rng),
        generate_state_mutation(rng),
        generate_state_mutation(rng),
        generate_state_mutation(rng),
        generate_state_mutation(rng),
    )
}

use std::fmt;

type FramebufferIndex = usize;

struct Mutation {
    pub map: HashMap<FramebufferIndex, PixelTransform>,
}

impl fmt::Display for Mutation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (&fi, &pt) in self.map.iter() {
            write!(f, "{}\n", MutationEntry(fi, pt))?;
        }

        Ok(())
    }
}

struct MutationEntry(FramebufferIndex, PixelTransform);

impl fmt::Display for MutationEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let MutationEntry(index, transform) = *self;

        write!(
            f,
            "buffer[{}] = match buffer[{}] {{
            BLUE => {},
            GREEN => {},
            RED => {},
            YELLOW => {},
            PURPLE => {},
            GREY => {},
            WHITE => {},
            BLACK => {},
            other => other
        }};",
            index,
            index,
            u32::from(transform[usize::from(Blue)]),
            u32::from(transform[usize::from(Green)]),
            u32::from(transform[usize::from(Red)]),
            u32::from(transform[usize::from(Yellow)]),
            u32::from(transform[usize::from(Purple)]),
            u32::from(transform[usize::from(Grey)]),
            u32::from(transform[usize::from(White)]),
            u32::from(transform[usize::from(Black)]),
        )
    }
}

const COLOUR_COUNT: usize = 8;

type PixelTransform = [Colour; COLOUR_COUNT];

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
    let state_subset_size = rng.gen_range(0, SCREEN_HEIGHT);
    let mut map = HashMap::with_capacity(state_subset_size);

    //Apparently due to a Robert Floyd.
    //see https://stackoverflow.com/a/2394292/4496839
    for j in 1..state_subset_size {
        let current_index = rng.gen_range(0, j);
        if map.contains_key(&current_index) {
            map.insert(j, generate_pixel_transform(rng));
        } else {
            map.insert(current_index, generate_pixel_transform(rng));
        }
    }

    Mutation { map }
}

use rand::distributions::{Range, Sample};

fn generate_pixel_transform<R: Rng + Sized>(rng: &mut R) -> PixelTransform {
    let mut result = [Blue; COLOUR_COUNT];

    let mut colour_range = Range::new(0, COLOUR_COUNT);

    for i in 0..COLOUR_COUNT {
        result[i] = Colour::from(colour_range.sample(rng));
    }

    result
}
