use std::collections::{HashMap, HashSet};

mod node;
pub use node::Node;
mod parsing;
pub use parsing::*;

mod construction;

mod queries;

mod modifications;

#[cfg(test)]
mod tests;

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct HashGraph {
    nodes: HashMap<Node, HashSet<(Node, Node)>>,
}
