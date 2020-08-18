//! Arr! A Rusty RDF triplestore!
//!
//! This crate contains convenient and ergonomic ways to handle and store RDF graphs. If RDF is new to you, you should definitely check out the [RDF 1.1 Primer](https://www.w3.org/TR/rdf11-primer/) and also the [RDF 1.1 Concepts and Abstract Syntax](https://www.w3.org/TR/rdf11-concepts/) if you want to know the details.
//!
//!
//!
//! # Nodes
//!
//! In general, this crate handles generalized RDF triples and graphs. This means that the subject, the predicate and the object of a triple may always be an IRI, a Literal or a Blank. Therefore, a [`Node`](struct.Node.html) is implemented as an [`Arc`](https://doc.rust-lang.org/stable/std/sync/struct.Arc.html) pointing to a shared, immutable string that can either contain an IRI, a literal, or nothing if it's a blank.
//!
//! This means that blank nodes can not be distinguished by a string ID. Instead, they are distinguished by the address of the shared string: Two blank nodes are equal if and only if they point to the same, empty string, and two non-blank nodes are equal if and only if the pointed strings are equal. Obviously, a blank node and a non-blank node are never equal.
//!
//! This might look a bit counter-intuitive at first, but it leads to a rather intuitive way to use nodes:
//!
//! ```
//! use arrdf::Node;
//!
//! // Obviously, Google and DuckDuckGo are not the same!
//! let google = Node::from("https://google.com");
//! let duck_duck_go = Node::from("https://duckduckgo.com");
//! assert_ne!(&google, &duck_duck_go);
//!
//! // However, if we create a new node with an equal IRI, the nodes are equal too!
//! let also_google = Node::from("https://google.com");
//! assert_eq!(&google, &also_google);
//!
//! // Two independent blank nodes aren't equal,
//! // because a new, empty string is allocated for both of them.
//! let blank_a = Node::blank();
//! let blank_b = Node::blank();
//! assert_ne!(&blank_a, &blank_b);
//!
//! // Cloning, however, produces another, equal node:
//! let cloned_blank_a = blank_a.clone();
//! assert_eq!(&blank_a, &cloned_blank_a);
//! ```
//!
//! # Graphs
//!
//! You can think of a graph as a set of `Node` triples, e.g. something like `std::collections::HashSet<(Node, Node, Node)>`. However, this crate mostly uses an optimized collection for triples, the [`HashGraph`](struct.HashGraph.html). The interface is heavily inspired by the standard `HashSet` and is abstracted by the [`Graph` trait](trait.Graph.html).
//!
//! Some examples on how to use a graph:
//!
//! ```
//! use arrdf::{Node, Graph, HashGraph};
//! use std::collections::HashSet;
//!
//! // First, we need to crate some meaningful nodes:
//! let janonard = Node::from("https://github.com/Janonard");
//! let torvalds = Node::from("https://github.com/torvalds");
//!
//! let maintainer = Node::from("http://schema.org/maintainer");
//!
//! let rust_lv2 = Node::from("https://github.com/RustAudio/rust-lv2");
//! let linux = Node::from("https://github.com/torvalds/linux");
//!
//! let programming_language = Node::from("https://schema.org/programmingLanguage");
//!
//! let rust = Node::from("https://www.rust-lang.org/");
//! let c = Node::from("http://www.open-std.org/jtc1/sc22/wg14/");
//!
//! // Now, we can create a graph and insert the triples into it.
//! let mut graph: HashGraph = vec![
//!     (&rust_lv2, &maintainer, &janonard),
//!     (&linux, &maintainer, &torvalds),
//!     (&rust_lv2, &programming_language, &rust),
//!     (&linux, &programming_language, &c),
//! ].into_iter().collect();
//!
//! // Let's query the maintainers of projects that are written in Rust:
//! let projects: HashSet<Node> = graph
//!     .iter()
//!     .filter_map(|(subject, predicate, object)| {
//!         if predicate == &programming_language && object == &rust {
//!             Some(subject.clone())
//!         } else {
//!             None
//!         }
//!     })
//!     .collect();
//!
//! let maintainers: HashSet<Node> = graph
//!     .iter()
//!     .filter_map(|(subject, predicate, object)| {
//!         if projects.contains(subject) && predicate == &maintainer {
//!             Some(object.clone())
//!         } else {
//!             None
//!         }
//!     })
//!     .collect();
//!
//! assert_eq!(1, maintainers.len());
//! assert!(maintainers.contains(&janonard));
//!
//! // We can also remove triples with `retain`.
//! // This will remove all triples with Rust-LV2 as a subject.
//! graph.retain(|s, p, o| s == &rust_lv2);
//! ```
//!
//! # Set operations
//!
//! As graphs are basically sets, you might also want to use some set operations, like the difference
//! or the union. The [`set`](set/index.html) contains functions with all set operations which can also be applied with differing kinds of graphs. Some examples:
//!
//! ```
//! use arrdf::{Node, Graph, HashGraph};
//! use std::collections::HashSet;
//!
//! // Node definitions same as above...
//! # let janonard = Node::from("https://github.com/Janonard");
//! # let torvalds = Node::from("https://github.com/torvalds");
//! #
//! # let maintainer = Node::from("http://schema.org/maintainer");
//! #
//! # let rust_lv2 = Node::from("https://github.com/RustAudio/rust-lv2");
//! # let linux = Node::from("https://github.com/torvalds/linux");
//! #
//! # let programming_language = Node::from("https://schema.org/programmingLanguage");
//! #
//! # let rust = Node::from("https://www.rust-lang.org/");
//! # let c = Node::from("http://www.open-std.org/jtc1/sc22/wg14/");
//!
//! // Instead of one big graph, we now have two distinct graphs:
//! let rust_projects: HashGraph = vec![
//!     (&rust_lv2, &maintainer, &janonard),
//!     (&rust_lv2, &programming_language, &rust),
//! ].into_iter().collect();
//!
//! let c_projects: HashGraph = vec![
//!     (&linux, &maintainer, &torvalds),
//!     (&linux, &programming_language, &c),
//! ].into_iter().collect();
//!
//! // Let's put them together:
//! let all_projects: HashGraph = arrdf::set::union(&rust_projects, &c_projects).collect();
//!
//! assert_eq!(4, all_projects.len());
//! assert!(all_projects.contains(&rust_lv2, &maintainer, &janonard));
//! assert!(all_projects.contains(&rust_lv2, &programming_language, &rust));
//! assert!(all_projects.contains(&linux, &maintainer, &torvalds));
//! assert!(all_projects.contains(&linux, &programming_language, &c));
//! ```
//!
//! # Transactions
//!
//! The `HashGraph` is great for single-threaded applications. However, if you'd like to query and modify the graph with multiple threads, you should use the [`TransactionGraph`](transaction/struct.TransactionGraph.html). It wraps another graph and provides atomic, revertable transactions as well as cachable queries. It internally uses an atomically reference-counted
//! [`RwLock`](https://doc.rust-lang.org/stable/std/sync/struct.RwLock.html), which means that many reading transactions
//! may take place at once, but only one writing transaction.
//!
//! ```
//! use arrdf::{Node, Graph, HashGraph, transaction::TransactionGraph};
//! use std::collections::HashSet;
//!
//! // The nodes are the same again...
//! # let janonard = Node::from("https://github.com/Janonard");
//! # let torvalds = Node::from("https://github.com/torvalds");
//! #
//! # let maintainer = Node::from("http://schema.org/maintainer");
//! #
//! # let rust_lv2 = Node::from("https://github.com/RustAudio/rust-lv2");
//! # let linux = Node::from("https://github.com/torvalds/linux");
//! #
//! # let programming_language = Node::from("https://schema.org/programmingLanguage");
//! #
//! # let rust = Node::from("https://www.rust-lang.org/");
//! # let c = Node::from("http://www.open-std.org/jtc1/sc22/wg14/");
//!
//! // First, we create a new graph with some triples.
//! let rust_projects: HashGraph = vec![
//!     (&rust_lv2, &maintainer, &janonard),
//!     (&rust_lv2, &programming_language, &rust),
//! ].into_iter().collect();
//!
//! // Then, we wrap it in a `TransactionGraph`.
//! let rust_projects = TransactionGraph::new(rust_projects);
//!
//! // Here we create a cached query. If the the underlying graph changes, the closure is executed
//! // again to produce the new query result.
//! let mut rust_lv2_maintainers = rust_projects.cached_query(|g: &HashGraph| {
//!     let maintainers: HashSet<Node> = g.iter().filter_map(|(s, p, o)| {
//!         if s == &rust_lv2 && p == &maintainer {
//!             Some(o.clone())
//!         } else {
//!             None
//!         }
//!     }).collect();
//!     maintainers
//! });
//!
//! // The returned object dereferences to the created `HashSet`:
//! assert_eq!(1, rust_lv2_maintainers.len());
//! assert!(rust_lv2_maintainers.contains(&janonard));
//!
//! // If you want to alter the graph, you have to start a mutable transaction.
//! // Immutable transactions that only provide immutable access are available too!
//! let mut transaction = rust_projects.mut_transaction();
//!
//! // A mutable transaction implements `Graph`!
//! transaction.clone_insert(&rust_lv2, &maintainer, &torvalds);
//!
//! // You have to commit your change once your done. If the transaction is dropped,
//! // all changes are discarded.
//! transaction.commit();
//!
//! // Now, we update our query and assert the result:
//! rust_lv2_maintainers.update();
//! assert_eq!(2, rust_lv2_maintainers.len());
//! assert!(rust_lv2_maintainers.contains(&janonard));
//! assert!(rust_lv2_maintainers.contains(&torvalds));
//! ```
mod graph;
mod hash_graph;
mod node;
pub mod set;
pub mod transaction;

pub use graph::Graph;
pub use hash_graph::HashGraph;
pub use node::Node;

#[cfg(test)]
struct Testbed {
    predicate_a: Node,
    predicate_b: Node,
    predicate_c: Node,

    node_a: Node,
    node_b: Node,
    node_c: Node,

    graph: hash_graph::HashGraph,
}

#[cfg(test)]
impl Testbed {
    fn new() -> Self {
        let predicate_a = Node::from("urn:arrf:tests:predicate:a");
        let predicate_b = Node::from("urn:arrf:tests:predicate:b");
        let predicate_c = Node::from("urn:arrf:tests:predicate:c");

        let node_a = Node::from("urn:arrf:tests:node:a");
        let node_b = Node::from("urn:arrf:tests:node:b");
        let node_c = Node::blank();

        let mut graph = hash_graph::HashGraph::new();
        graph.insert(node_a.clone(), predicate_a.clone(), node_b.clone());
        graph.insert(node_b.clone(), predicate_b.clone(), node_c.clone());
        graph.insert(node_c.clone(), predicate_c.clone(), node_a.clone());

        Self {
            predicate_a,
            predicate_b,
            predicate_c,

            node_a,
            node_b,
            node_c,

            graph,
        }
    }
}
