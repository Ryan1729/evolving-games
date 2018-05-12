
    use common::*;

    fn input_responder_0(state: &mut GameState, input: Input, id: usize) {
                if input.pressed_this_frame(Button::Left) {
                    
                }

                if input.pressed_this_frame(Button::Right) {
                    
                }

                if input.pressed_this_frame(Button::Up) {
                    
                }

                if input.pressed_this_frame(Button::Down) {
                    
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

        for i in 0..GameState::ENTITY_COUNT {
            let (x, y) = state.positions[i];
            let appearance = &mut state.appearances[i];

            appearance.render(framebuffer, (x as usize, y as usize), (20, 28));
        }
    }
    