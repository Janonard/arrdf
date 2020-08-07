use std::sync::Arc;

#[derive(Clone, Eq)]
pub struct Node {
    referent: Arc<str>,
}

impl<'a> From<&'a str> for Node {
    fn from(referent: &'a str) -> Self {
        Self {
            referent: Arc::from(referent),
        }
    }
}

impl std::cmp::PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        if self.is_blank() {
            std::ptr::eq(self.referent.as_ptr(), other.referent.as_ptr())
        } else {
            self.as_str() == other.as_str()
        }
    }
}

impl std::hash::Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        if self.is_blank() {
            self.referent.as_ptr().hash(state);
        } else {
            self.as_str().hash(state);
        }
    }
}

impl std::ops::Deref for Node {
    type Target = str;

    fn deref(&self) -> &str {
        self.referent.as_ref()
    }
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        if self.is_blank() {
            f.write_fmt(format_args!("Node <{:?}>", self.referent.as_ptr()))
        } else {
            f.write_fmt(format_args!("Node {:?}", self.referent.as_ref()))
        }
    }
}

impl Node {
    pub fn blank() -> Self {
        Self {
            referent: Arc::from(""),
        }
    }

    pub fn is_blank(&self) -> bool {
        self.referent.is_empty()
    }

    pub fn as_str(&self) -> &str {
        self.referent.as_ref()
    }

    pub fn internal(&self) -> &Arc<str> {
        &self.referent
    }
}

#[cfg(test)]
mod tests {
    use crate::node::Node;
    use std::collections::HashMap;

    #[test]
    fn equivalance() {
        let blank_a = Node::blank();
        let blank_b = blank_a.clone();
        let blank_c = Node::blank();
        let blanks = [&blank_a, &blank_b, &blank_c];

        assert_eq!(blank_a, blank_b);
        assert_ne!(blank_a, blank_c);
        assert_ne!(blank_b, blank_c);

        let node_a = Node::from("Hello");
        let node_b = node_a.clone();
        let node_c = Node::from("Hello");
        let node_d = Node::from("World");
        let nodes = [&node_a, &node_b, &node_c, &node_d];

        assert_eq!(node_a, node_b);
        assert_eq!(node_a, node_c);
        assert_ne!(node_a, node_d);
        assert_eq!(node_b, node_c);
        assert_ne!(node_b, node_d);
        assert_ne!(node_c, node_d);

        for blank in &blanks {
            for node in &nodes {
                assert_ne!(*blank, *node);
            }
        }

        let mut map: HashMap<Node, Node> = HashMap::new();
        for node in Iterator::chain(blanks.iter(), nodes.iter()) {
            map.insert((*node).clone(), (*node).clone());
        }

        assert_eq!(4, map.len());
        assert_eq!(blank_b, map[&blank_a]);
        assert_eq!(blank_b, map[&blank_b]);
        assert_eq!(blank_c, map[&blank_c]);
        assert_eq!(node_c, map[&node_a]);
        assert_eq!(node_c, map[&node_b]);
        assert_eq!(node_c, map[&node_c]);
        assert_eq!(node_d, map[&node_d]);
    }

    #[test]
    fn formatting() {
        let node = Node::from("Hello");
        assert_eq!("Node \"Hello\"", format!("{:?}", node));

        let node = Node::blank();
        assert_eq!(
            format!("Node <{:?}>", node.internal().as_ptr()),
            format!("{:?}", node)
        );
    }
}
