use std::fs::OpenOptions;
use std::path::Path;
use std::io::Write;

fn main() {
    let filename = "../player/src/game.rs";
    println!("overwriting {}", filename);

    let code = "use common::*;

impl GameState {
    fn isAvatar(&self, id: usize) -> bool {
        self.entities[id].contains(Component::Player) && self.player_types[id] == PlayerType::Avatar
    }
}

#[inline]
pub fn update_and_render(state: &mut GameState, framebuffer: &mut Framebuffer, input: Input) {
    for i in 0..GameState::ENTITY_COUNT {
        if state.mode == Mode::MoveAvatar && state.isAvatar(i) {
            let appearance = &mut state.appearances[i];

            if appearance.is_offset() {
                appearance.reduce_offset(8);
                continue;
            }

            let (mut x, mut y) = state.positions[i];

            if input.pressed_this_frame(Button::Left) && x > 0 {
                x = x.saturating_sub(1);
                appearance.offset.0 = CELL_WIDTH as isize;
            }

            if input.pressed_this_frame(Button::Right) && x < BOARD_WIDTH - 1 {
                x = x.saturating_add(1);
                appearance.offset.0 = -(CELL_WIDTH as isize)
            }

            if input.pressed_this_frame(Button::Up) && y > 0 {
                y = y.saturating_sub(1);
                appearance.offset.1 = CELL_WIDTH as isize;
            }

            if input.pressed_this_frame(Button::Down) && y < BOARD_HEIGHT - 1 {
                y = y.saturating_add(1);
                appearance.offset.1 = -(CELL_WIDTH as isize);
            }

            state.positions[i] = (x, y);
        }
    }

    framebuffer.clear();

    for i in 0..GameState::ENTITY_COUNT {
        let entity = state.entities[i];
        if entity.contains(Component::Position | Component::Appearance) {
            let pos = state.positions[i];

            let appearance = &mut state.appearances[i];
            appearance.render_positioned(framebuffer, pos);
        }
    }
}
    ";

    println!("{:?}", Path::new(filename).as_os_str());

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(Path::new(filename))
        .unwrap();

    if let Err(error) = file.write_all(code.as_bytes()) {
        println!("{}", error);
    } else {
        println!("overwrote {} successfully", filename);
    }
}
