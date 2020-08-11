use crate::Node;

pub trait Graph {
    fn len(&self) -> usize;

    #[cfg(not(tarpaulin_include))]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn contains(&self, subject: &Node, predicate: &Node, object: &Node) -> bool;

    fn iter<'a>(&'a self) -> Box<dyn 'a + Iterator<Item = (&'a Node, &'a Node, &'a Node)>>;

    fn is_valid_graph(&self) -> bool {
        self.iter().all(|(s, p, _)| !s.is_literal() && p.is_iri())
    }

    fn insert(&mut self, subject: Node, predicate: Node, object: Node);

    #[cfg(not(tarpaulin_include))]
    fn clone_insert(&mut self, subject: &Node, predicate: &Node, object: &Node) {
        self.insert(subject.clone(), predicate.clone(), object.clone());
    }

    fn extend<G>(&mut self, iter: G)
    where
        G: IntoIterator<Item = (Node, Node, Node)>,
    {
        for (s, p, o) in iter {
            self.insert(s.clone(), p.clone(), o.clone());
        }
    }

    fn clone_extend<'a, G>(&mut self, iter: G)
    where
        G: 'a + IntoIterator<Item = (&'a Node, &'a Node, &'a Node)>,
    {
        self.extend(
            iter.into_iter()
                .map(|(s, p, o)| (s.clone(), p.clone(), o.clone())),
        )
    }

    fn remove(&mut self, subject: &Node, predicate: &Node, object: &Node);

    #[cfg(not(tarpaulin_include))]
    fn remove_all<'a, G>(&mut self, iter: G)
    where
        G: IntoIterator<Item = (&'a Node, &'a Node, &'a Node)>,
    {
        for (s, p, o) in iter {
            self.remove(s, p, o);
        }
    }

    fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&Node, &Node, &Node) -> bool;

    fn sanitize(&mut self) {
        self.retain(|s, p, _| !s.is_literal() && p.is_iri());
    }
}
