use crate::transaction::*;
use crate::*;

#[test]
fn query() {
    let validator = Validator::new(HashGraph::new());
    let graph = TransactionGraph::new(validator.graph);
    let node_a = &validator.node_a;
    let node_b = &validator.node_b;
    let node_c = &validator.node_c;
    let predicate_a = &validator.predicate_a;
    let predicate_b = &validator.predicate_b;
    let predicate_c = &validator.predicate_c;

    assert_eq!(3, graph.transaction().len());
    assert!(graph.transaction().contains(node_a, predicate_a, node_b));
    assert!(graph.transaction().contains(node_b, predicate_b, node_c));
    assert!(graph.transaction().contains(node_c, predicate_c, node_a));
}

#[test]
fn try_query() {
    let validator = Validator::new(HashGraph::new());
    let graph = TransactionGraph::new(validator.graph);
    let node_a = &validator.node_a;
    let node_b = &validator.node_b;
    let node_c = &validator.node_c;
    let predicate_a = &validator.predicate_a;
    let predicate_b = &validator.predicate_b;
    let predicate_c = &validator.predicate_c;

    assert_eq!(3, graph.transaction().len());
    assert!(graph
        .try_transaction()
        .unwrap()
        .contains(node_a, predicate_a, node_b));
    assert!(graph
        .try_transaction()
        .unwrap()
        .contains(node_b, predicate_b, node_c));
    assert!(graph
        .try_transaction()
        .unwrap()
        .contains(node_c, predicate_c, node_a));

    let _transaction = graph.mut_transaction();

    assert!(graph.try_transaction().is_none());
}

#[test]
fn cached_query() {
    let validator = Validator::new(HashGraph::new());
    let graph = TransactionGraph::new(validator.graph);
    let node_a = &validator.node_a;
    let predicate_a = &validator.predicate_a;

    let mut query = graph.cached_query(|g| g.len());
    assert_eq!(3, *query);

    let mut transaction = graph.mut_transaction();
    transaction.clone_insert(node_a, predicate_a, node_a);

    // Assert that we can't run a query since a mutable transaction is in progress.
    assert!(graph.try_cached_query(|g| g.len()).is_none());

    transaction.commit();

    query.update();
    assert_eq!(4, *query);
}

#[test]
fn mut_transaction() {
    let graph = TransactionGraph::new(HashGraph::new());

    // Let the validator set up the graph, commit the setup and start a new transaction.
    let mut validator = Validator::new(graph.mut_transaction());
    validator.graph.commit();

    validator.graph = graph.mut_transaction();
    validator.validate();
}

#[test]
fn try_mut_transaction() {
    let graph = TransactionGraph::new(HashGraph::new());

    // Assert that we can start a mutable transaction now.
    assert!(graph.try_mut_transaction().is_some());

    // Start an immutable transaction and assert that no mutable transaction can be started.
    let _transaction = graph.transaction();
    assert!(graph.try_mut_transaction().is_none());
}
