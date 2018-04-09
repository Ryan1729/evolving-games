use std::fs::OpenOptions;
use std::path::Path;
use std::io::Write;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

extern crate rand;
use rand::{Rng, SeedableRng, XorShiftRng};

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

    let code = "use common::*;

    #[inline]
    pub fn update_and_render(state: &mut Framebuffer, input: Input) {
        let mut clearColour = 0;

        if input.pressed_this_frame(Button::Left) {
            clearColour = BLUE;
        }

        if input.pressed_this_frame(Button::Right) {
            clearColour = GREEN;
        }

        if input.pressed_this_frame(Button::Up) {
            clearColour = RED;
        }

        if input.pressed_this_frame(Button::Down) {
            clearColour = YELLOW;
        }

        if input.pressed_this_frame(Button::Select) {
            clearColour = PURPLE;
        }

        if input.pressed_this_frame(Button::Start) {
            clearColour = GREY;
        }

        if input.pressed_this_frame(Button::A) {
            clearColour = WHITE;
        }

        if input.pressed_this_frame(Button::B) {
            clearColour = BLACK;
        }

        if clearColour != 0 {
            state.clearTo(clearColour);
        }
    }
";

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)
        .unwrap();

    if let Err(error) = file.write_all(code.as_bytes()) {
        println!("{}", error);
    } else {
        println!("Overwrote {} successfully.", filename);
    }
}
