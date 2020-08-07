use crate::*;

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
        graph.extend(iter);
        graph
    }
}
