use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Node {
    Blank,
    Literal(String),
    IRI(String),
}

impl Node {
    pub fn referent(&self) -> Option<&str> {
        match self {
            Self::Blank => None,
            Self::Literal(literal) => Some(literal),
            Self::IRI(iri) => Some(iri),
        }
    }

    pub fn is_blank(&self) -> bool {
        match self {
            Self::Blank => true,
            _ => false,
        }
    }

    pub fn is_literal(&self) -> bool {
        match self {
            Self::Literal(_) => true,
            _ => false,
        }
    }

    pub fn is_iri(&self) -> bool {
        match self {
            Self::IRI(_) => true,
            _ => false,
        }
    }
}

pub struct Graph {
    triples: HashMap<usize, Vec<(usize, usize)>>,
    nodes: Vec<Node>,
}

impl Default for Graph {
    fn default() -> Self {
        Self {
            triples: HashMap::new(),
            nodes: Vec::new(),
        }
    }
}

impl Graph {
    pub fn nodes(&self) -> impl Iterator<Item = &Node> {
        self.nodes.iter()
    }

    fn node_index(&self, node: &Node) -> Option<usize> {
        for i in 0..self.nodes.len() {
            if *node == self.nodes[i] {
                return Some(i);
            }
        }
        None
    }

    fn node_impl(&self, referent: &str) -> Option<usize> {
        for i in 0..self.nodes.len() {
            let node_referent = self.nodes[i].referent().unwrap();
            if node_referent == referent {
                return Some(i);
            }
        }
        None
    }

    pub fn node<'a>(&'a self, referent: &str) -> Option<&'a Node> {
        self.node_impl(referent).map(|i| &self.nodes[i])
    }

    fn relationships_impl(&self, index: usize) -> impl Iterator<Item = (&Node, &Node, &Node)> {
        self.triples[&index].iter().map(move |(predicate, object)| {
            (
                &self.nodes[index],
                &self.nodes[*predicate],
                &self.nodes[*object],
            )
        })
    }

    pub fn relationships<'a>(
        &'a self,
        subject: &'a Node,
    ) -> Option<impl Iterator<Item = (&'a Node, &'a Node, &'a Node)>> {
        self.node_index(subject).map(|i| self.relationships_impl(i))
    }

    pub fn triples(&self) -> impl Iterator<Item = (&Node, &Node, &Node)> {
        (0..self.nodes.len())
            .map(move |i| self.relationships_impl(i))
            .flatten()
    }

    fn add_node_impl(&mut self, new_node: Node) -> usize {
        self.node_index(&new_node).unwrap_or_else(|| {
            let node_index = self.nodes.len();
            self.nodes.push(new_node);
            self.triples.insert(node_index, Vec::new());
            node_index
        })
    }

    pub fn add_node(&mut self, new_node: Node) {
        self.add_node_impl(new_node);
    }

    pub fn add_triple(&mut self, subject: Node, predicate: Node, object: Node) {
        let subject = self.node_index(&subject).unwrap_or_else(|| self.add_node_impl(subject));
        let predicate = self.node_index(&predicate).unwrap_or_else(|| self.add_node_impl(predicate));
        let object = self.node_index(&object).unwrap_or_else(|| self.add_node_impl(object));

        self.triples
            .get_mut(&subject)
            .unwrap()
            .push((predicate, object));
    }

    fn remove_triple_impl(&mut self, subject: &Node, predicate: &Node, object: &Node) -> Option<()> {
        let subject = self.node_index(subject)?;
        let predicate = self.node_index(predicate)?;
        let object = self.node_index(object)?;

        self.triples.get_mut(&subject)?.retain(|(p, o)| *p != predicate || *o != object);

        Some(())
    }

    pub fn remove_triple(&mut self, subject: &Node, predicate: &Node, object: &Node) {
        self.remove_triple_impl(subject, predicate, object);
    }
}
