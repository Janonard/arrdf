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
    nodes: Vec<(Node, Vec<(usize, usize)>)>,
    node_lut: Option<HashMap<Node, usize>>,
}

impl Default for Graph {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            node_lut: None,
        }
    }
}

impl Graph {
    pub fn with_node_lut() -> Self {
        Self {
            nodes: Vec::new(),
            node_lut: Some(HashMap::new()),
        }
    }

    pub fn add_node_lut(&mut self) {
        if self.node_lut.is_some() {
            return;
        }

        let mut node_lut = HashMap::with_capacity(self.nodes.len());
        for i in 0..self.nodes.len() {
            node_lut.insert(self.nodes[i].0.clone(), i);
        }

        self.node_lut = Some(node_lut);
    }

    pub fn nodes(&self) -> impl Iterator<Item = &Node> {
        self.nodes.iter().map(|(node, _)| node)
    }

    fn node_index(&self, node: &Node) -> Option<usize> {
        match &self.node_lut {
            Some(lut) => lut.get(node).cloned(),
            None => {
                for i in 0..self.nodes.len() {
                    if *node == self.nodes[i].0 {
                        return Some(i);
                    }
                }
                None
            }
        }
    }

    pub fn contains(&self, node: &Node) -> bool {
        match &self.node_lut {
            Some(lut) => lut.contains_key(node),
            None => self.nodes.iter().find(|contained_node| contained_node.0 == *node).is_some(),
        }
    }

    fn relationships_impl(&self, index: usize) -> impl Iterator<Item = (&Node, &Node, &Node)> {
        self.nodes[index].1.iter().map(move |(predicate, object)| {
            (
                &self.nodes[index].0,
                &self.nodes[*predicate].0,
                &self.nodes[*object].0,
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

            if let Some(lut) = &mut self.node_lut {
                lut.insert(new_node.clone(), node_index);
            }
            self.nodes.push((new_node, Vec::new()));

            node_index
        })
    }

    pub fn add_node(&mut self, new_node: Node) {
        self.add_node_impl(new_node);
    }

    pub fn add_triple(&mut self, subject: Node, predicate: Node, object: Node) {
        let subject = self
            .node_index(&subject)
            .unwrap_or_else(|| self.add_node_impl(subject));
        let predicate = self
            .node_index(&predicate)
            .unwrap_or_else(|| self.add_node_impl(predicate));
        let object = self
            .node_index(&object)
            .unwrap_or_else(|| self.add_node_impl(object));

        self.nodes[subject].1.push((predicate, object));
    }

    fn remove_triple_impl(
        &mut self,
        subject: &Node,
        predicate: &Node,
        object: &Node,
    ) -> Option<()> {
        let subject = self.node_index(subject)?;
        let predicate = self.node_index(predicate)?;
        let object = self.node_index(object)?;

        self.nodes[subject]
            .1
            .retain(|(p, o)| *p != predicate || *o != object);

        Some(())
    }

    pub fn remove_triple(&mut self, subject: &Node, predicate: &Node, object: &Node) {
        self.remove_triple_impl(subject, predicate, object);
    }
}
