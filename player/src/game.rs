
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
                    state.mark_won();
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
        respond_to_input(state, input, 0, Variety::default());

        if state.has_won() {
            draw_winning_screen(framebuffer);
        }
    }
    