use crate::*;

#[test]
fn len() {
    let mut testbed = Testbed::new();
    assert_eq!(3, testbed.graph.len());

    testbed
        .graph
        .insert(testbed.node_a, testbed.predicate_a, testbed.node_c);
    assert_eq!(4, testbed.graph.len());
}

#[test]
fn triples() {
    let testbed = Testbed::new();

    let triples: Vec<(&Node, &Node, &Node)> = testbed.graph.triples().collect();
    assert_eq!(3, triples.len());
    assert!(triples.contains(&(&testbed.node_a, &testbed.predicate_a, &testbed.node_b)));
    assert!(triples.contains(&(&testbed.node_b, &testbed.predicate_b, &testbed.node_c)));
    assert!(triples.contains(&(&testbed.node_c, &testbed.predicate_c, &testbed.node_a)));
}

#[test]
fn is_valid_rdf() {
    let mut testbed = Testbed::new();

    assert!(!testbed.graph.is_valid_graph());
    testbed.graph.sanitize();
    assert!(testbed.graph.is_valid_graph());
}

#[test]
fn contains_triple() {
    let testbed = Testbed::new();

    assert!(testbed
        .graph
        .contains_triple(&testbed.node_a, &testbed.predicate_a, &testbed.node_b));
    assert!(testbed
        .graph
        .contains_triple(&testbed.node_b, &testbed.predicate_b, &testbed.node_c));
    assert!(testbed
        .graph
        .contains_triple(&testbed.node_c, &testbed.predicate_c, &testbed.node_a));

    assert!(!testbed
        .graph
        .contains_triple(&testbed.node_a, &testbed.predicate_b, &testbed.node_b));
}

#[test]
fn remove() {
    let mut testbed = Testbed::new();

    testbed
        .graph
        .remove(&testbed.node_c, &testbed.predicate_c, &testbed.node_a);

    let triples: Vec<(&Node, &Node, &Node)> = testbed.graph.triples().collect();
    assert_eq!(2, triples.len());
    assert!(triples.contains(&(&testbed.node_a, &testbed.predicate_a, &testbed.node_b)));
    assert!(triples.contains(&(&testbed.node_b, &testbed.predicate_b, &testbed.node_c)));
}

#[test]
fn retain() {
    let mut testbed = Testbed::new();

    testbed.graph.retain(|_, _, object| object.is_blank());

    let triples: Vec<(&Node, &Node, &Node)> = testbed.graph.triples().collect();
    assert_eq!(1, triples.len());
    assert!(triples.contains(&(&testbed.node_b, &testbed.predicate_b, &testbed.node_c)));
}

#[test]
fn from_iter() {
    let testbed = Testbed::new();
    let other_graph: HashGraph = testbed.graph.triples().collect();
    assert_eq!(testbed.graph, other_graph);

    let other_graph: HashGraph = testbed
        .graph
        .triples()
        .map(|(s, p, o)| (s.clone(), p.clone(), o.clone()))
        .collect();
    assert_eq!(testbed.graph, other_graph);
}
