use crate::{HashGraph, Node};

impl HashGraph {
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

    pub fn is_valid_rdf(&self) -> bool {
        self.triples()
            .find(|(subject, predicate, _)| subject.is_blank() || predicate.is_blank())
            .is_none()
    }
}
