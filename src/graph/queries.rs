use crate::{HashGraph, Node};

impl HashGraph {
    pub fn len(&self) -> usize {
        self.nodes.iter().map(|(_, r)| r.len()).sum()
    }

    pub fn contains_subject(&self, node: &Node) -> bool {
        self.nodes.contains_key(node)
    }

    pub fn subjects(&self) -> impl Iterator<Item = &Node> {
        self.nodes.keys()
    }

    pub fn relationships<'a, 'b>(
        &'a self,
        subject: &'a Node,
    ) -> impl Iterator<Item = (&'a Node, &'a Node, &'a Node)> {
        self.nodes
            .get(subject)
            .into_iter()
            .map(|relationships| relationships.iter())
            .flatten()
            .map(move |(predicate, object)| (subject, predicate, object))
    }

    pub fn contains_triple(&self, subject: &Node, predicate: &Node, object: &Node) -> bool {
        if let Some(relationships) = self.nodes.get(subject) {
            relationships.contains(&(predicate.clone(), object.clone()))
        } else {
            false
        }
    }

    pub fn triples(&self) -> impl Iterator<Item = (&Node, &Node, &Node)> {
        self.nodes
            .iter()
            .map(|(subject, relationships)| {
                relationships
                    .iter()
                    .map(move |(predicate, object)| (subject, predicate, object))
            })
            .flatten()
    }

    pub fn is_valid_graph(&self) -> bool {
        self.triples()
            .find(|(subject, predicate, _)| subject.is_literal() || !predicate.is_iri())
            .is_none()
    }

    pub fn difference<'a>(
        &'a self,
        other: &'a HashGraph,
    ) -> impl 'a + Iterator<Item = (&Node, &Node, &Node)> {
        self.triples()
            .filter(move |(s, p, o)| !other.contains_triple(s, p, o))
    }

    pub fn symmetric_difference<'a>(
        &'a self,
        other: &'a HashGraph,
    ) -> impl 'a + Iterator<Item = (&Node, &Node, &Node)> {
        self.difference(other).chain(other.difference(self))
    }

    pub fn intersection<'a>(
        &'a self,
        other: &'a HashGraph,
    ) -> impl 'a + Iterator<Item = (&Node, &Node, &Node)> {
        self.triples()
            .filter(move |(s, p, o)| other.contains_triple(s, p, o))
    }

    pub fn union<'a>(
        &'a self,
        other: &'a HashGraph,
    ) -> impl 'a + Iterator<Item = (&Node, &Node, &Node)> {
        self.triples().chain(other.difference(self))
    }

    pub fn is_subset(&self, other: &HashGraph) -> bool {
        self.triples()
            .all(|(s, p, o)| other.contains_triple(s, p, o))
    }

    pub fn is_superset(&self, other: &HashGraph) -> bool {
        other.is_subset(self)
    }

    pub fn is_disjoint(&self, other: &HashGraph) -> bool {
        self.intersection(other).next().is_none()
    }
}
