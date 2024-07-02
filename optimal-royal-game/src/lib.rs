pub mod board;
pub mod build_graph;
mod consts;
#[cfg(test)]
mod tests;
mod types;

use serde_json::to_writer;
use std::collections::{HashMap, HashSet};
use types::Graph;

pub fn value_iteration(graph: Graph, min_delta: f64, max_iter: usize) {}
