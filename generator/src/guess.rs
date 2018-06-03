use rand::Rng;

use common::*;
use error::Result;

pub fn render_game<R: Rng + Sized>(rng: &mut R) -> Result<RenderableGame> {
    let mut button_responses = ButtonResponses::default();

    let winning_index = rng.gen_range(0, BUTTON_COUNT);

    let winner = code_string!{
        state.mark_won();
    };

    match winning_index {
        0 => {
            button_responses.up = winner;
        }
        1 => {
            button_responses.down = winner;
        }
        2 => {
            button_responses.left = winner;
        }
        3 => {
            button_responses.right = winner;
        }
        4 => {
            button_responses.a = winner;
        }
        5 => {
            button_responses.b = winner;
        }
        6 => {
            button_responses.start = winner;
        }
        7 => {
            button_responses.select = winner;
        }
        _ => {}
    }

    let responder = InputResponder {
        button_responses,
        variety: Default::default(),
    };

    let mut game_state_impl: GameStateImpl = Default::default();

    game_state_impl.custom_code = code_string!{
        pub fn mark_won(&mut self) {
            self.positions[0] = (1,1);
        }

        pub fn has_won(&self) -> bool {
            self.positions[0].0 == 1
        }
    };

    Ok(RenderableGame {
        game_type: Guess,
        input_responders: vec![responder],
        game_state_impl,
        grid_dimensions: Default::default(),
    })
}
