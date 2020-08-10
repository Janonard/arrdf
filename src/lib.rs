mod node;
pub mod hash_graph;
pub mod set;

pub use node::Node;
pub use hash_graph::HashGraph;

pub trait Graph<'a> {
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn contains_triple(&self, subject: &Node, predicate: &Node, object: &Node) -> bool;

    type TripleIter: 'a + Iterator<Item = (&'a Node, &'a Node, &'a Node)>;

    fn triples(&'a self) -> Self::TripleIter;

    fn is_valid_graph(&'a self) -> bool {
        self.triples()
            .all(|(s, p, _)| !s.is_literal() && p.is_iri())
    }

    fn insert(&mut self, subject: Node, predicate: Node, object: Node);

    fn clone_insert(&mut self, subject: &Node, predicate: &Node, object: &Node) {
        self.insert(subject.clone(), predicate.clone(), object.clone());
    }

    fn extend<G>(&mut self, iter: G)
    where
        G: IntoIterator<Item = (Node, Node, Node)>,
    {
        for (s, p, o) in iter {
            self.insert(s.clone(), p.clone(), o.clone());
        }
    }

    fn remove(&mut self, subject: &Node, predicate: &Node, object: &Node);

    fn remove_all<G>(&mut self, iter: G)
    where
        G: IntoIterator<Item = (&'a Node, &'a Node, &'a Node)>,
    {
        for (s, p, o) in iter {
            self.remove(s, p, o);
        }
    }

    fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&Node, &Node, &Node) -> bool;

    fn sanitize(&mut self) {
        self.retain(|s, p, _| !s.is_literal() && p.is_iri());
    }
}

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
