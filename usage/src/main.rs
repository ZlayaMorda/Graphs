use graph_lib::dft::deserializer::DftDeserializer;
use graph_lib::dir_graph::graph::Graph;

fn main() {
    let mut deserialized_graph: Graph<u32, u32, String> = Graph::default();
    if let Err(e) = deserialized_graph.deserialize("graph.txt") {
        println!("Error while deserialize graph: {e}")
    }
    deserialized_graph.dfs(&1);
}
