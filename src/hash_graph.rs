use crate::{Graph, Node};
use std::collections::{HashMap, HashSet};

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct HashGraph {
    nodes: HashMap<Node, HashMap<Node, HashSet<Node>>>,
}

impl HashGraph {
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
    let validator = crate::Testbed::new(HashGraph::new());
    validator.validate();
}
