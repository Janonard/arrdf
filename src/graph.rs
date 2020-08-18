use crate::Node;

/// A generalized RDF triple store.
///
/// If you just want to use a triple store, check out the [`HashGraph`](struct.HashGraph.html), which implements this trait.
///
/// An implementation of this trait only requires very few methods since many others have a default implementation based on them.
/// However, you may re-implement provided methods if your underlying data structure allows optimizations for them since
/// the provided implementations are rather inefficient.
///
/// Check out the [crate-level introduction](index.html) for some examples.
pub trait Graph {
    /// Return an iterator over all triples in the store.
    ///
    /// The actual type of the iterator is omited to allow compact implementations.
    fn iter<'a>(&'a self) -> Box<dyn 'a + Iterator<Item = (&'a Node, &'a Node, &'a Node)>>;

    /// Insert a triple into the store.
    ///
    /// Since a graph is a set of triples, adding a triple twice has no effect.
    ///
    /// This method takes ownership of the nodes. If you only have references to nodes, you can use [`clone_insert`](#method.clone_insert)
    /// to clone the nodes and insert them into the graph.
    ///
    /// ## Examples
    ///
    /// Basic usage:
    /// ```
    /// use arrdf::{Node, Graph, HashGraph};
    ///
    /// let mut graph = HashGraph::new();
    /// let node_a = Node::from("Node A");
    /// let node_b = Node::from("Node B");
    /// let node_c = Node::from("Node C");
    ///
    /// graph.insert(node_a, node_b, node_c);
    /// ```
    fn insert(&mut self, subject: Node, predicate: Node, object: Node);

    /// Remove a triple from the store.
    ///
    /// If the graph does not contain this triple, this method does nothing.
    ///
    /// ## Examples
    ///
    /// Basic usage:
    /// ```
    /// use arrdf::{Node, Graph, HashGraph};
    ///
    /// let node_a = Node::from("Node A");
    /// let node_b = Node::from("Node B");
    /// let node_c = Node::from("Node C");
    /// let mut graph: HashGraph = vec![(&node_a, &node_b, &node_c)].into_iter().collect();
    ///
    /// graph.remove(&node_a, &node_b, &node_c);
    ///
    /// assert!(!graph.contains(&node_a, &node_b, &node_c));
    /// ```
    fn remove(&mut self, subject: &Node, predicate: &Node, object: &Node);

    /// Return the number of triples in the store.
    ///
    /// ## Examples
    ///
    /// Basic usage:
    /// ```
    /// use arrdf::{Node, Graph, HashGraph};
    ///
    /// let node_a = Node::from("Node A");
    /// let node_b = Node::from("Node B");
    /// let node_c = Node::from("Node C");
    /// let mut graph: HashGraph = vec![(node_a, node_b, node_c)].into_iter().collect();
    ///
    /// assert_eq!(1, graph.len());
    /// ```
    fn len(&self) -> usize {
        self.iter().count()
    }

    /// Return `true` if the graph contains no triples.
    ///
    /// ## Examples
    ///
    /// Basic usage:
    /// ```
    /// use arrdf::{Node, Graph, HashGraph};
    ///
    /// let mut graph = HashGraph::new();
    /// let node_a = Node::from("Node A");
    /// let node_b = Node::from("Node B");
    /// let node_c = Node::from("Node C");
    ///
    /// assert!(graph.is_empty());
    /// graph.insert(node_a, node_b, node_c);
    /// assert!(!graph.is_empty());
    /// ```
    #[cfg(not(tarpaulin_include))]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Return `true` if the graph contains the given triples.
    ///
    /// The graph contains the given triple if and only if the iterator returned by [the `iter` method](#tymethod.iter)
    /// produces a triple where the subject, predicate and object are equal to the given subject, predicate and object nodes,
    /// as defined by the [`PartialEq`](https://doc.rust-lang.org/stable/std/cmp/trait.PartialEq.html) implementation of the [`Node`](struct.Node.html).
    /// An implementation may use an optimized query that doesn't use the triples iterator.
    ///
    /// ## Examples
    ///
    /// Basic usage:
    /// ```
    /// use arrdf::{Node, Graph, HashGraph};
    ///
    /// let node_a = Node::from("Node A");
    /// let node_b = Node::from("Node B");
    /// let node_c = Node::from("Node C");
    /// let mut graph: HashGraph = vec![(&node_a, &node_b, &node_c)].into_iter().collect();
    ///
    /// assert!(graph.contains(&node_a, &node_b, &node_c));
    /// ```
    fn contains(&self, subject: &Node, predicate: &Node, object: &Node) -> bool {
        self.iter()
            .any(|(s, p, o)| s == subject && p == predicate && o == object)
    }

    /// Return `true if the graph is valid, non-generalized RDF graph.
    ///
    /// In this crate, all graphs are "generalized graphs" per default. This means that both subject,
    /// predicate and object may be an IRI, a literal or a blank node. However, concrete implementations
    /// of RDF require that a subject must not be a literal and a predicate must be an IRI.
    ///
    /// You can use the [`sanitize`](#method.sanitize) method to remove triples that don't obey to this rule.
    fn is_valid_graph(&self) -> bool {
        self.iter().all(|(s, p, _)| !s.is_literal() && p.is_iri())
    }

    /// Clone the referenced nodes and insert them as a triple.
    ///
    /// ## Examples
    ///
    /// ```
    /// use arrdf::{Node, Graph, HashGraph};
    ///
    /// let node_a = Node::from("Node A");
    /// let node_b = Node::from("Node B");
    /// let node_c = Node::from("Node C");
    /// let mut graph = HashGraph::new();
    ///
    /// graph.clone_insert(&node_a, &node_b, &node_c);
    ///
    /// // With the default implementation, this is equivalent to:
    /// graph.insert(node_a.clone(), node_b.clone(), node_c.clone());
    /// ```
    #[cfg(not(tarpaulin_include))]
    fn clone_insert(&mut self, subject: &Node, predicate: &Node, object: &Node) {
        self.insert(subject.clone(), predicate.clone(), object.clone());
    }

    /// Extend the graph with the contents of the iterator.
    ///
    /// This method provides the same functionality as the [`Extend`](https://doc.rust-lang.org/stable/std/iter/trait.Extend.html) trait,
    /// but since external traits like this can not be implemented automatically for every implementation of `Graph`, you have to use
    /// this method instead.
    ///
    /// ## Examples
    ///
    /// ```
    /// use arrdf::{Node, Graph, HashGraph};
    ///
    /// let node_a = Node::from("Node A");
    /// let node_b = Node::from("Node B");
    /// let node_c = Node::from("Node C");
    /// let mut graph = HashGraph::new();
    ///
    /// let collection = vec![
    ///     (node_a.clone(), node_b.clone(), node_c.clone()),
    ///     (node_c, node_b, node_a)
    /// ];
    ///
    /// graph.extend(collection);
    /// ```
    ///
    /// With the default implementation, this is equivalent to:
    /// ```
    /// # use arrdf::{Node, Graph, HashGraph};
    /// #
    /// # let node_a = Node::from("Node A");
    /// # let node_b = Node::from("Node B");
    /// # let node_c = Node::from("Node C");
    /// # let mut graph = HashGraph::new();
    /// #
    /// # let collection = vec![
    /// #     (node_a.clone(), node_b.clone(), node_c.clone()),
    /// #     (node_c, node_b, node_a)
    /// # ];
    /// #
    /// for (subject, predicate, object) in collection {
    ///     graph.insert(subject, predicate, object);
    /// }
    /// ```
    fn extend<G>(&mut self, iter: G)
    where
        G: IntoIterator<Item = (Node, Node, Node)>,
    {
        for (s, p, o) in iter {
            self.insert(s, p, o);
        }
    }

    /// Extend the graph with the cloned contents of the iterator.
    ///
    /// This version of the [`extend`](#method.extend) method is useful to extend a graph with an
    /// iterator over another graph.
    ///
    /// ## Examples
    /// ```
    /// use arrdf::{Node, Graph, HashGraph};
    ///
    /// let node_a = Node::from("Node A");
    /// let node_b = Node::from("Node B");
    /// let node_c = Node::from("Node C");
    ///
    /// let mut graph_a: HashGraph = vec![(&node_a, &node_b, &node_c)].into_iter().collect();
    /// let graph_b: HashGraph = vec![(node_c, node_b, node_a)].into_iter().collect();
    ///
    /// graph_a.clone_extend(graph_b.iter());
    /// ```
    fn clone_extend<'a, G>(&mut self, iter: G)
    where
        G: 'a + IntoIterator<Item = (&'a Node, &'a Node, &'a Node)>,
    {
        self.extend(
            iter.into_iter()
                .map(|(s, p, o)| (s.clone(), p.clone(), o.clone())),
        )
    }

    /// Remove all triples produced by the iterator.
    ///
    /// ## Examples
    /// ```
    /// use arrdf::{Node, Graph, HashGraph};
    ///
    /// let node_a = Node::from("Node A");
    /// let node_b = Node::from("Node B");
    /// let node_c = Node::from("Node C");
    ///
    /// let mut graph_a: HashGraph = vec![(&node_a, &node_b, &node_c)].into_iter().collect();
    /// let graph_b: HashGraph = vec![(node_c, node_b, node_a)].into_iter().collect();
    ///
    /// graph_a.remove_all(graph_b.iter());
    /// ```
    #[cfg(not(tarpaulin_include))]
    fn remove_all<'a, G>(&mut self, iter: G)
    where
        G: IntoIterator<Item = (&'a Node, &'a Node, &'a Node)>,
    {
        for (s, p, o) in iter {
            self.remove(s, p, o);
        }
    }

    /// Retain only triples where the predicate `f` returns `true`.
    ///
    /// In other words, remove all triples such that the predicate `f` returns `false`.
    fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&Node, &Node, &Node) -> bool,
    {
        let mut removed_nodes = std::collections::HashSet::<(Node, Node, Node)>::new();
        for (s, p, o) in self.iter() {
            if !f(s, p, o) {
                removed_nodes.insert((s.clone(), p.clone(), o.clone()));
            }
        }
        self.remove_all(removed_nodes.iter().map(|(s, p, o)| (s, p, o)))
    }

    /// Remove all triples that don't comply with the W3C definition of a well-formed RDF triple.
    ///
    /// In this crate, all graphs are "generalized graphs" per default. This means that both subject,
    /// predicate and object may be an IRI, a literal or a blank node. However, concrete implementations
    /// of RDF require that a subject must not be a literal and a predicate must be an IRI.
    ///
    /// The [`is_valid_graph`](#method.is_valid_graph) method returns `true` if all triples are well-formed and
    /// therefore checks if `sanitize` would remove any triples.
    fn sanitize(&mut self) {
        self.retain(|s, p, _| !s.is_literal() && p.is_iri());
    }
}
