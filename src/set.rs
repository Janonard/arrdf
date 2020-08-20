use crate::{Graph, Node};

pub fn difference<'a, G, H>(
    lhs: &'a G,
    rhs: &'a H,
) -> impl 'a + Iterator<Item = (&'a Node, &'a Node, &'a Node)>
where
    G: Graph,
    H: Graph,
{
    lhs.iter().filter(move |(s, p, o)| !rhs.contains(s, p, o))
}

pub fn symmetric_difference<'a, G, H>(
    lhs: &'a G,
    rhs: &'a H,
) -> impl 'a + Iterator<Item = (&'a Node, &'a Node, &'a Node)>
where
    G: Graph,
    H: Graph,
{
    difference(lhs, rhs).chain(difference(rhs, lhs))
}

pub fn intersection<'a, G, H>(
    lhs: &'a G,
    rhs: &'a H,
) -> impl 'a + Iterator<Item = (&'a Node, &'a Node, &'a Node)>
where
    G: Graph,
    H: Graph,
{
    lhs.iter().filter(move |(s, p, o)| rhs.contains(s, p, o))
}

pub fn union<'a, G, H>(
    lhs: &'a G,
    rhs: &'a H,
) -> impl 'a + Iterator<Item = (&'a Node, &'a Node, &'a Node)>
where
    G: Graph,
    H: Graph,
{
    lhs.iter().chain(difference(rhs, lhs))
}

pub fn is_subset<'a, G, H>(lhs: &'a G, rhs: &'a H) -> bool
where
    G: Graph,
    H: Graph,
{
    lhs.iter().all(|(s, p, o)| rhs.contains(s, p, o))
}

pub fn is_superset<'a, G, H>(lhs: &'a G, rhs: &'a H) -> bool
where
    G: Graph,
    H: Graph,
{
    is_subset(rhs, lhs)
}

pub fn is_disjoint<'a, G, H>(lhs: &'a G, rhs: &'a H) -> bool
where
    G: Graph,
    H: Graph,
{
    intersection(lhs, rhs).next().is_none()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn difference() {
        let testbed = Testbed::new(HashGraph::new());

        let a = testbed.graph();
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

        let difference: HashGraph = set::difference(&a, &b).collect();

        assert_eq!(2, difference.len());
        assert!(difference.contains(&testbed.node_b, &testbed.predicate_b, &testbed.node_c));
        assert!(difference.contains(&testbed.node_c, &testbed.predicate_c, &testbed.node_a));
    }

    #[test]
    fn symmetric_difference() {
        let testbed = Testbed::new(HashGraph::new());

        let a = testbed.graph();
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

        let difference: HashGraph = set::symmetric_difference(&a, &b).collect();

        assert_eq!(3, difference.len());
        assert!(difference.contains(&testbed.node_b, &testbed.predicate_b, &testbed.node_c));
        assert!(difference.contains(&testbed.node_c, &testbed.predicate_c, &testbed.node_a));
        assert!(difference.contains(&testbed.node_b, &testbed.predicate_a, &testbed.node_a));
    }

    #[test]
    fn union() {
        let testbed = Testbed::new(HashGraph::new());

        let a = testbed.graph();
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

        let union: HashGraph = set::union(&a, &b).collect();

        assert_eq!(4, union.len());
        assert!(union.contains(&testbed.node_a, &testbed.predicate_a, &testbed.node_b));
        assert!(union.contains(&testbed.node_b, &testbed.predicate_b, &testbed.node_c));
        assert!(union.contains(&testbed.node_c, &testbed.predicate_c, &testbed.node_a));
        assert!(union.contains(&testbed.node_b, &testbed.predicate_a, &testbed.node_a));
    }

    #[test]
    fn is_subset_superset() {
        let testbed = Testbed::new(HashGraph::new());

        let a = testbed.graph();
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

        assert!(set::is_subset(&a, &a));
        assert!(set::is_superset(&a, &a));

        assert!(!set::is_subset(&b, &a));
        assert!(!set::is_subset(&a, &b));

        b.remove(&testbed.node_b, &testbed.predicate_a, &testbed.node_a);

        assert!(set::is_subset(&b, &a));
        assert!(set::is_superset(&a, &b));
    }

    #[test]
    fn is_disjoint() {
        let testbed = Testbed::new(HashGraph::new());

        let a = testbed.graph();
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

        assert!(!set::is_disjoint(&a, &a));
        assert!(!set::is_disjoint(&a, &b));

        b.remove(&testbed.node_a, &testbed.predicate_a, &testbed.node_b);

        assert!(set::is_disjoint(&a, &b));
    }
}
