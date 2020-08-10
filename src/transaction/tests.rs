use crate::transaction::*;
use crate::*;

#[test]
fn query() {
    let testbed = Testbed::new();
    let graph = TransactionGraph::new(testbed.graph);
    let node_a = &testbed.node_a;
    let node_b = &testbed.node_b;
    let node_c = &testbed.node_c;
    let predicate_a = &testbed.predicate_a;
    let predicate_b = &testbed.predicate_b;
    let predicate_c = &testbed.predicate_c;

    assert_eq!(3, graph.query(|g| g.len()));
    assert!(graph.query(|g| g.contains(node_a, predicate_a, node_b)));
    assert!(graph.query(|g| g.contains(node_b, predicate_b, node_c)));
    assert!(graph.query(|g| g.contains(node_c, predicate_c, node_a)));
}

#[test]
fn try_query() {
    let testbed = Testbed::new();
    let graph = TransactionGraph::new(testbed.graph);
    let node_a = &testbed.node_a;
    let node_b = &testbed.node_b;
    let node_c = &testbed.node_c;
    let predicate_a = &testbed.predicate_a;
    let predicate_b = &testbed.predicate_b;
    let predicate_c = &testbed.predicate_c;

    assert_eq!(3, graph.query(|g| g.len()));
    assert!(graph
        .try_query(|g| g.contains(node_a, predicate_a, node_b))
        .unwrap());
    assert!(graph
        .try_query(|g| g.contains(node_b, predicate_b, node_c))
        .unwrap());
    assert!(graph
        .try_query(|g| g.contains(node_c, predicate_c, node_a))
        .unwrap());

    let _transaction = graph.transaction();

    assert!(graph.try_query(|g| g.len()).is_none());
}

#[test]
fn cached_query() {
    let testbed = Testbed::new();
    let graph = TransactionGraph::new(testbed.graph);
    let node_a = &testbed.node_a;
    let predicate_a = &testbed.predicate_a;

    let mut query = graph.cached_query(|g| g.len());
    assert_eq!(3, *query);

    let mut transaction = graph.transaction();
    transaction.clone_insert(node_a, predicate_a, node_a);
    transaction.commit();

    query.update();
    assert_eq!(4, *query);
}
