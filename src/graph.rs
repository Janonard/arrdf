use crate::Node;
use std::collections::HashMap;

pub struct Graph {
    nodes: HashMap<Node, Vec<(Node, Node)>>,
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

    pub fn with_capacity(n_nodes: usize) -> Self {
        Self {
            nodes: HashMap::with_capacity(n_nodes),
        }
    }

    pub fn nodes(&self) -> impl Iterator<Item = &Node> {
        self.nodes.keys()
    }

    pub fn contains(&self, node: &Node) -> bool {
        self.nodes.contains_key(node)
    }

    pub fn relationships<'a>(
        &'a self,
        subject: &Node,
    ) -> Option<impl 'a + Iterator<Item = (Node, Node, Node)>> {
        let subject = subject.clone();
        Some(
            self.nodes
                .get(&subject)?
                .iter()
                .map(move |(predicate, object)| {
                    (subject.clone(), predicate.clone(), object.clone())
                }),
        )
    }

    pub fn triples<'a>(&'a self) -> impl 'a + Iterator<Item = (Node, Node, Node)> {
        self.nodes()
            .map(move |subject| self.relationships(subject).unwrap())
            .flatten()
    }

    fn add_node(&mut self, new_node: Node) -> Node {
        if self.nodes.contains_key(&new_node) {
            new_node
        } else {
            self.nodes.insert(new_node.clone(), Vec::new());
            new_node
        }
    }

    pub fn add_triple(&mut self, subject: Node, predicate: Node, object: Node) {
        let subject = self.add_node(subject);
        let predicate = self.add_node(predicate);
        let object = self.add_node(object);

        self.nodes
            .get_mut(&subject)
            .unwrap()
            .push((predicate, object));
    }

    pub fn remove_triple(&mut self, subject: &Node, predicate: &Node, object: &Node) {
        if let Some(relationships) = self.nodes.get_mut(subject) {
            relationships.retain(|(p, o)| p != predicate || o != object)
        }
    }
}
