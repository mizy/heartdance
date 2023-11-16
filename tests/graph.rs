use heartdance::{algo, graph::Graph};
use rand::Rng;

#[test]
fn test_graph_init() {
    let mut graph = Graph::new();
    let vertexs = ['a', 'b', 'c', 'd'];
    for vertex in vertexs.iter() {
        graph.add_vertex(vertex.to_string(), 0., 0., 0.);
    }

    let edges = [('a', 'b'), ('b', 'c'), ('c', 'd'), ('d', 'a')];
    let mut rng = rand::thread_rng();
    for edge in edges.iter() {
        graph.add_edge(edge.0.to_string(), edge.1.to_string(), rng.gen());
    }
    graph.calc_degree();
    graph.set_directed(true);
    let ptr = graph.get_nodes_position_ptr();
    print!("ptr: {:p}", ptr);
}
