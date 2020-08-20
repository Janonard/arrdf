use crate::{set, Graph, HashGraph, Node};
use std::ops::Deref;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard, TryLockError};

#[cfg(test)]
mod tests;

pub struct TransactionGraph<G> {
    graph: Arc<RwLock<IntTransactionGraph<G>>>,
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

impl<G> Clone for TransactionGraph<G> {
    fn clone(&self) -> Self {
        Self {
            graph: self.graph.clone(),
        }
    }
}

impl<G: Graph + Default> Default for TransactionGraph<G> {
    #[cfg(not(tarpaulin_include))]
    fn default() -> Self {
        Self::new(G::default())
    }
}

impl<G: Graph> TransactionGraph<G> {
    pub fn new(graph: G) -> Self {
        Self {
            graph: Arc::new(RwLock::new(IntTransactionGraph::new(graph))),
        }
    }

    pub fn transaction(&self) -> Transaction<G> {
        Transaction::new(self.graph.read().unwrap())
    }

    pub fn try_transaction(&self) -> Option<Transaction<G>> {
        match self.graph.try_read() {
            Ok(guard) => Some(Transaction::new(guard)),
            Err(TryLockError::WouldBlock) => None,
            #[cfg(not(tarpaulin_include))]
            _ => panic!("An active transaction panicked (Graph is poisoned)"),
        }
    }

    pub fn mut_transaction(&self) -> MutTransaction<G> {
        self.graph
            .write()
            .map(|guard| MutTransaction::new(guard))
            .unwrap()
    }

    pub fn try_mut_transaction(&self) -> Option<MutTransaction<G>> {
        match self.graph.try_write() {
            Ok(guard) => Some(MutTransaction::new(guard)),
            Err(TryLockError::WouldBlock) => None,
            #[cfg(not(tarpaulin_include))]
            _ => panic!("An active transaction panicked (Graph is poisoned)"),
        }
    }

    pub fn cached_query<T, Q: FnMut(&G) -> T>(&self, query: Q) -> CachedQuery<T, G, Q> {
        let guard = self.graph.read().unwrap();
        CachedQuery::new(self.clone(), guard, query)
    }

    pub fn try_cached_query<T, Q: FnMut(&G) -> T>(&self, query: Q) -> Option<CachedQuery<T, G, Q>> {
        match self.graph.try_read() {
            Ok(guard) => Some(CachedQuery::new(self.clone(), guard, query)),
            Err(TryLockError::WouldBlock) => None,
            #[cfg(not(tarpaulin_include))]
            _ => panic!("An active transaction panicked (Graph is poisoned)"),
        }
    }
}

pub struct Transaction<'a, G> {
    guard: RwLockReadGuard<'a, IntTransactionGraph<G>>,
}

impl<'a, G> Transaction<'a, G> {
    fn new(guard: RwLockReadGuard<'a, IntTransactionGraph<G>>) -> Self {
        Self { guard }
    }
}

impl<'a, G> Deref for Transaction<'a, G> {
    type Target = G;

    fn deref(&self) -> &G {
        &self.guard.graph
    }
}

pub struct MutTransaction<'a, G> {
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

impl<'a, G: Graph> Graph for MutTransaction<'a, G> {
    fn len(&self) -> usize {
        self.guard.graph.len() + self.added_triples.len() - self.removed_triples.len()
    }

    fn contains(&self, subject: &Node, predicate: &Node, object: &Node) -> bool {
        if self.added_triples.contains(subject, predicate, object) {
            true
        } else if self.removed_triples.contains(subject, predicate, object) {
            false
        } else {
            self.guard.graph.contains(subject, predicate, object)
        }
    }

    fn iter<'b>(&'b self) -> Box<dyn Iterator<Item = (&'b Node, &'b Node, &'b Node)> + 'b> {
        Box::new(
            set::difference(&self.guard.graph, &self.removed_triples)
                .chain(self.added_triples.iter()),
        )
    }

    fn insert(&mut self, subject: Node, predicate: Node, object: Node) {
        if self.removed_triples.contains(&subject, &predicate, &object) {
            self.removed_triples.remove(&subject, &predicate, &object)
        } else if !self.guard.graph.contains(&subject, &predicate, &object) {
            self.added_triples.insert(subject, predicate, object);
        }

        if cfg!(test) {
            assert!(self.is_valid());
        }
    }

    fn remove(&mut self, subject: &Node, predicate: &Node, object: &Node) {
        if self.added_triples.contains(&subject, &predicate, &object) {
            self.added_triples.remove(&subject, &predicate, &object)
        } else if self.guard.graph.contains(&subject, &predicate, &object) {
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
            self.iter().filter(|(s, p, o)| !f(s, p, o)).collect();
        self.remove_all(newly_removed_triples.iter());

        if cfg!(test) {
            assert!(self.is_valid());
        }
    }

    fn clear(&mut self) {
        self.added_triples.clear();
        self.removed_triples.clone_extend(self.guard.graph.iter());

        if cfg!(test) {
            assert!(self.is_valid());
        }
    }
}

pub struct CachedQuery<T, G, Q> {
    graph: TransactionGraph<G>,
    query: Q,
    result: T,
    current_revision: usize,
}

impl<T, G, Q: FnMut(&G) -> T> CachedQuery<T, G, Q> {
    fn new(
        graph: TransactionGraph<G>,
        guard: RwLockReadGuard<IntTransactionGraph<G>>,
        mut query: Q,
    ) -> Self {
        Self {
            graph,
            result: query(&guard.graph),
            query,
            current_revision: guard.revision,
        }
    }
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
