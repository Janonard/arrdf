pub mod node;
pub use node::Node;

pub mod hash;

pub trait Graph<'a> {
    type TripleIter: 'a + Iterator;
    fn triples(&'a self) -> Self::TripleIter;

    fn contains_triple(&self, subject: &Node, predicate: &Node, object: &Node) -> bool;

    fn add_triple(&mut self, subject: Node, predicate: Node, object: Node);

    fn remove_triple(&mut self, subject: &Node, predicate: &Node, object: &Node);
}
