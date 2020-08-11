mod graph;
mod hash_graph;
mod node;
pub mod set;
pub mod transaction;
#[cfg(feature = "parsing")]
pub mod turtle;

pub use graph::Graph;
pub use hash_graph::HashGraph;
pub use node::Node;

#[cfg(test)]
struct Testbed {
    predicate_a: Node,
    predicate_b: Node,
    predicate_c: Node,

    node_a: Node,
    node_b: Node,
    node_c: Node,

    graph: hash_graph::HashGraph,
}

#[cfg(test)]
impl Testbed {
    fn new() -> Self {
        let predicate_a = Node::from("urn:arrf:tests:predicate:a");
        let predicate_b = Node::from("urn:arrf:tests:predicate:b");
        let predicate_c = Node::from("urn:arrf:tests:predicate:c");

        let node_a = Node::from("urn:arrf:tests:node:a");
        let node_b = Node::from("urn:arrf:tests:node:b");
        let node_c = Node::blank();

        let mut graph = hash_graph::HashGraph::new();
        graph.insert(node_a.clone(), predicate_a.clone(), node_b.clone());
        graph.insert(node_b.clone(), predicate_b.clone(), node_c.clone());
        graph.insert(node_c.clone(), predicate_c.clone(), node_a.clone());

        Self {
            predicate_a,
            predicate_b,
            predicate_c,

            node_a,
            node_b,
            node_c,

            graph,
        }
    }
}
