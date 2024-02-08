use pointer_dirgraph::dft::deserializer::DftDeserializer;
use pointer_dirgraph::graph::Graph;

fn main() {
    let mut deserialized_graph: Graph<u32, u32, String> = Graph::default();
    if let Err(e) = deserialized_graph.deserialize("graph.txt") {
        println!("Error while deserialize graph: {e}")
    }
    deserialized_graph.dfs(&1);
}
