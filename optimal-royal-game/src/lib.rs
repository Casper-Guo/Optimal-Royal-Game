mod board;
mod build_graph;
mod consts;
#[cfg(test)]
mod tests;
mod types;

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

use types::{Graph, State, StateValue};

const P_ROLL: [f32; 5] = [0.0625, 0.25, 0.375, 0.25, 0.0625];

fn initialize_values(graph: &Graph) -> HashMap<State, RefCell<StateValue>> {
    let mut values: HashMap<State, RefCell<StateValue>> = HashMap::new();

    for state in graph.keys() {
        // implement a better heuristic for initial values
        values.insert(*state, RefCell::new(0.0));
    }

    values
}

fn calculate_reward(state: State) -> f32 {
    let rosette_status = (state & (0b11 << 14)) >> 14;

    match rosette_status {
        0 => 0.0,
        1 => 100.0,
        2 => -100.0,
        _ => 0.0,
    }
}

/// The awkward RefCell type is used because it permits mutating the values Hashmap while iterating.
/// This is avoidable but will lead to slower convergence.
fn value_iteration(
    graph: &Graph,
    min_delta: f32,
    max_iter: usize,
) -> HashMap<State, RefCell<StateValue>> {
    let values = initialize_values(graph);

    for _num_iter in 0..max_iter {
        let mut iter_delta = 0.0;

        for (state, value) in values.iter() {
            let mut new_value: f32 = 0.0625 * *value.borrow();
            let next_states = graph.get(state).unwrap();

            for (roll, roll_states) in next_states.iter() {
                let mut roll_value = f32::MIN;

                // reward only calculated for transition to end states
                if (roll_states.len()) == 1 {
                    // check end game status instead of values membership
                    roll_value = if values.contains_key(&roll_states[0]) {
                        *values.get(&roll_states[0]).unwrap().borrow()
                    } else {
                        calculate_reward(roll_states[0])
                    }
                } else {
                    for roll_state in roll_states {
                        // in the final version we shouldn't need any conditional in this branch
                        let move_value = if values.contains_key(roll_state) {
                            *values.get(roll_state).unwrap().borrow()
                        } else {
                            calculate_reward(roll_states[0])
                        };
                        roll_value = f32::max(roll_value, move_value);
                    }
                }

                new_value += P_ROLL[usize::from(*roll)] * roll_value;
            }

            let state_delta = (new_value - *value.borrow()).abs();
            iter_delta = f32::max(iter_delta, state_delta);
            *values.get(state).unwrap().borrow_mut() = new_value;
        }

        if iter_delta < min_delta {
            break;
        }
    }

    values
}

pub fn driver(min_delta: f32, max_iter: usize) {
    let fin = std::fs::OpenOptions::new()
        .read(true)
        .open("graph.json")
        .expect("Unable to open file");
    let graph: Graph = serde_json::from_reader(fin).unwrap();

    let values = value_iteration(&graph, min_delta, max_iter);

    let fout = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("values.json")
        .expect("Unable to open file");
    serde_json::to_writer_pretty(fout, &values).unwrap();
}

pub fn save_graph() {
    let fout = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("graph.json")
        .expect("Unable to open file");
    let graph = build_graph::dfs();
    serde_json::to_writer_pretty(fout, &graph).unwrap();
}
