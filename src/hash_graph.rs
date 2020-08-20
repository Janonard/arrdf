use crate::{Graph, Node};
use std::collections::{HashMap, HashSet};

/// A canonical implementation of the `Graph` trait.
///
/// The `HashGraph` implements the `Graph` trait with a hierarchy of `HashMap`s and a `HashSet` to
/// efficiently support path traversals and containment queries. If you simply want to use a `Graph`,
/// use this one.
///
/// Check out the [crate-level introduction](index.html) for some examples.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct HashGraph {
    nodes: HashMap<Node, HashMap<Node, HashSet<Node>>>,
}

impl HashGraph {
    /// Create a new, empty graph.
    pub fn new() -> Self {
        HashGraph {
            nodes: HashMap::new(),
        }
    }
}

impl Graph for HashGraph {
    fn len(&self) -> usize {
        self.nodes
            .values()
            .map(|relationships| {
                relationships
                    .values()
                    .map(|objects| objects.len())
                    .sum::<usize>()
            })
            .sum::<usize>()
    }

    fn is_empty(&self) -> bool {
        self.nodes
            .values()
            .all(|rels| rels.values().all(|objects| objects.is_empty()))
    }

    fn contains(&self, subject: &Node, predicate: &Node, object: &Node) -> bool {
        self.nodes
            .get(subject)
            .and_then(|r| r.get(predicate))
            .map(|o| o.contains(object))
            .unwrap_or(false)
    }

    fn iter<'a>(&'a self) -> Box<dyn 'a + Iterator<Item = (&'a Node, &'a Node, &'a Node)>> {
        let relationships = self
            .nodes
            .iter()
            .map(|(subject, relationships)| {
                relationships
                    .iter()
                    .map(move |(predicate, objects)| {
                        objects
                            .iter()
                            .map(move |object| (subject, predicate, object))
                    })
                    .flatten()
            })
            .flatten();
        Box::new(relationships)
    }

    fn insert(&mut self, subject: Node, predicate: Node, object: Node) {
        self.nodes
            .entry(subject)
            .or_insert_with(HashMap::new)
            .entry(predicate)
            .or_insert_with(HashSet::new)
            .insert(object);
    }

    fn remove(&mut self, subject: &Node, predicate: &Node, object: &Node) {
        let objects = self
            .nodes
            .get_mut(subject)
            .and_then(|relationships| relationships.get_mut(predicate));
        if let Some(objects) = objects {
            objects.remove(object);
        }
    }

    fn retain<F: FnMut(&Node, &Node, &Node) -> bool>(&mut self, mut f: F) {
        for (subject, relationships) in self.nodes.iter_mut() {
            for (predicate, objects) in relationships.iter_mut() {
                objects.retain(|object| f(subject, predicate, object));
            }
        }
    }

    fn clear(&mut self) {
        self.nodes.clear();
    }

    fn relationships<'a>(
        &'a self,
        subject: &'a Node,
    ) -> Box<dyn 'a + Iterator<Item = (&Node, &Node, &Node)>> {
        if let Some(relationships) = self.nodes.get(subject) {
            let iter = relationships
                .iter()
                .map(|(predicate, objects)| objects.iter().map(move |object| (predicate, object)))
                .flatten();
            let iter = iter.map(move |(predicate, object)| (subject, predicate, object));
            Box::new(iter)
        } else {
            Box::new(std::iter::empty())
        }
    }

    fn objects<'a>(
        &'a self,
        subject: &'a Node,
        predicate: &'a Node,
    ) -> Box<dyn 'a + Iterator<Item = (&Node, &Node, &Node)>> {
        if let Some(objects) = self
            .nodes
            .get(subject)
            .and_then(|relationships| relationships.get(predicate))
        {
            Box::new(
                objects
                    .iter()
                    .map(move |object| (subject, predicate, object)),
            )
        } else {
            Box::new(std::iter::empty())
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
        iter.into_iter()
            .map(|(s, p, o)| (s.clone(), p.clone(), o.clone()))
            .collect()
    }
}

impl std::iter::IntoIterator for HashGraph {
    type Item = (Node, Node, Node);
    type IntoIter = Box<dyn Iterator<Item = (Node, Node, Node)>>;

    fn into_iter(self) -> Self::IntoIter {
        let relationships = self
            .nodes
            .into_iter()
            .map(|(subject, relationships)| {
                relationships
                    .into_iter()
                    .map(|(predicate, objects)| {
                        objects
                            .into_iter()
                            .map(move |object| (predicate.clone(), object))
                    })
                    .flatten()
                    .map(move |(predicate, object)| (subject.clone(), predicate, object))
            })
            .flatten();
        Box::new(relationships)
    }
}

#[test]
#[cfg(test)]
fn validate() {
    let mut validator = crate::Validator::new(HashGraph::new());
    validator.validate();
}
