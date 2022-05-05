use fdg_sim::{
    petgraph::graph::NodeIndex, CpuSimulation, ForceGraph, ForceGraphHelper, Simulation,
    SimulationParameters,
};

#[macroquad::main("Force Graph 4D Cube Demo")]
async fn main() {
    pretty_env_logger::init();

    let mut graph: ForceGraph<()> = ForceGraph::default();

    // create center cube
    let mut cube: [NodeIndex; 8] = gen_cube(&mut graph);
    let layers: u8 = 1;

    for _ in 0..layers {
        cube = add_layer(&mut graph, cube);
    }

    fdg_macroquad::run_window(&mut CpuSimulation::from_graph(
        &graph,
        SimulationParameters::default(),
    ))
    .await;
}

fn add_layer(graph: &mut ForceGraph<()>, inner: [NodeIndex; 8]) -> [NodeIndex; 8] {
    let mut graph = graph;
    let new_cube = gen_cube(&mut graph);

    graph.add_edge(inner[0], new_cube[0], ());
    graph.add_edge(inner[1], new_cube[1], ());
    graph.add_edge(inner[2], new_cube[2], ());
    graph.add_edge(inner[3], new_cube[3], ());
    graph.add_edge(inner[4], new_cube[4], ());
    graph.add_edge(inner[5], new_cube[5], ());
    graph.add_edge(inner[6], new_cube[6], ());
    graph.add_edge(inner[7], new_cube[7], ());

    new_cube
}

fn gen_cube(graph: &mut ForceGraph<()>) -> [NodeIndex; 8] {
    let one = graph.add_force_node("", ());
    let two = graph.add_force_node("", ());
    let three = graph.add_force_node("", ());
    let four = graph.add_force_node("", ());
    let five = graph.add_force_node("", ());
    let six = graph.add_force_node("", ());
    let seven = graph.add_force_node("", ());
    let eight = graph.add_force_node("", ());

    graph.add_edge(one, two, ());
    graph.add_edge(two, three, ());
    graph.add_edge(three, four, ());
    graph.add_edge(four, one, ());
    graph.add_edge(five, six, ());
    graph.add_edge(six, seven, ());
    graph.add_edge(seven, eight, ());
    graph.add_edge(eight, five, ());

    graph.add_edge(one, five, ());
    graph.add_edge(two, six, ());
    graph.add_edge(three, seven, ());
    graph.add_edge(four, eight, ());

    [one, two, three, four, five, six, seven, eight]
}
