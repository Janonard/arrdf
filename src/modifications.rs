use crate::{HashGraph, Node};
use std::collections::HashSet;

impl HashGraph {
    pub fn insert(&mut self, subject: Node, predicate: Node, object: Node) {
        self.nodes
            .entry(subject)
            .or_insert_with(|| HashSet::new())
            .insert((predicate, object));
    }

    pub fn remove(&mut self, subject: &Node, predicate: &Node, object: &Node) {
        if let Some(relationships) = self.nodes.get_mut(subject) {
            relationships.retain(|(p, o)| p != predicate || o != object);
        }
    }

    pub fn retain<F: FnMut(&Node, &Node, &Node) -> bool>(&mut self, mut f: F) {
        for (subject, relationships) in self.nodes.iter_mut() {
            relationships.retain(|(predicate, object)| f(subject, predicate, object));
        }
    }

    pub fn sanitize(&mut self) {
        self.retain(|subject, predicate, _| !subject.is_blank() && !predicate.is_blank())
    }
}
