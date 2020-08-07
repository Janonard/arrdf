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
