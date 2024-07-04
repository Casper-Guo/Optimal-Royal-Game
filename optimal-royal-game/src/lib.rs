mod board;
mod build_graph;
mod consts;
#[cfg(test)]
mod tests;
mod types;

use consts::{BLACK, WHITE};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use types::{Graph, State, StateValue, Values};

const P_ROLL: [f32; 5] = [0.0625, 0.25, 0.375, 0.25, 0.0625];

fn initial_value_heuristics(state: State) -> f32 {
    // let mut white_total = (state & (0b111 << 56)) >> 56;
    // let mut black_total = (state & (0b111 << 59)) >> 59;

    // for offset in {0..28}.step_by(2) {
    //     match (state & (0b11 << offset)) >> offset {
    //         WHITE => white_total += 1,
    //         BLACK => black_total += 1,
    //         _ => (),
    //     }
    // }

    // for offset in [28, 30, 32, 34, 52, 54] {
    //     if (state & (0b11 << offset)) >> offset == BLACK {
    //         black_total += 1;
    //     }
    // }

    // more black pieces still on/off the board
    // means more white pieces have ascended
    // ((black_total - white_total) as f32) * 10.0
    0.0
}

fn initialize_values(graph: &Graph) -> Values {
    let mut values: HashMap<State, RefCell<StateValue>> = HashMap::new();

    for state in graph.keys() {
        values.insert(*state, RefCell::new(initial_value_heuristics(*state)));
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
fn value_iteration<'a>(graph: &Graph, values: &'a mut Values) -> (&'a mut Values, f32) {
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

        *values.get(state).unwrap().borrow_mut() = new_value;
        let state_delta = (new_value - *value.borrow()).abs();
        iter_delta = f32::max(iter_delta, state_delta);
    }

    (values, iter_delta)
}

pub fn write_json<T>(map: &T, filename: String)
where
    T: serde::Serialize,
{
    let fout = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(filename)
        .expect("Unable to open file");
    serde_json::to_writer_pretty(fout, map).unwrap();
}

pub fn driver(graph: &Graph, min_delta: f32, max_iter: usize) {
    let mut values = initialize_values(graph);

    for _num_iter in 0..max_iter {
        let (values, iter_delta) = value_iteration(graph, &mut values);

        if iter_delta < min_delta {
            break;
        }

        // checkpoint condition
        // or the values will be saved every iteration
        write_json(&values, format!("values_iter{}.json", _num_iter));
    }
}
