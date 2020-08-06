#[derive(Clone, Debug, PartialEq, Eq)]
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
}

impl Default for Graph {
    fn default() -> Self {
        Self { nodes: Vec::new() }
    }
}

impl Graph {
    pub fn nodes(&self) -> impl Iterator<Item = &Node> {
        self.nodes.iter().map(|(node, _)| node)
    }

    fn find_node(&self, node: &Node) -> Option<usize> {
        for i in 0..self.nodes.len() {
            if *node == self.nodes[i].0 {
                return Some(i);
            }
        }
        None
    }

    fn node_impl(&self, referent: &str) -> Option<usize> {
        for i in 0..self.nodes.len() {
            let node_referent = self.nodes[i].0.referent().unwrap();
            if node_referent == referent {
                return Some(i);
            }
        }
        None
    }

    pub fn node<'a>(&'a self, referent: &str) -> Option<&'a Node> {
        self.node_impl(referent).map(|i| &self.nodes[i].0)
    }

    pub fn contains(&self, node: &Node) -> bool {
        self.find_node(node).is_some()
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
        self.find_node(subject).map(|i| self.relationships_impl(i))
    }

    pub fn triples(&self) -> impl Iterator<Item = (&Node, &Node, &Node)> {
        (0..self.nodes.len())
            .map(move |i| self.relationships_impl(i))
            .flatten()
    }

    fn add_node_impl(&mut self, new_node: Node) -> usize {
        self.find_node(&new_node).unwrap_or_else(|| {
            let node_index = self.nodes.len();
            self.nodes.push((new_node, Vec::new()));
            node_index
        })
    }

    pub fn add_node(&mut self, new_node: Node) {
        self.add_node_impl(new_node);
    }

    pub fn add_triple(&mut self, subject: Node, predicate: Node, object: Node) {
        let subject = self
            .find_node(&subject)
            .unwrap_or_else(|| self.add_node_impl(subject));
        let predicate = self
            .find_node(&predicate)
            .unwrap_or_else(|| self.add_node_impl(predicate));
        let object = self
            .find_node(&object)
            .unwrap_or_else(|| self.add_node_impl(object));

        self.nodes[subject].1.push((predicate, object));
    }

    fn remove_triple_impl(
        &mut self,
        subject: &Node,
        predicate: &Node,
        object: &Node,
    ) -> Option<()> {
        let subject = self.find_node(subject)?;
        let predicate = self.find_node(predicate)?;
        let object = self.find_node(object)?;

        self.nodes[subject]
            .1
            .retain(|(p, o)| *p != predicate || *o != object);

        Some(())
    }

    pub fn remove_triple(&mut self, subject: &Node, predicate: &Node, object: &Node) {
        self.remove_triple_impl(subject, predicate, object);
    }
}
