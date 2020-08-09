use crate::*;

struct Testbed {
    predicate_a: Node,
    predicate_b: Node,
    predicate_c: Node,

    node_a: Node,
    node_b: Node,
    node_c: Node,

    graph: HashGraph,
}

impl Testbed {
    fn new() -> Self {
        let predicate_a = Node::from("urn:arrf:tests:predicate:a");
        let predicate_b = Node::from("urn:arrf:tests:predicate:b");
        let predicate_c = Node::from("urn:arrf:tests:predicate:c");

        let node_a = Node::from("urn:arrf:tests:node:a");
        let node_b = Node::from("urn:arrf:tests:node:b");
        let node_c = Node::blank();

        let mut graph = HashGraph::new();
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
fn contains_subject() {
    let testbed = Testbed::new();

    assert!(testbed.graph.contains_subject(&testbed.node_a));
    assert!(testbed.graph.contains_subject(&testbed.node_b));
    assert!(testbed.graph.contains_subject(&testbed.node_c));

    assert!(!testbed.graph.contains_subject(&testbed.predicate_a));
    assert!(!testbed.graph.contains_subject(&testbed.predicate_b));
    assert!(!testbed.graph.contains_subject(&testbed.predicate_c));
}

#[test]
fn subject() {
    let testbed = Testbed::new();

    let subjects: Vec<Node> = testbed.graph.subjects().cloned().collect();

    assert_eq!(3, subjects.len());

    assert!(subjects.contains(&testbed.node_a));
    assert!(subjects.contains(&testbed.node_b));
    assert!(subjects.contains(&testbed.node_c));

    assert!(!subjects.contains(&testbed.predicate_a));
    assert!(!subjects.contains(&testbed.predicate_b));
    assert!(!subjects.contains(&testbed.predicate_c));
}

#[test]
fn relationships() {
    let testbed = Testbed::new();

    let relationships: Vec<(&Node, &Node, &Node)>;
    relationships = testbed.graph.relationships(&testbed.node_a).collect();

    assert_eq!(1, relationships.len());
    assert!(relationships.contains(&(&testbed.node_a, &testbed.predicate_a, &testbed.node_b)));

    let relationships: Vec<(&Node, &Node, &Node)>;
    relationships = testbed.graph.relationships(&testbed.node_b).collect();

    assert_eq!(1, relationships.len());
    assert!(relationships.contains(&(&testbed.node_b, &testbed.predicate_b, &testbed.node_c)));

    let relationships: Vec<(&Node, &Node, &Node)>;
    relationships = testbed.graph.relationships(&testbed.node_c).collect();

    assert_eq!(1, relationships.len());
    assert!(relationships.contains(&(&testbed.node_c, &testbed.predicate_c, &testbed.node_a)));

    assert_eq!(0, testbed.graph.relationships(&testbed.predicate_a).count());
    assert_eq!(0, testbed.graph.relationships(&testbed.predicate_b).count());
    assert_eq!(0, testbed.graph.relationships(&testbed.predicate_c).count());
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

#[test]
fn difference() {
    let testbed = Testbed::new();

    let a = testbed.graph;
    let mut b = HashGraph::new();
    b.insert(
        testbed.node_a.clone(),
        testbed.predicate_a.clone(),
        testbed.node_b.clone(),
    );
    b.insert(
        testbed.node_b.clone(),
        testbed.predicate_a.clone(),
        testbed.node_a.clone(),
    );

    let difference: HashGraph = a.difference(&b).collect();

    assert_eq!(2, difference.len());
    assert!(difference.contains_triple(&testbed.node_b, &testbed.predicate_b, &testbed.node_c));
    assert!(difference.contains_triple(&testbed.node_c, &testbed.predicate_c, &testbed.node_a));
}

#[test]
fn symmetric_difference() {
    let testbed = Testbed::new();

    let a = testbed.graph;
    let mut b = HashGraph::new();
    b.insert(
        testbed.node_a.clone(),
        testbed.predicate_a.clone(),
        testbed.node_b.clone(),
    );
    b.insert(
        testbed.node_b.clone(),
        testbed.predicate_a.clone(),
        testbed.node_a.clone(),
    );

    let difference: HashGraph = a.symmetric_difference(&b).collect();

    assert_eq!(3, difference.len());
    assert!(difference.contains_triple(&testbed.node_b, &testbed.predicate_b, &testbed.node_c));
    assert!(difference.contains_triple(&testbed.node_c, &testbed.predicate_c, &testbed.node_a));
    assert!(difference.contains_triple(&testbed.node_b, &testbed.predicate_a, &testbed.node_a));
}

#[test]
fn union() {
    let testbed = Testbed::new();

    let a = testbed.graph;
    let mut b = HashGraph::new();
    b.insert(
        testbed.node_a.clone(),
        testbed.predicate_a.clone(),
        testbed.node_b.clone(),
    );
    b.insert(
        testbed.node_b.clone(),
        testbed.predicate_a.clone(),
        testbed.node_a.clone(),
    );

    let union: HashGraph = a.union(&b).collect();

    assert_eq!(4, union.len());
    assert!(union.contains_triple(&testbed.node_a, &testbed.predicate_a, &testbed.node_b));
    assert!(union.contains_triple(&testbed.node_b, &testbed.predicate_b, &testbed.node_c));
    assert!(union.contains_triple(&testbed.node_c, &testbed.predicate_c, &testbed.node_a));
    assert!(union.contains_triple(&testbed.node_b, &testbed.predicate_a, &testbed.node_a));
}

#[test]
fn is_subset_superset() {
    let testbed = Testbed::new();

    let a = testbed.graph;
    let mut b = HashGraph::new();
    b.insert(
        testbed.node_a.clone(),
        testbed.predicate_a.clone(),
        testbed.node_b.clone(),
    );
    b.insert(
        testbed.node_b.clone(),
        testbed.predicate_a.clone(),
        testbed.node_a.clone(),
    );

    assert!(a.is_subset(&a));
    assert!(a.is_superset(&a));

    assert!(!b.is_subset(&a));
    assert!(!a.is_subset(&b));

    b.remove(&testbed.node_b, &testbed.predicate_a, &testbed.node_a);

    assert!(b.is_subset(&a));
    assert!(a.is_superset(&b));
}

#[test]
fn is_disjoint() {
    let testbed = Testbed::new();

    let a = testbed.graph;
    let mut b = HashGraph::new();
    b.insert(
        testbed.node_a.clone(),
        testbed.predicate_a.clone(),
        testbed.node_b.clone(),
    );
    b.insert(
        testbed.node_b.clone(),
        testbed.predicate_a.clone(),
        testbed.node_a.clone(),
    );

    assert!(!a.is_disjoint(&a));
    assert!(!a.is_disjoint(&b));

    b.remove(&testbed.node_a, &testbed.predicate_a, &testbed.node_b);

    assert!(a.is_disjoint(&b));
}
