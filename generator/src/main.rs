use std::fs::OpenOptions;
use std::path::Path;
use std::io::Write;

fn main() {
    let filename = "../player/src/game.rs";
    let path = Path::new(filename);
    println!("overwriting {:?}", path.as_os_str());

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

        if (clearColour != 0) {
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
        println!("overwrote {} successfully", filename);
    }
}
