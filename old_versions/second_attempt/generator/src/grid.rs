use rand::Rng;

use common::*;
use error::Result;

pub fn render_game<R: Rng + ?Sized>(rng: &mut R, spec: GridGameSpec) -> Result<RenderableGame> {
    let (w, h) = spec.grid_dimensions;

    let grid_cell_size = (next_power_of_2(w as _) as u8, next_power_of_2(h as _) as u8);

    debug_assert!(spec.entity_animacies.len() == spec.entity_controls.len());

    let entity_type_count = spec.entity_animacies.len();

    let mut appearances = Vec::with_capacity(entity_type_count);
    let mut positions = Vec::with_capacity(entity_type_count);
    let mut varieties = Vec::with_capacity(entity_type_count);

    let mut input_responders = Vec::with_capacity(entity_type_count);

    for i in 0..entity_type_count {
        appearances.push(vec![Appearance(rng.gen::<u8>().saturating_add(1))]);

        positions.push(vec![(rng.gen_range(0, w), rng.gen_range(0, h))]);

        varieties.push(i as Variety);

        input_responders.push(InputResponder {
            button_responses: spec.entity_controls[i]
                .map(controls_to_button_responses)
                .unwrap_or_else(|| Default::default()),
            variety: i as Variety,
        });
    }

    let initial_state = InitialState {
        animacies: spec.entity_animacies,
        positions,
        appearances,
        varieties,
        ..Default::default()
    };

    let custom_consts = format!(
        stringify!{
            pub const GRID_DIMENSIONS: (u8, u8) = ({}, {});
            pub const GRID_CELL_SIZE: (u8, u8) = ({}, {});
        },
        w,
        h,
        grid_cell_size.0,
        grid_cell_size.1
    );

    let custom_code = code_string!{
        impl GameState {
            pub fn find_nearest_empty_pos(
                &self,
                start_pos: Position,
            ) -> Option<Position> {{
                use std::collections::{{VecDeque, HashSet}};
                //PERF would it be faster to preallocate this?
                //I expect the common case not to use anything close to the maximum.
                let mut queue = VecDeque::new();

                let mut full = HashSet::with_capacity(GameState::ENTITY_COUNT);
                let mut visited = HashSet::with_capacity(GameState::ENTITY_COUNT);

                //PERF it might make sense to just keep track of which slots are free all the time
                for i in 0..GameState::ENTITY_COUNT {{
                    if !self.entities[i].is_empty() {{
                        full.insert(self.positions[i]);
                    }}
                }}

                queue.push_back(start_pos);

                while let Some(pos) = queue.pop_front() {{
                    if !full.contains(&pos) {{
                        return Some(pos)
                    }}

                    if visited.contains(&pos) {{
                        continue;
                    }}

                    visited.insert(pos);

                    //TODO we might want to figure out a heuristic on which direction to look in
                    //first, given the start_pos. It would also prevent the empty ones always
                    //being in a certain direction.

                    if pos.0 > 0 {{
                        queue.push_back((pos.0 - 1, pos.1));
                    }}

                    if pos.0 < GameState::GRID_DIMENSIONS.0 - 1 {{
                        queue.push_back((pos.0 + 1, pos.1));
                    }}

                    if pos.1 > 0 {{
                        queue.push_back((pos.0, pos.1 - 1));
                    }}

                    if pos.1 < GameState::GRID_DIMENSIONS.1 - 1 {{
                        queue.push_back((pos.0, pos.1 + 1));
                    }}
                }}

                None
            }}
        }
    };

    let game_state_impl = GameStateImpl {
        entity_count: 256,
        entity_piece_count: 1,
        custom_consts,
        initial_state,
        custom_code,
        ..Default::default()
    };

    Ok(RenderableGame {
        game_type: GridBased,
        input_responders,
        game_state_impl,
        grid_dimensions: spec.grid_dimensions,
    })
}

fn controls_to_button_responses(controls: EntityControl) -> ButtonResponses {
    let up;
    let down;
    let left;
    let right;

    match controls.movement {
        Orthogonal => {
            up = code_string!{
                let pos = &mut state.positions[id];
                pos.1 = pos.1.saturating_sub(1);
            };
            down = code_string!{
                let pos = &mut state.positions[id];
                pos.1 = pos.1.saturating_add(1);
            };
            left = code_string!{
                let pos = &mut state.positions[id];
                pos.0 = pos.0.saturating_sub(1);
            };
            right = code_string!{
                let pos = &mut state.positions[id];
                pos.0 = pos.0.saturating_add(1);
            };
        }
        Diagonal => {
            up = code_string!{
                let pos = &mut state.positions[id];
                pos.0 = pos.0.saturating_add(1);
                pos.1 = pos.1.saturating_sub(1);
            };
            down = code_string!{
                let pos = &mut state.positions[id];
                pos.0 = pos.0.saturating_sub(1);
                pos.1 = pos.1.saturating_add(1);
            };
            left = code_string!{
                let pos = &mut state.positions[id];
                pos.0 = pos.0.saturating_sub(1);
                pos.1 = pos.1.saturating_sub(1);
            };
            right = code_string!{
                let pos = &mut state.positions[id];
                pos.0 = pos.0.saturating_add(1);
                pos.1 = pos.1.saturating_add(1);
            };
        }
    }

    let a = action_to_button_responses(controls.a);
    let b = action_to_button_responses(controls.b);
    let select = action_to_button_responses(controls.select);

    let start = code_string!{};

    ButtonResponses {
        up,
        down,
        left,
        right,
        a,
        b,
        start,
        select,
    }
}

fn action_to_button_responses(action: Action) -> String {
    match action {
        SwapPlayerControlToNext(offset) => format!(
            stringify!{state.player_controlling_variety = state.player_controlling_variety.wrapping_add({});},
            offset
        ),
        CopySelf => code_string!{
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
}

// https://graphics.stanford.edu/~seander/bithacks.html#RoundUpPowerOf2
fn next_power_of_2(mut x: usize) -> usize {
    //The basic idea here is fill in all the bits below the highest set bit
    //and then add one, making a power of two. We do this by taking the
    //highest set bit, (the only one we know we have) and progressively
    //ORing with the lower bits of the number. Once we do the first OR,
    //then we know there are two set bits at the top, so we can set the
    //next two below it at once. Then we know the top 4 are set and so on.
    x = x.wrapping_sub(1); //This subtraction makes, for instance, 8 map to 8 instead of 16.
    x |= x >> 1;
    x |= x >> 2;
    x |= x >> 4;
    x |= x >> 8;
    x |= x >> 16;
    x |= x >> 32;
    x.wrapping_add(1)
}
