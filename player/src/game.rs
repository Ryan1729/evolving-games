
    use common::*;

    fn input_responder_0(state: &mut GameState, input: Input, id: usize) {
                if input.pressed_this_frame(Button::Left) {
                    let mut pos = state.positions[id];
pos.0 = pos.0.saturating_sub(1);
pos.1 = pos.1.saturating_sub(1);

                }

                if input.pressed_this_frame(Button::Right) {
                    let mut pos = state.positions[id];
pos.0 = pos.0.saturating_add(1);
pos.1 = pos.1.saturating_add(1);

                }

                if input.pressed_this_frame(Button::Up) {
                    let mut pos = state.positions[id];
pos.0 = pos.0.saturating_add(1);
pos.1 = pos.1.saturating_sub(1);

                }

                if input.pressed_this_frame(Button::Down) {
                    let mut pos = state.positions[id];
pos.0 = pos.0.saturating_sub(1);
pos.1 = pos.1.saturating_add(1);

                }

                if input.pressed_this_frame(Button::Select) {
                    state.player_controlling_variety = state.player_controlling_variety.wrapping_add(155);
                }

                if input.pressed_this_frame(Button::Start) {
                    
                }

                if input.pressed_this_frame(Button::A) {
                    state.player_controlling_variety = state.player_controlling_variety.wrapping_add(46);
                }

                if input.pressed_this_frame(Button::B) {
                    state.player_controlling_variety = state.player_controlling_variety.wrapping_add(32);
                }
            }
        
fn input_responder_1(state: &mut GameState, input: Input, id: usize) {
                if input.pressed_this_frame(Button::Left) {
                    let mut pos = state.positions[id];
pos.0 = pos.0.saturating_sub(1);

                }

                if input.pressed_this_frame(Button::Right) {
                    let mut pos = state.positions[id];
pos.0 = pos.0.saturating_add(1);

                }

                if input.pressed_this_frame(Button::Up) {
                    let mut pos = state.positions[id];
pos.1 = pos.1.saturating_sub(1);

                }

                if input.pressed_this_frame(Button::Down) {
                    let mut pos = state.positions[id];
pos.1 = pos.1.saturating_add(1);

                }

                if input.pressed_this_frame(Button::Select) {
                    if let Some(clone_id) = state.get_free_id() {
            if let Some(clone_pos) = state.find_nearest_empty_pos(state.positions[id]) {
                state.positions[clone_id] = clone_pos;
                state.entities[clone_id] = state.entities[id];
                state.appearances[clone_id] = state.appearances[id];
                state.varieties[clone_id] = state.varieties[id];
            }
        }

                }

                if input.pressed_this_frame(Button::Start) {
                    
                }

                if input.pressed_this_frame(Button::A) {
                    if let Some(clone_id) = state.get_free_id() {
            if let Some(clone_pos) = state.find_nearest_empty_pos(state.positions[id]) {
                state.positions[clone_id] = clone_pos;
                state.entities[clone_id] = state.entities[id];
                state.appearances[clone_id] = state.appearances[id];
                state.varieties[clone_id] = state.varieties[id];
            }
        }

                }

                if input.pressed_this_frame(Button::B) {
                    state.player_controlling_variety = state.player_controlling_variety.wrapping_add(238);
                }
            }
        
fn input_responder_2(state: &mut GameState, input: Input, id: usize) {
                if input.pressed_this_frame(Button::Left) {
                    let mut pos = state.positions[id];
pos.0 = pos.0.saturating_sub(1);

                }

                if input.pressed_this_frame(Button::Right) {
                    let mut pos = state.positions[id];
pos.0 = pos.0.saturating_add(1);

                }

                if input.pressed_this_frame(Button::Up) {
                    let mut pos = state.positions[id];
pos.1 = pos.1.saturating_sub(1);

                }

                if input.pressed_this_frame(Button::Down) {
                    let mut pos = state.positions[id];
pos.1 = pos.1.saturating_add(1);

                }

                if input.pressed_this_frame(Button::Select) {
                    if let Some(clone_id) = state.get_free_id() {
            if let Some(clone_pos) = state.find_nearest_empty_pos(state.positions[id]) {
                state.positions[clone_id] = clone_pos;
                state.entities[clone_id] = state.entities[id];
                state.appearances[clone_id] = state.appearances[id];
                state.varieties[clone_id] = state.varieties[id];
            }
        }

                }

                if input.pressed_this_frame(Button::Start) {
                    
                }

                if input.pressed_this_frame(Button::A) {
                    state.player_controlling_variety = state.player_controlling_variety.wrapping_add(24);
                }

                if input.pressed_this_frame(Button::B) {
                    if let Some(clone_id) = state.get_free_id() {
            if let Some(clone_pos) = state.find_nearest_empty_pos(state.positions[id]) {
                state.positions[clone_id] = clone_pos;
                state.entities[clone_id] = state.entities[id];
                state.appearances[clone_id] = state.appearances[id];
                state.varieties[clone_id] = state.varieties[id];
            }
        }

                }
            }
        
fn respond_to_input(state: &mut GameState, input: Input, id: usize, variety: Variety) {
    match variety {
           0 => input_responder_0(state, input, id),
           1 => input_responder_1(state, input, id),
           2 => input_responder_2(state, input, id),
           _ => {},
        }
    }


    #[inline]
    pub fn update_and_render(framebuffer: &mut Framebuffer, state: &mut GameState, input: Input) {
        for id in 0..GameState::ENTITY_COUNT {
            if !state.entities[id].contains(Component::Animate) {
                continue;
            }

            let current_variety = state.varieties[id];
            if state.entities[id].contains(Component::PlayerControlled) {
                if current_variety == state.player_controlling_variety {
                    respond_to_input(state, input, id, current_variety);
                }
            } else {
                let artifical_input = Input::default(); //TODO AI

                respond_to_input(state, artifical_input, id, current_variety);
            }
        }

        framebuffer.clear();

        for i in 0..GameState::ENTITY_COUNT {
            let (x, y) = state.positions[i];
            let appearance = &mut state.appearances[i];

            appearance.render(framebuffer, (x as usize, y as usize), (0, 0));
        }
    }
    