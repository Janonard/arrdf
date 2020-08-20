use crate::{Graph, Node};

pub struct Validator<G> {
    pub predicate_a: Node,
    pub predicate_b: Node,
    pub predicate_c: Node,

    pub node_a: Node,
    pub node_b: Node,
    pub node_c: Node,

    pub graph: G,
}

impl<G: Graph> Validator<G> {
    pub fn new(graph: G) -> Self {
        let predicate_a = Node::from("urn:arrf:tests:predicate:a");
        let predicate_b = Node::from("urn:arrf:tests:predicate:b");
        let predicate_c = Node::from("urn:arrf:tests:predicate:c");

        let node_a = Node::from("urn:arrf:tests:node:a");
        let node_b = Node::from("urn:arrf:tests:node:b");
        let node_c = Node::blank();

        let mut validator = Validator {
            predicate_a,
            predicate_b,
            predicate_c,

            node_a,
            node_b,
            node_c,

            graph,
        };

        validator.restore_graph();
        validator
    }

    fn restore_graph(&mut self) {
        self.graph.clear();
        assert!(self.graph.is_empty());
        self.graph
            .clone_insert(&self.node_a, &self.predicate_a, &self.node_b);
        self.graph
            .clone_insert(&self.node_b, &self.predicate_b, &self.node_c);
        self.graph
            .clone_insert(&self.node_c, &self.predicate_c, &self.node_a);
    }

    fn len(&mut self) {
        assert_eq!(3, self.graph.len());

        self.graph
            .clone_insert(&self.node_a, &self.predicate_a, &self.node_c);
        assert_eq!(4, self.graph.len());
    }

    fn is_empty(&mut self) {
        assert!(!self.graph.is_empty());

        self.graph
            .remove(&self.node_a, &self.predicate_a, &self.node_b);
        self.graph
            .remove(&self.node_b, &self.predicate_b, &self.node_c);
        self.graph
            .remove(&self.node_c, &self.predicate_c, &self.node_a);

        assert!(self.graph.is_empty());
    }

    fn iter(&self) {
        let iter: Vec<(&Node, &Node, &Node)> = self.graph.iter().collect();
        assert_eq!(3, iter.len());
        assert!(iter.contains(&(&self.node_a, &self.predicate_a, &self.node_b)));
        assert!(iter.contains(&(&self.node_b, &self.predicate_b, &self.node_c)));
        assert!(iter.contains(&(&self.node_c, &self.predicate_c, &self.node_a)));
    }

    fn is_valid_rdf(&mut self) {
        assert!(!self.graph.is_valid_graph());
        self.graph.sanitize();
        assert!(self.graph.is_valid_graph());
    }

    fn contains(&mut self) {
        assert!(self
            .graph
            .contains(&self.node_a, &self.predicate_a, &self.node_b));
        assert!(self
            .graph
            .contains(&self.node_b, &self.predicate_b, &self.node_c));
        assert!(self
            .graph
            .contains(&self.node_c, &self.predicate_c, &self.node_a));

        assert!(!self
            .graph
            .contains(&self.node_a, &self.predicate_b, &self.node_b));
    }

    fn remove(&mut self) {
        self.graph
            .remove(&self.node_c, &self.predicate_c, &self.node_a);

        let iter: Vec<(&Node, &Node, &Node)> = self.graph.iter().collect();
        assert_eq!(2, iter.len());
        assert!(iter.contains(&(&self.node_a, &self.predicate_a, &self.node_b)));
        assert!(iter.contains(&(&self.node_b, &self.predicate_b, &self.node_c)));
    }

    fn retain(&mut self) {
        self.graph.retain(|_, _, object| object.is_blank());

        let iter: Vec<(&Node, &Node, &Node)> = self.graph.iter().collect();
        assert_eq!(1, iter.len());
        assert!(iter.contains(&(&self.node_b, &self.predicate_b, &self.node_c)));
    }

