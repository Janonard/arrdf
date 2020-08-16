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

    assert_eq!(3, graph.transaction().len());
    assert!(graph.transaction().contains(node_a, predicate_a, node_b));
    assert!(graph.transaction().contains(node_b, predicate_b, node_c));
    assert!(graph.transaction().contains(node_c, predicate_c, node_a));
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
    let testbed = Testbed::new();
    let graph = TransactionGraph::new(testbed.graph);
    let node_a = &testbed.node_a;
    let predicate_a = &testbed.predicate_a;

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
    let testbed = Testbed::new();
    let graph = TransactionGraph::new(testbed.graph.clone());
    let node_a = &testbed.node_a;
    let node_b = &testbed.node_b;
    let node_c = &testbed.node_c;
    let predicate_a = &testbed.predicate_a;
    let predicate_b = &testbed.predicate_b;
    let predicate_c = &testbed.predicate_c;

    /// Actually test the mutation graph.
    ///
    /// All modifications are done twice to also test no-op operations (e.g. removing a triple
    /// that isn't in the store).
    fn run_transaction(t: &mut MutTransaction<HashGraph>, testbed: &Testbed) {
        let node_a = &testbed.node_a;
        let node_b = &testbed.node_b;
        let node_c = &testbed.node_c;
        let predicate_a = &testbed.predicate_a;
        let predicate_b = &testbed.predicate_b;
        let predicate_c = &testbed.predicate_c;

        assert_eq!(3, t.len());
        assert!(t.contains(node_a, predicate_a, node_b));
        assert!(t.contains(node_b, predicate_b, node_c));
        assert!(t.contains(node_c, predicate_c, node_a));

        t.clone_insert(node_a, predicate_a, node_a);
        t.clone_insert(node_a, predicate_a, node_a);

        assert_eq!(4, t.len());
        assert!(t.contains(node_a, predicate_a, node_b));
        assert!(t.contains(node_b, predicate_b, node_c));
        assert!(t.contains(node_c, predicate_c, node_a));
        assert!(t.contains(node_a, predicate_a, node_a));

        t.remove(node_a, predicate_a, node_a);
        t.remove(node_a, predicate_a, node_a);

        assert_eq!(3, t.len());
        assert!(t.contains(node_a, predicate_a, node_b));
        assert!(t.contains(node_b, predicate_b, node_c));
        assert!(t.contains(node_c, predicate_c, node_a));

        t.remove(node_a, predicate_a, node_b);
        t.remove(node_a, predicate_a, node_b);

        assert_eq!(2, t.len());
        assert!(!t.contains(node_a, predicate_a, node_b));
        assert!(t.contains(node_b, predicate_b, node_c));
        assert!(t.contains(node_c, predicate_c, node_a));

        t.clone_insert(node_a, predicate_a, node_b);
        t.clone_insert(node_a, predicate_a, node_b);

        assert_eq!(3, t.len());
        assert!(t.contains(node_a, predicate_a, node_b));
        assert!(t.contains(node_b, predicate_b, node_c));
        assert!(t.contains(node_c, predicate_c, node_a));

        t.clone_insert(node_b, predicate_b, node_b);
        t.clone_insert(node_b, predicate_b, node_b);
        t.retain(|s, _, _| s == node_a);
        t.retain(|s, _, _| s == node_a);

        let result: HashGraph = t.iter().collect();

        assert_eq!(3, result.len());
        assert!(!result.contains(node_a, predicate_a, node_b));
        assert!(result.contains(node_b, predicate_b, node_c));
        assert!(result.contains(node_c, predicate_c, node_a));
        assert!(result.contains(node_b, predicate_b, node_b));
    }

    // Execute, but don't commit.
    run_transaction(&mut graph.mut_transaction(), &testbed);

    // No alterations should be present.
    assert_eq!(3, graph.transaction().len());
    assert!(graph.transaction().contains(node_a, predicate_a, node_b));
    assert!(graph.transaction().contains(node_b, predicate_b, node_c));
    assert!(graph.transaction().contains(node_c, predicate_c, node_a));

    // Execute and commit.
    let mut transaction = graph.try_mut_transaction().unwrap();
    run_transaction(&mut transaction, &testbed);

    // Assert that we can't start another transaction.
    assert!(graph.try_mut_transaction().is_none());

    transaction.commit();

    // The alterations should be present now.
    assert_eq!(3, graph.transaction().len());
    assert!(!graph.transaction().contains(node_a, predicate_a, node_b));
    assert!(graph.transaction().contains(node_b, predicate_b, node_c));
    assert!(graph.transaction().contains(node_c, predicate_c, node_a));
    assert!(graph.transaction().contains(node_b, predicate_b, node_b));
}
