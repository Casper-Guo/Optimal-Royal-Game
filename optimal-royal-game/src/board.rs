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
use crate::types::{Roll, State, States};
use num::{iter, Integer};

pub fn verify_state(state: State) -> Result<(), State> {
    let (mut white_total, mut black_total, mut offset): (u64, u64, u64) = (0, 0, 0);

    while offset < 28 {
        let status_bit = (state & (0b11 << offset)) >> offset;
        if status_bit == WHITE {
            white_total += 1;
        }
        offset += 2;
    }
    while offset < 56 {
        let status_bit = (state & (0b11 << offset)) >> offset;
        if status_bit == BLACK {
            black_total += 1;
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
pub fn get_next_states(current_state: State, roll: Roll, player: impl Integer) -> States {
    let v: States = Vec::new();
    v
}

/// The board is assumed valid. This permits the following inference.
/// For example, if white has no pieces not yet or currently on the board, then all its pieces must have ascended.
/// Therefore the board is in a white win state.
pub fn set_endgame_status(state: State) -> State {
    // PERF: consider removing preprocessing as we can guarentee only unset states are fed into this function
    if ((state & (0b11 << 62)) >> 62) != UNSET_STATUS {
        return state;
    }

    let mut white_total = (state & (0b111 << 56)) >> 56;
    let mut black_total = (state & (0b111 << 59)) >> 59;

    if white_total > 0 && black_total > 0 {
        return state + (0b11 << 62);
    }

    let iter_range = if white_total == 0 {
        0..28
    } else {
        28..56
    };

    for offset in iter_range.step_by(2) {
        match (state & (0b11 << offset)) >> offset {
            WHITE => white_total += 1,
            BLACK => black_total += 1,
            _ => (),
        }
        if white_total > 0 && black_total > 0 {
            return state + (0b11 << 62);
        }
    }

    if white_total == 0 {
        // if white_total == 0, then iter_range was set to 0..28
        // which means we have checked all possible white piece locations
        return state + (0b01 << 62);
    } else {
        // white_total != 0, implies black_total == 0
        // else would have returned already
        for offset in [28, 30, 32, 34, 52, 54] {
            if (state & (0b11 << offset)) >> offset == BLACK {
                // still in progress
                return state + (0b11 << 62);
            }
        }
    }

    // given white_total != 0 and we have checked black_total == 0
    state + (0b10 << 62)
}
