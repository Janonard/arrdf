use crate::{Graph, Node};
use std::collections::{HashMap, HashSet};

#[cfg(test)]
mod tests;

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct HashGraph {
    nodes: HashMap<Node, HashSet<(Node, Node)>>,
}

impl HashGraph {
    pub fn new() -> Self {
        HashGraph {
            nodes: HashMap::new(),
        }
    }

    #[cfg(not(tarpaulin_include))]
    pub fn with_capacity(n_subjects: usize) -> Self {
        HashGraph {
            nodes: HashMap::with_capacity(n_subjects),
        }
    }

    #[cfg(not(tarpaulin_include))]
    pub fn shrink_to_fit(&mut self) {
        self.nodes.retain(|_, relationships| {
            relationships.shrink_to_fit();
            !relationships.is_empty()
        })
    }
}

pub struct TripleIter<'a> {
    internal: Box<dyn 'a + Iterator<Item = (&'a Node, &'a Node, &'a Node)>>,
}

impl<'a> TripleIter<'a> {
    fn new(graph: &'a HashGraph) -> Self {
        Self {
            internal: Box::new(
                graph
                    .nodes
                    .iter()
                    .map(|(s, rels)| rels.iter().map(move |(p, o)| (s, p, o)))
                    .flatten(),
            ),
        }
    }
}

impl<'a> Iterator for TripleIter<'a> {
    type Item = (&'a Node, &'a Node, &'a Node);

    fn next(&mut self) -> Option<Self::Item> {
        self.internal.next()
    }
}

impl<'a> Graph<'a> for HashGraph {
    fn len(&self) -> usize {
        self.nodes.iter().map(|(_, r)| r.len()).sum()
    }

    fn is_empty(&self) -> bool {
        if self.nodes.is_empty() {
            true
        } else {
            self.nodes.iter().all(|(_, rels)| rels.is_empty())
        }
    }

    fn contains_triple(&self, subject: &Node, predicate: &Node, object: &Node) -> bool {
        if let Some(relationships) = self.nodes.get(subject) {
            relationships.contains(&(predicate.clone(), object.clone()))
        } else {
            false
        }
    }

    type TripleIter = TripleIter<'a>;

    fn triples(&'a self) -> TripleIter<'a> {
        TripleIter::new(self)
    }

    fn insert(&mut self, subject: Node, predicate: Node, object: Node) {
        self.nodes
            .entry(subject)
            .or_insert_with(HashSet::new)
            .insert((predicate, object));
    }

    fn remove(&mut self, subject: &Node, predicate: &Node, object: &Node) {
        if let Some(relationships) = self.nodes.get_mut(subject) {
            relationships.retain(|(p, o)| p != predicate || o != object);
        }
    }

    fn retain<F: FnMut(&Node, &Node, &Node) -> bool>(&mut self, mut f: F) {
        for (subject, relationships) in self.nodes.iter_mut() {
            relationships.retain(|(predicate, object)| f(subject, predicate, object));
        }
    }
}

impl std::iter::FromIterator<(Node, Node, Node)> for HashGraph {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (Node, Node, Node)>,
    {
        let mut graph = HashGraph::new();
        graph.extend(iter);
        graph
    }
}

impl<'a> std::iter::FromIterator<(&'a Node, &'a Node, &'a Node)> for HashGraph {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (&'a Node, &'a Node, &'a Node)>,
    {
        let mut graph = HashGraph::new();
        graph.extend(
            iter.into_iter()
                .map(|(s, p, o)| (s.clone(), p.clone(), o.clone())),
        );
        graph
    }
}