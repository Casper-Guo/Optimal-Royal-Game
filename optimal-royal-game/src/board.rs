use crate::consts::*;
/// - Two bits each for board grids to achieve consistency in the order of W1...W4, 5...12, W13, W14, B1...B4, 5...12, B13, B14 (from least to most significant).
///     - 00: unoccupied
///     - 01: white
///     - 10: black
/// - Three bits each for WS and BS from the least to the most significant (6 total)
/// - Bits 62-63 indicate the endgame state. This is useful for stopping search and assigning rewards.
///     - 00: not set/calculated
///     - 01: white win
///     - 10: black win
///     - 11: in progress
use crate::types::{State, States};
use std::panic;

pub fn verify_state(state: State) -> Result<(), State> {
    let (mut white_total, mut black_total, mut offset): (u64, u64, u64) = (0, 0, 0);

    while offset < 28 {
        let status_bit = (state & (0b11 << offset)) >> offset;
        if status_bit == WHITE {
            white_total += 1;
        }
        if status_bit == 3 {
            panic!("{state}");
        }
        offset += 2;
    }
    while offset < 56 {
        let status_bit = (state & (0b11 << offset)) >> offset;
        if status_bit == BLACK {
            black_total += 1;
        }
        if status_bit == 3 {
            panic!("{state}");
        }
        offset += 2;
    }

    white_total += (state & (0b111 << 56)) >> 56;
    black_total += (state & (0b111 << 59)) >> 59;

    if white_total > 7 || black_total > 7 {
        Err(state)
    } else {
        Ok(())
    }
}

/// Expect player to be either 0 (white) or 1 (black)
/// Do not correctly handle end game states
/// Assume passed in states represent in-progress games
/// Assume roll is between 1 and 4 inclusive.
pub fn get_next_states(current_state: State, roll: u64, player: u64) -> States {
    let mut next_states: States = Vec::with_capacity(7);

    let self_offset_start = player * 28;
    let opponent_offset_start = 28 - self_offset_start;
    let self_status = player + 1;

    // handle onboard moves
    // take advantage of the player argument to calculate the correct offset
    let start_offset = 56 + 3 * player;
    let num_off_board = (current_state & (0b111 << start_offset)) >> start_offset;
    // roll - 1 because a roll of 1 onboards to grid 0, et cetera
    let onboard_grid_offset = self_offset_start + 2 * (roll - 1);
    let onboard_grid_status =
        (current_state & (0b11 << onboard_grid_offset)) >> onboard_grid_offset;

    if num_off_board > 0 && onboard_grid_status == UNOCCUPIED {
        let mut next_state = current_state;
        next_state -= 1 << start_offset;

        // only 2 bits need to be updated since the onboard grid must be a private grid
        next_state += self_status << onboard_grid_offset;
        next_states.push(next_state);
    }

    let ascension_grid = 14 - roll;
    // grid is 0-indexed as it is used to calculate the offset
    for start_grid in 0..ascension_grid {
        let dest_grid = start_grid + roll;

        let start_grid_offset = self_offset_start + 2 * start_grid;
        let dest_grid_offset = self_offset_start + 2 * dest_grid;

        let start_grid_status = (current_state & (0b11 << start_grid_offset)) >> start_grid_offset;
        let dest_grid_status = (current_state & (0b11 << dest_grid_offset)) >> dest_grid_offset;

        // PERF: reduce branching
        if start_grid_status == self_status && dest_grid_status != self_status {
            // 7 instead of 8 because of 0-indexing
            if !(dest_grid == 7 && dest_grid_status != UNOCCUPIED) {
                let mut next_state = current_state;
                next_state -= start_grid_status << start_grid_offset;

                // reset destination grid status bits first before updating
                next_state -= dest_grid_status << dest_grid_offset;
                next_state += self_status << dest_grid_offset;

                if (4..12).contains(&start_grid) {
                    let opponent_start_grid_offset =
                        start_grid_offset - self_offset_start + opponent_offset_start;
                    next_state -= self_status << opponent_start_grid_offset;
                }

                if (4..12).contains(&dest_grid) {
                    let opponent_dest_grid_offset =
                        dest_grid_offset - self_offset_start + opponent_offset_start;
                    next_state -= dest_grid_status << opponent_dest_grid_offset;
                    next_state += self_status << opponent_dest_grid_offset;
                }

                // handles capturing
                if dest_grid_status != UNOCCUPIED {
                    let opponent_offset_start = 56 + 3 * (1 - player);
                    next_state += 1 << opponent_offset_start;
                }

                next_states.push(next_state);
            } else {
                // handle center rosette special case
                // check if next grid (grid 8) is unoccupied
                let next_grid_offset = self_offset_start + 2 * 8;
                let next_grid_status =
                    (current_state & (0b11 << next_grid_offset)) >> next_grid_offset;
                if next_grid_status == UNOCCUPIED {
                    let mut next_state = current_state;
                    let opponent_start_grid_offset =
                        start_grid_offset - self_offset_start + opponent_offset_start;
                    let opponent_next_grid_offset = 2 * 8 + opponent_offset_start;

                    next_state -= start_grid_status << start_grid_offset;
                    next_state += self_status << next_grid_offset;

                    // grid 3 is the only private grid from which
                    // a piece can land on grid 7
                    // and be subsequently bumped to grid 8
                    if start_grid != 3 {
                        next_state -= self_status << opponent_start_grid_offset;
                    }
                    next_state += self_status << opponent_next_grid_offset;

                    next_states.push(next_state);
                }
            }
        }
    }

    // handle ascensions
    let ascension_grid_offset = self_offset_start + 2 * ascension_grid;
    let ascension_grid_status =
        (current_state & (0b11 << ascension_grid_offset)) >> ascension_grid_offset;
    if ascension_grid_status == self_status {
        let mut next_state =
            current_state - (self_status << (self_offset_start + 2 * ascension_grid));

        if (4..12).contains(&ascension_grid) {
            let opponent_ascension_grid_offset =
                ascension_grid_offset - self_offset_start + opponent_offset_start;
            next_state -= ascension_grid_status << opponent_ascension_grid_offset;
        }
        next_states.push(next_state);
    }

    next_states
}

/// The board is assumed valid. This permits the following inference.
/// For example, if white has no pieces not yet or currently on the board, then all its pieces must have ascended.
/// Therefore the board is in a white win state.
pub fn set_endgame_status(state: State) -> State {
    // grab lower 62 bits
    let state = state & 0x3FFFFFFFFFFFFFFF;
    let mut white_total = (state & (0b111 << 56)) >> 56;
    let mut black_total = (state & (0b111 << 59)) >> 59;

    if white_total > 0 && black_total > 0 {
        return state + (0b11 << 62);
    }

    for offset in { 0..28 }.step_by(2) {
        match (state & (0b11 << offset)) >> offset {
            WHITE => white_total += 1,
            BLACK => black_total += 1,
            _ => (),
        }
    }

    if white_total == 0 {
        // if white_total == 0, then iter_range was set to 0..28
        // which means we have checked all possible white piece locations
        return state + (0b01 << 62);
    }

    if black_total == 0 {
        // still need to check black private grids
        for offset in [28, 30, 32, 34, 52, 54] {
            if (state & (0b11 << offset)) >> offset == BLACK {
                // still in progress
                return state + (0b11 << 62);
            }
        }
        return state + (0b10 << 62);
    }

    state + (0b11 << 62)
}
