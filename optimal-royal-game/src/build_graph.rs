use crate::board::{get_next_states, set_endgame_status};
use crate::types::{Graph, State, States};
use crate::{HashMap, HashSet};

/// final version should stop expanding if a player has won
/// implement earlier stopping for testing
fn is_endstate(state: State) -> bool {
    // let status = (state & (0b11 << 62)) >> 62;
    // status != IN_PROGRESS
    state & 0xFF00000 != 0
}

pub fn dfs() -> Graph {
    let mut queue: States = Vec::from([18374686479671623680]);
    let mut seen: HashSet<State> = HashSet::from([18374686479671623680]);
    let mut graph: Graph = HashMap::new();

    while let Some(current_state) = queue.pop() {
        graph.insert(current_state, HashMap::new());
        seen.remove(&current_state);

        for roll in 1..=4 {
            let mut next_states = get_next_states(current_state, roll, 0);

            for next_state in &mut next_states {
                *next_state = set_endgame_status(*next_state);
                if !(seen.contains(next_state)
                    || graph.contains_key(next_state)
                    || is_endstate(*next_state))
                {
                    seen.insert(*next_state);
                    queue.push(*next_state);
                }
            }

            // as simply takes the lowest 8 bits of the number
            // safe because roll is guarenteed to be between 1 and 4 here
            graph
                .get_mut(&current_state)
                .unwrap()
                .insert(roll as u8, next_states);
        }
        for roll in 1..=4 {
            let mut next_states = get_next_states(current_state, roll, 1);

            for next_state in &mut next_states {
                *next_state = set_endgame_status(*next_state);
                if !(seen.contains(next_state)
                    || graph.contains_key(next_state)
                    || is_endstate(*next_state))
                {
                    seen.insert(*next_state);
                    queue.push(*next_state);
                }
            }
        }
    }
    graph
}
