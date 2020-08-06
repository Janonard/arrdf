use crate::Node;
use std::collections::{HashMap, HashSet};

pub struct Graph {
    nodes: HashMap<Node, HashSet<(Node, Node)>>,
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn with_capacity(n_subjects: usize) -> Self {
        Self {
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

    pub fn triples(&self) -> impl Iterator<Item = (&Node, &Node, &Node)> {
        self.subjects()
            .map(move |subject| self.relationships(subject).unwrap())
            .flatten()
    }

    pub fn add_triple(&mut self, subject: Node, predicate: Node, object: Node) {
        self.nodes
            .entry(subject)
            .or_insert_with(|| HashSet::new())
            .insert((predicate, object));
    }

    pub fn remove_triple(&mut self, subject: &Node, predicate: &Node, object: &Node) {
        if let Some(relationships) = self.nodes.get_mut(subject) {
            relationships.retain(|(p, o)| p != predicate || o != object);
        }
    }
}