    fn extend(&mut self) {
        self.graph.clone_extend(vec![
            (&self.node_a, &self.predicate_a, &self.node_b),
            (&self.node_b, &self.predicate_b, &self.node_c),
            (&self.node_c, &self.predicate_c, &self.node_a),
            (&self.node_a, &self.predicate_a, &self.node_c),
            (&self.node_b, &self.predicate_b, &self.node_a),
            (&self.node_c, &self.predicate_c, &self.node_b),
        ]);

        assert_eq!(6, self.graph.len());
        assert!(self
            .graph
            .contains(&self.node_a, &self.predicate_a, &self.node_b));
        assert!(self
            .graph
            .contains(&self.node_b, &self.predicate_b, &self.node_c));
        assert!(self
            .graph
            .contains(&self.node_c, &self.predicate_c, &self.node_a));
        assert!(self
            .graph
            .contains(&self.node_a, &self.predicate_a, &self.node_c));
        assert!(self
            .graph
            .contains(&self.node_b, &self.predicate_b, &self.node_a));
        assert!(self
            .graph
            .contains(&self.node_c, &self.predicate_c, &self.node_b));
    }

    fn duplicate_actions(&mut self) {
        let node_a = &self.node_a;
        let node_b = &self.node_b;
        let node_c = &self.node_c;
        let predicate_a = &self.predicate_a;
        let predicate_b = &self.predicate_b;
        let predicate_c = &self.predicate_c;

        assert_eq!(3, self.graph.len());
        assert!(self.graph.contains(node_a, predicate_a, node_b));
        assert!(self.graph.contains(node_b, predicate_b, node_c));
        assert!(self.graph.contains(node_c, predicate_c, node_a));

        self.graph.clone_insert(node_a, predicate_a, node_a);
        self.graph.clone_insert(node_a, predicate_a, node_a);

        assert_eq!(4, self.graph.len());
        assert!(self.graph.contains(node_a, predicate_a, node_b));
        assert!(self.graph.contains(node_b, predicate_b, node_c));
        assert!(self.graph.contains(node_c, predicate_c, node_a));
        assert!(self.graph.contains(node_a, predicate_a, node_a));

        self.graph.remove(node_a, predicate_a, node_a);
        self.graph.remove(node_a, predicate_a, node_a);

        assert_eq!(3, self.graph.len());
        assert!(!self.graph.contains(node_a, predicate_a, node_a));
        assert!(self.graph.contains(node_a, predicate_a, node_b));
        assert!(self.graph.contains(node_b, predicate_b, node_c));
        assert!(self.graph.contains(node_c, predicate_c, node_a));

        self.graph.remove(node_a, predicate_a, node_b);
        self.graph.remove(node_a, predicate_a, node_b);

        assert_eq!(2, self.graph.len());
        assert!(!self.graph.contains(node_a, predicate_a, node_a));
        assert!(!self.graph.contains(node_a, predicate_a, node_b));
        assert!(self.graph.contains(node_b, predicate_b, node_c));
        assert!(self.graph.contains(node_c, predicate_c, node_a));

        self.graph.clone_insert(node_a, predicate_a, node_b);
        self.graph.clone_insert(node_a, predicate_a, node_b);

        assert_eq!(3, self.graph.len());
        assert!(!self.graph.contains(node_a, predicate_a, node_a));
        assert!(self.graph.contains(node_a, predicate_a, node_b));
        assert!(self.graph.contains(node_b, predicate_b, node_c));
        assert!(self.graph.contains(node_c, predicate_c, node_a));
    }

    pub fn validate(&mut self) {
        self.len();
        self.restore_graph();
        self.is_empty();
        self.restore_graph();
        self.iter();
        self.restore_graph();
        self.is_valid_rdf();
        self.restore_graph();
        self.contains();
        self.restore_graph();
        self.remove();
        self.restore_graph();
        self.retain();
        self.restore_graph();
        self.extend();
        self.restore_graph();
        self.duplicate_actions();
    }
}
