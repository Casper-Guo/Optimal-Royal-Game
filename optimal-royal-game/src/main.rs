fn main() {
    let graph = optimal_royal_game::build_graph::dfs();
    // println!("{:?}", graph.get(&3206567470190051856));
    println!("{:?}", graph.len());
}
