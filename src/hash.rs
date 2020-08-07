use crate::{Graph, Node};
use std::collections::{HashMap, HashSet};

pub struct HashGraph {
    nodes: HashMap<Node, HashSet<(Node, Node)>>,
}

impl Default for HashGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl HashGraph {
    pub fn new() -> Self {
        HashGraph {
            nodes: HashMap::new(),
        }
    }

    pub fn with_capacity(n_subjects: usize) -> Self {
        HashGraph {
            nodes: HashMap::with_capacity(n_subjects),
        }
    }

    pub fn contains_subject(&self, node: &Node) -> bool {
        self.nodes.contains_key(node)
    }

    pub fn subjects(&self) -> impl Iterator<Item = &Node> {
        self.nodes.keys()
    }

    pub fn relationships<'a>(
        &'a self,
        subject: &'a Node,
    ) -> Option<impl 'a + Iterator<Item = (&Node, &Node, &Node)>> {
        Some(
            self.nodes
                .get(subject)?
                .iter()
                .map(move |(predicate, object)| (subject, predicate, object)),
        )
    }
}

impl<'a> Graph<'a> for HashGraph {
    type TripleIter = HashTripleIter<'a>;
    fn triples(&'a self) -> HashTripleIter<'a> {
        HashTripleIter::new(self)
    }

    fn contains_triple(&self, subject: &Node, predicate: &Node, object: &Node) -> bool {
        if let Some(relationships) = self.nodes.get(subject) {
            relationships.contains(&(predicate.clone(), object.clone()))
        } else {
            false
        }
    }

    fn add_triple(&mut self, subject: Node, predicate: Node, object: Node) {
        self.nodes
            .entry(subject)
            .or_insert_with(|| HashSet::new())
            .insert((predicate, object));
    }

    fn remove_triple(&mut self, subject: &Node, predicate: &Node, object: &Node) {
        if let Some(relationships) = self.nodes.get_mut(subject) {
            relationships.retain(|(p, o)| p != predicate || o != object);
        }
    }
}

pub struct HashTripleIter<'a> {
    graph: &'a HashGraph,
    keys: std::collections::hash_map::Keys<'a, Node, HashSet<(Node, Node)>>,
    relationships: Option<(&'a Node, std::collections::hash_set::Iter<'a, (Node, Node)>)>,
}

impl<'a> HashTripleIter<'a> {
    fn new(graph: &'a HashGraph) -> Self {
        let mut iter = Self {
            graph,
            keys: graph.nodes.keys(),
            relationships: None,
        };
        iter.next_subject();
        iter
    }

    fn next_subject(&mut self) {
        self.relationships = self
            .keys
            .next()
            .map(|subject| (subject, self.graph.nodes[subject].iter()));
    }
}

