use crate::{Graph, Node};

pub struct Testbed<G> {
    pub predicate_a: Node,
    pub predicate_b: Node,
    pub predicate_c: Node,

    pub node_a: Node,
    pub node_b: Node,
    pub node_c: Node,

    prototype: G,
}

impl<G: Graph + Clone> Testbed<G> {
    pub fn new(mut prototype: G) -> Self {
        let predicate_a = Node::from("urn:arrf:tests:predicate:a");
        let predicate_b = Node::from("urn:arrf:tests:predicate:b");
        let predicate_c = Node::from("urn:arrf:tests:predicate:c");

        let node_a = Node::from("urn:arrf:tests:node:a");
        let node_b = Node::from("urn:arrf:tests:node:b");
        let node_c = Node::blank();

        assert!(prototype.is_empty());
        prototype.insert(node_a.clone(), predicate_a.clone(), node_b.clone());
        prototype.insert(node_b.clone(), predicate_b.clone(), node_c.clone());
        prototype.insert(node_c.clone(), predicate_c.clone(), node_a.clone());

        Self {
            predicate_a,
            predicate_b,
            predicate_c,

            node_a,
            node_b,
            node_c,

            prototype,
        }
    }

    pub fn graph(&self) -> G {
        self.prototype.clone()
    }

    fn len(&self) {
        let mut graph = self.graph();
        assert_eq!(3, graph.len());

        graph.clone_insert(&self.node_a, &self.predicate_a, &self.node_c);
        assert_eq!(4, graph.len());
    }

    fn is_empty(&self) {
        let mut graph = self.graph();
        assert!(!graph.is_empty());

        graph.remove(&self.node_a, &self.predicate_a, &self.node_b);
        graph.remove(&self.node_b, &self.predicate_b, &self.node_c);
        graph.remove(&self.node_c, &self.predicate_c, &self.node_a);

        assert!(graph.is_empty());
    }

    fn iter(&self) {
        let graph = self.graph();
        let iter: Vec<(&Node, &Node, &Node)> = graph.iter().collect();
        assert_eq!(3, iter.len());
        assert!(iter.contains(&(&self.node_a, &self.predicate_a, &self.node_b)));
        assert!(iter.contains(&(&self.node_b, &self.predicate_b, &self.node_c)));
        assert!(iter.contains(&(&self.node_c, &self.predicate_c, &self.node_a)));
    }

    fn is_valid_rdf(&self) {
        let mut graph = self.graph();
        assert!(!graph.is_valid_graph());
        graph.sanitize();
        assert!(graph.is_valid_graph());
    }

    fn contains(&self) {
        let graph = self.graph();
        assert!(graph.contains(&self.node_a, &self.predicate_a, &self.node_b));
        assert!(graph.contains(&self.node_b, &self.predicate_b, &self.node_c));
        assert!(graph.contains(&self.node_c, &self.predicate_c, &self.node_a));

        assert!(!graph.contains(&self.node_a, &self.predicate_b, &self.node_b));
    }

    fn remove(&self) {
        let mut graph = self.graph();
        graph.remove(&self.node_c, &self.predicate_c, &self.node_a);

        let iter: Vec<(&Node, &Node, &Node)> = graph.iter().collect();
        assert_eq!(2, iter.len());
        assert!(iter.contains(&(&self.node_a, &self.predicate_a, &self.node_b)));
        assert!(iter.contains(&(&self.node_b, &self.predicate_b, &self.node_c)));
    }

    fn retain(&self) {
        let mut graph = self.graph();
        graph.retain(|_, _, object| object.is_blank());

        let iter: Vec<(&Node, &Node, &Node)> = graph.iter().collect();
        assert_eq!(1, iter.len());
        assert!(iter.contains(&(&self.node_b, &self.predicate_b, &self.node_c)));
    }

    pub fn validate(&self) {
        self.len();
        self.is_empty();
        self.iter();
        self.is_valid_rdf();
        self.contains();
        self.remove();
        self.retain();
    }
}
