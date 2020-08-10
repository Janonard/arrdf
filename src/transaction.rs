use crate::{set, Graph, HashGraph, Node};
use std::ops::Deref;
use std::sync::{Arc, RwLock, RwLockWriteGuard};

pub struct TransactionGraph<G> {
    graph: Arc<RwLock<IntTransactionGraph<G>>>,
}

impl<G> Clone for TransactionGraph<G> {
    fn clone(&self) -> Self {
        Self {
            graph: self.graph.clone(),
        }
    }
}

impl<G: Graph> TransactionGraph<G> {
    pub fn new(graph: G) -> Self {
        Self {
            graph: Arc::new(RwLock::new(IntTransactionGraph::new(graph))),
        }
    }

    pub fn transaction(&mut self) -> Option<Transaction<G>> {
        self.graph.write().map(|guard| Transaction::new(guard)).ok()
    }

    pub fn try_transaction(&mut self) -> Option<Transaction<G>> {
        self.graph
            .try_write()
            .map(|guard| Transaction::new(guard))
            .ok()
    }

    pub fn query<T, Q: FnOnce(&G) -> T>(&self, query: Q) -> Option<T> {
        if let Ok(guard) = self.graph.read() {
            Some(query(&guard.graph))
        } else {
            None
        }
    }

    pub fn cached_query<T, Q: FnMut(&G) -> T>(&self, mut query: Q) -> Option<CachedQuery<T, G, Q>> {
        if let Ok(guard) = self.graph.read() {
            let result = query(&guard.graph);
            Some(CachedQuery {
                graph: self.clone(),
                query,
                result,
                current_revision: guard.revision,
            })
        } else {
            None
        }
    }
}

struct IntTransactionGraph<G> {
    graph: G,
    revision: usize,
}

impl<G> IntTransactionGraph<G> {
    pub fn new(graph: G) -> Self {
        Self { graph, revision: 0 }
    }
}

pub struct CachedQuery<T, G, Q> {
    graph: TransactionGraph<G>,
    query: Q,
    result: T,
    current_revision: usize,
}

impl<T, G: Graph, Q: FnMut(&G) -> T> CachedQuery<T, G, Q> {
    pub fn update(&mut self) {
        if let Ok(guard) = self.graph.graph.read() {
            if guard.revision > self.current_revision {
                self.result = (self.query)(&guard.graph);
                self.current_revision = guard.revision;
            }
        }
    }
}

impl<T, G, Q> Deref for CachedQuery<T, G, Q> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.result
    }
}

pub struct Transaction<'a, G> {
    guard: RwLockWriteGuard<'a, IntTransactionGraph<G>>,
    added_triples: HashGraph,
    removed_triples: HashGraph,
}

impl<'a, G: Graph> Transaction<'a, G> {
    fn new(guard: RwLockWriteGuard<'a, IntTransactionGraph<G>>) -> Self {
        Transaction {
            guard,
            added_triples: HashGraph::new(),
            removed_triples: HashGraph::new(),
        }
    }

    fn is_valid(&self) -> bool {
        set::is_subset(&self.removed_triples, &self.guard.graph)
            && set::is_disjoint(&self.guard.graph, &self.added_triples)
    }

    pub fn commit(mut self) {
        if cfg!(test) {
            assert!(self.is_valid());
        }

        self.guard.graph.remove_all(self.removed_triples.iter());
        self.guard.graph.extend(self.added_triples.into_iter());
        self.guard.revision += 1;
    }
}

impl<'a, G: Graph> Graph for Transaction<'a, G> {
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

    fn iter<'b>(&'b self) -> Box<dyn Iterator<Item = (&'b Node, &'b Node, &'b Node)> + 'b> {
        Box::new(
            set::difference(&self.guard.graph, &self.removed_triples)
                .chain(self.added_triples.iter()),
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
        let newly_removed_triples: HashGraph = self.iter().filter(|(s, p, o)| f(s, p, o)).collect();
        self.remove_all(newly_removed_triples.iter());

        if cfg!(test) {
            assert!(self.is_valid());
        }
    }
}
