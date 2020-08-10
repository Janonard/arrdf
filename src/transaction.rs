use crate::{set, Graph, HashGraph, Node};
use std::ops::Deref;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub struct TransactionGraph<G: Graph> {
    graph: Arc<RwLock<IntTransactionGraph<G>>>,
}

impl<G: Graph> TransactionGraph<G> {
    pub fn new(graph: G) -> Self {
        Self {
            graph: Arc::new(RwLock::new(IntTransactionGraph::new(graph))),
        }
    }

    pub fn transaction(&self) -> Option<Transaction<G>> {
        self.graph.read().map(|guard| Transaction::new(guard)).ok()
    }

    pub fn try_transaction(&self) -> Option<Transaction<G>> {
        self.graph
            .try_read()
            .map(|guard| Transaction { guard })
            .ok()
    }

    pub fn mut_transaction(&mut self) -> Option<MutTransaction<G>> {
        self.graph
            .write()
            .map(|guard| MutTransaction::new(guard))
            .ok()
    }

    pub fn try_mut_transaction(&mut self) -> Option<MutTransaction<G>> {
        self.graph
            .try_write()
            .map(|guard| MutTransaction::new(guard))
            .ok()
    }
}

struct IntTransactionGraph<G: Graph> {
    graph: G,
}

impl<G: Graph> IntTransactionGraph<G> {
    pub fn new(graph: G) -> Self {
        Self { graph }
    }
}

pub struct Transaction<'a, G: Graph> {
    guard: RwLockReadGuard<'a, IntTransactionGraph<G>>,
}

impl<'a, G: Graph> Transaction<'a, G> {
    fn new(guard: RwLockReadGuard<'a, IntTransactionGraph<G>>) -> Self {
        Self { guard }
    }
}

impl<'a, G: Graph> Deref for Transaction<'a, G> {
    type Target = G;

    fn deref(&self) -> &G {
        &self.guard.graph
    }
}

pub struct MutTransaction<'a, G: Graph> {
    guard: RwLockWriteGuard<'a, IntTransactionGraph<G>>,
    added_triples: HashGraph,
    removed_triples: HashGraph,
}

impl<'a, G: Graph> MutTransaction<'a, G> {
    fn new(guard: RwLockWriteGuard<'a, IntTransactionGraph<G>>) -> Self {
        Self {
            guard,
            added_triples: HashGraph::new(),
            removed_triples: HashGraph::new(),
        }
    }

    fn is_valid(&self) -> bool {
        set::is_subset(&self.removed_triples, &self.guard.graph) &&
        set::is_disjoint(&self.guard.graph, &self.added_triples)
    }

    pub fn commit(mut self) {
        if cfg!(test) {
            assert!(self.is_valid());
        }

        self.guard.graph.remove_all(self.removed_triples.triples());
        self.guard.graph.clone_extend(self.added_triples.triples());
    }
}

impl<'a, G: Graph> Graph for MutTransaction<'a, G> {
    fn len(&self) -> usize {
        self.guard.graph.len() + self.added_triples.len() - self.removed_triples.len()
    }

    fn contains_triple(&self, subject: &Node, predicate: &Node, object: &Node) -> bool {
        if self
            .added_triples
            .contains_triple(subject, predicate, object)
        {
            true
        } else if self
            .removed_triples
            .contains_triple(subject, predicate, object)
        {
            false
        } else {
            self.guard.graph.contains_triple(subject, predicate, object)
        }
    }

    fn triples<'b>(&'b self) -> Box<dyn Iterator<Item = (&'b Node, &'b Node, &'b Node)> + 'b> {
        Box::new(
            set::difference(&self.guard.graph, &self.removed_triples)
                .chain(self.added_triples.triples()),
        )
    }

    fn insert(&mut self, subject: Node, predicate: Node, object: Node) {
        if self
            .removed_triples
            .contains_triple(&subject, &predicate, &object)
        {
            self.removed_triples.remove(&subject, &predicate, &object)
        } else {
            self.added_triples.insert(subject, predicate, object);
        }

        if cfg!(test) {
            assert!(self.is_valid());
        }
    }

    fn remove(&mut self, subject: &Node, predicate: &Node, object: &Node) {
        if self
            .added_triples
            .contains_triple(&subject, &predicate, &object)
        {
            self.added_triples.remove(&subject, &predicate, &object)
        } else {
            self.removed_triples
                .clone_insert(subject, predicate, object);
        }

        if cfg!(test) {
            assert!(self.is_valid());
        }
    }

    fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&Node, &Node, &Node) -> bool,
    {
        let newly_removed_triples: HashGraph =
            self.triples().filter(|(s, p, o)| f(s, p, o)).collect();
        self.remove_all(newly_removed_triples.triples());

        if cfg!(test) {
            assert!(self.is_valid());
        }
    }
}
