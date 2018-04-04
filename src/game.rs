use common::*;

impl GameState {
    fn isSelectrix(&self, id: usize) -> bool {
        self.entities[id].contains(Component::Player | Component::IntraCellPosition)
            && self.player_types[id] == PlayerType::Selectrix
    }

    fn isAvatar(&self, id: usize) -> bool {
        self.entities[id].contains(Component::Player) && self.player_types[id] == PlayerType::Avatar
    }
}
//TODO picking up and throwing orbs (no interactions just placement first)

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
        } else if state.mode == Mode::MoveSelectrix && state.isSelectrix(i) {
            let appearance = &mut state.appearances[i];

            if appearance.is_offset() {
                appearance.reduce_offset(8);
                continue;
            }

            let (mut x, mut y) = state.positions[i];
            let mut inter_pos = state.intra_cell_positions[i];

            if input.pressed_this_frame(Button::Left) {
                if x > 0 && inter_pos.on_left_edge() {
                    x = x.saturating_sub(1);
                    inter_pos = inter_pos.left();
                    appearance.offset.0 = (CELL_WIDTH / 2) as isize;
                } else if !inter_pos.on_left_edge() {
                    inter_pos = inter_pos.left();
                    appearance.offset.0 = (CELL_WIDTH / 2) as isize;
                }
            }

            if input.pressed_this_frame(Button::Right) {
                if x < BOARD_WIDTH - 1 && inter_pos.on_right_edge() {
                    x = x.saturating_add(1);
                    inter_pos = inter_pos.right();
                    appearance.offset.0 = -((CELL_WIDTH / 2) as isize);
                } else if !inter_pos.on_right_edge() {
                    inter_pos = inter_pos.right();
                    appearance.offset.0 = -((CELL_WIDTH / 2) as isize);
                }
            }

            if input.pressed_this_frame(Button::Up) {
                if y > 0 && inter_pos.on_top_edge() {
                    y = y.saturating_sub(1);
                    inter_pos = inter_pos.up();
                    appearance.offset.1 = (CELL_HEIGHT / 2) as isize;
                } else if !inter_pos.on_top_edge() {
                    inter_pos = inter_pos.up();
                    appearance.offset.1 = (CELL_HEIGHT / 2) as isize;
                }
            }

            if input.pressed_this_frame(Button::Down) {
                if y < BOARD_HEIGHT - 1 && inter_pos.on_bottom_edge() {
                    y = y.saturating_add(1);
                    inter_pos = inter_pos.down();
                    appearance.offset.1 = -((CELL_HEIGHT / 2) as isize);
                } else if !inter_pos.on_bottom_edge() {
                    inter_pos = inter_pos.down();
                    appearance.offset.1 = -((CELL_HEIGHT / 2) as isize);
                }
            }

            state.positions[i] = (x, y);
            state.intra_cell_positions[i] = inter_pos;
        }
    }

    state.mode = match state.mode {
        Mode::MoveAvatar if input.pressed_this_frame(Button::B) => {
            state.positions[state.selectrixId] = state.positions[state.avatarId];
            state.entities[state.selectrixId].insert(Component::Appearance);

            Mode::MoveSelectrix
        }
        Mode::MoveSelectrix if input.pressed_this_frame(Button::B) => {
            state.entities[state.selectrixId].remove(Component::Appearance);

            Mode::MoveAvatar
        }
        _ => state.mode,
    };

    if input.pressed_this_frame(Button::Select) {
        state.inventory_index = (state.inventory_index + 1) % state.inventory.len() as u8;
    }

    framebuffer.clear();

    for i in 0..GameState::ENTITY_COUNT {
        let entity = state.entities[i];
        if state.isSelectrix(i) {
            if state.mode == Mode::MoveSelectrix {
                let pos = state.positions[i];
                let inter_pos = state.intra_cell_positions[i];

                let appearance = &mut state.appearances[i];
                appearance.render_intra_positioned(framebuffer, pos, inter_pos);
            }
        } else if entity
            .contains(Component::Position | Component::Appearance | Component::IntraCellPosition)
        {
            let pos = state.positions[i];
            let inter_pos = state.intra_cell_positions[i];

            let appearance = &mut state.appearances[i];
            appearance.render_intra_positioned(framebuffer, pos, inter_pos);
        } else if entity.contains(Component::Position | Component::Appearance) {
            let pos = state.positions[i];

            let appearance = &mut state.appearances[i];
            appearance.render_positioned(framebuffer, pos);
        }
    }

    framebuffer.draw_filled_rect(HUD_LEFT_EDGE, 0, HUD_WIDTH, SCREEN_HEIGHT, GREY);

    for i in 0..state.inventory.len() {
        let item = state.inventory[i];

        let x = INVENTORY_LEFT_EDGE;
        let y = (INVENTORY_HEIGHT + 4) * (i + 1);

        framebuffer.draw_filled_rect(x, y, INVENTORY_WIDTH, INVENTORY_HEIGHT, PURPLE);

        match item {
            OrbType::DeadOrb => {
                framebuffer.draw_circle(
                    x + INVENTORY_WIDTH / 2,
                    y + INVENTORY_HEIGHT / 2,
                    ORB_RADIUS,
                    RED,
                );
            }
            _ => {}
        }

        if i as u8 == state.inventory_index {
            framebuffer.draw_rect(x, y, INVENTORY_WIDTH, INVENTORY_HEIGHT, YELLOW);
        }
    }
}
