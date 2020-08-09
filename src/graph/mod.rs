use crate::Node;
use std::collections::{HashMap, HashSet};

mod construction;

mod queries;

mod modifications;

#[cfg(test)]
mod tests;

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct HashGraph {
    nodes: HashMap<Node, HashSet<(Node, Node)>>,
}
