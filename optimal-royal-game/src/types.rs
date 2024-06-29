use crate::HashMap;

pub type State = u64;
pub type Roll = u8;
pub type StateValue = f32;
pub type States = Vec<State>;
pub type Moves = HashMap<Roll, States>;
pub type Graph = HashMap<State, Moves>;