impl<'a> Iterator for HashTripleIter<'a> {
    type Item = (&'a Node, &'a Node, &'a Node);

    fn next(&mut self) -> Option<(&'a Node, &'a Node, &'a Node)> {
        let mut next_value: Option<(&'a Node, &'a Node, &'a Node)> = None;
        while self.relationships.is_some() && next_value.is_none() {
            let (subject, rels) = self.relationships.as_mut().unwrap();
            if let Some((predicate, object)) = rels.next() {
                next_value = Some((subject, predicate, object));
            } else {
                self.next_subject();
            }
        }
        next_value
    }
}

#[cfg(test)]
mod tests {
    use crate::hash::HashGraph;
    use crate::*;

    struct Testbed {
        predicate_a: Node,
        predicate_b: Node,
        predicate_c: Node,

        node_a: Node,
        node_b: Node,
        node_c: Node,

        graph: HashGraph,
    }

    impl Testbed {
        fn new() -> Self {
            let predicate_a = Node::from("urn:arrf:tests:predicate:a");
            let predicate_b = Node::from("urn:arrf:tests:predicate:b");
            let predicate_c = Node::from("urn:arrf:tests:predicate:c");

            let node_a = Node::from("urn:arrf:tests:node:a");
            let node_b = Node::from("urn:arrf:tests:node:b");
            let node_c = Node::blank();

            let mut graph = HashGraph::default();
            graph.add_triple(node_a.clone(), predicate_a.clone(), node_b.clone());
            graph.add_triple(node_b.clone(), predicate_b.clone(), node_c.clone());
            graph.add_triple(node_c.clone(), predicate_c.clone(), node_a.clone());

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

    #[test]
    fn contains_subject() {
        let testbed = Testbed::new();

        assert!(testbed.graph.contains_subject(&testbed.node_a));
        assert!(testbed.graph.contains_subject(&testbed.node_b));
        assert!(testbed.graph.contains_subject(&testbed.node_c));

        assert!(!testbed.graph.contains_subject(&testbed.predicate_a));
        assert!(!testbed.graph.contains_subject(&testbed.predicate_b));
        assert!(!testbed.graph.contains_subject(&testbed.predicate_c));
    }

    #[test]
    fn subject() {
        let testbed = Testbed::new();

        let subjects: Vec<Node> = testbed.graph.subjects().cloned().collect();

        assert_eq!(3, subjects.len());

        assert!(subjects.contains(&testbed.node_a));
        assert!(subjects.contains(&testbed.node_b));
        assert!(subjects.contains(&testbed.node_c));

        assert!(!subjects.contains(&testbed.predicate_a));
        assert!(!subjects.contains(&testbed.predicate_b));
        assert!(!subjects.contains(&testbed.predicate_c));
    }

    #[test]
    fn relationships() {
        let testbed = Testbed::new();

        let relationships: Vec<(&Node, &Node, &Node)>;
        relationships = testbed
            .graph
            .relationships(&testbed.node_a)
            .unwrap()
            .collect();

        assert_eq!(1, relationships.len());
        assert!(relationships.contains(&(&testbed.node_a, &testbed.predicate_a, &testbed.node_b)));

        let relationships: Vec<(&Node, &Node, &Node)>;
        relationships = testbed
            .graph
            .relationships(&testbed.node_b)
            .unwrap()
            .collect();

        assert_eq!(1, relationships.len());
        assert!(relationships.contains(&(&testbed.node_b, &testbed.predicate_b, &testbed.node_c)));

        let relationships: Vec<(&Node, &Node, &Node)>;
        relationships = testbed
            .graph
            .relationships(&testbed.node_c)
            .unwrap()
            .collect();

        assert_eq!(1, relationships.len());
        assert!(relationships.contains(&(&testbed.node_c, &testbed.predicate_c, &testbed.node_a)));

        assert!(testbed.graph.relationships(&testbed.predicate_a).is_none());
        assert!(testbed.graph.relationships(&testbed.predicate_b).is_none());
        assert!(testbed.graph.relationships(&testbed.predicate_c).is_none());
    }

    #[test]
    fn triples() {
        let testbed = Testbed::new();

        let triples: Vec<(&Node, &Node, &Node)> = testbed.graph.triples().collect();
        assert_eq!(3, triples.len());
        assert!(triples.contains(&(&testbed.node_a, &testbed.predicate_a, &testbed.node_b)));
        assert!(triples.contains(&(&testbed.node_b, &testbed.predicate_b, &testbed.node_c)));
        assert!(triples.contains(&(&testbed.node_c, &testbed.predicate_c, &testbed.node_a)));
    }

    #[test]
    fn contains_triple() {
        let testbed = Testbed::new();

        assert!(testbed.graph.contains_triple(
            &testbed.node_a,
            &testbed.predicate_a,
            &testbed.node_b
        ));
        assert!(testbed.graph.contains_triple(
            &testbed.node_b,
            &testbed.predicate_b,
            &testbed.node_c
        ));
        assert!(testbed.graph.contains_triple(
            &testbed.node_c,
            &testbed.predicate_c,
            &testbed.node_a
        ));
    }

    #[test]
    fn remove_triple() {
        let mut testbed = Testbed::new();

        testbed
            .graph
            .remove_triple(&testbed.node_c, &testbed.predicate_c, &testbed.node_a);

        let triples: Vec<(&Node, &Node, &Node)> = testbed.graph.triples().collect();
        assert_eq!(2, triples.len());
        assert!(triples.contains(&(&testbed.node_a, &testbed.predicate_a, &testbed.node_b)));
        assert!(triples.contains(&(&testbed.node_b, &testbed.predicate_b, &testbed.node_c)));
    }
}
