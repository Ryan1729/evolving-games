use inner_common::*;

        impl GameState {
            pub const ENTITY_COUNT: usize = 256;
            pub const GRID_DIMENSIONS: (u8, u8) = (228, 54);

            pub fn new() -> GameState {
                let mut entities = [Component::Ty::empty(); GameState::ENTITY_COUNT];

                let mut positions = [(0, 0); GameState::ENTITY_COUNT];
                let mut appearances = [Appearance::default(); GameState::ENTITY_COUNT];
                let mut varieties = [Variety::default(); GameState::ENTITY_COUNT];

                let player_controlling_variety = Variety::default();

                    entities[0] = Component::Player;
    positions[0] = (36, 14);
    appearances[0] = Appearance(2);
    varieties[0] = 0;
    entities[1] = Component::Player;
    positions[1] = (47, 1);
    appearances[1] = Appearance(8);
    varieties[1] = 1;
    entities[2] = Component::Player;
    positions[2] = (58, 38);
    appearances[2] = Appearance(47);
    varieties[2] = 2;


                GameState {
                    entities,
                    positions,
                    appearances,
                    varieties,
                    player_controlling_variety,
                }
            }

            pub fn get_free_id(&self) -> Option<usize> {
                for (i, e) in self.entities.iter().enumerate() {
                    if e.is_empty() {
                        return Some(i);
                    }
                }

                None
            }

            pub fn find_nearest_empty_pos(
                &self,
                start_pos: Position,
            ) -> Option<Position> {
                use std::collections::{VecDeque, HashSet};
                //PERF would it be faster to preallocate this?
                //I expect the common case not to use anything close to the maximum.
                let mut queue = VecDeque::new();

                let mut full = HashSet::with_capacity(GameState::ENTITY_COUNT);
                let mut visited = HashSet::with_capacity(GameState::ENTITY_COUNT);

                //PERF it might make sense to just keep track of which slots are free all the time
                for i in 0..GameState::ENTITY_COUNT {
                    if !self.entities[i].is_empty() {
                        full.insert(self.positions[i]);
                    }
                }

                queue.push_back(start_pos);

                while let Some(pos) = queue.pop_front() {
                    if !full.contains(&pos) {
                        return Some(pos)
                    }

                    if visited.contains(&pos) {
                        continue;
                    }

                    visited.insert(pos);

                    //TODO we might want to figure out a heuristic on which direction to look in
                    //first, given the start_pos. It would also prevent the empty ones always
                    //being in a certain direction.

                    if pos.0 > 0 {
                        queue.push_back((pos.0 - 1, pos.1));
                    }

                    if pos.0 < GameState::GRID_DIMENSIONS.0 - 1 {
                        queue.push_back((pos.0 + 1, pos.1));
                    }

                    if pos.1 > 0 {
                        queue.push_back((pos.0, pos.1 - 1));
                    }

                    if pos.1 < GameState::GRID_DIMENSIONS.1 - 1 {
                        queue.push_back((pos.0, pos.1 + 1));
                    }
                }

                None
            }
        }
