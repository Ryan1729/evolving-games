
    use common::*;

    fn input_responder_0(state: &mut GameState, input: Input, id: usize) {
                if input.pressed_this_frame(Button::Left) {
                    state . move_left ( id ) ;
                }

                if input.pressed_this_frame(Button::Right) {
                    state . move_right ( id ) ;
                }

                if input.pressed_this_frame(Button::Up) {
                    state . move_up ( id ) ;
                }

                if input.pressed_this_frame(Button::Down) {
                    state . move_down ( id ) ;
                }

                if input.pressed_this_frame(Button::Select) {
                    
                }

                if input.pressed_this_frame(Button::Start) {
                    
                }

                if input.pressed_this_frame(Button::A) {
                    
                }

                if input.pressed_this_frame(Button::B) {
                    
                }
            }
        
fn respond_to_input(state: &mut GameState, input: Input, id: usize, variety: Variety) {
    match variety {
           0 => input_responder_0(state, input, id),
           _ => {},
        }
    }


    #[inline]
    pub fn update_and_render(framebuffer: &mut Framebuffer, state: &mut GameState, input: Input) {
        for id in 0..GameState::ENTITY_COUNT {
            if state.entities[id].contains(Component::PlayerControlled) {
                respond_to_input(state, input, id, Variety::default());
            }
        }

        framebuffer.clear();

        for entity in 0..GameState::ENTITY_COUNT {
            if state.entities[entity].is_empty() {
                continue;
            }

            for i in 0..GameState::ENTITY_PIECE_COUNT {
                let (x, y) = state.positions[entity][i];
                let appearance = &state.appearances[entity][i];
                let (w, h) = state.sizes[entity][i];

                render(
                    appearance,
                    framebuffer,
                    (x as usize, y as usize),
                    (w as usize, h as usize)
                );
            }
        }
    }
    