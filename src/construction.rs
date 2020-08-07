use crate::*;

impl HashGraph {
    pub fn new() -> Self {
        HashGraph {
            nodes: HashMap::new(),
        }
    }

    #[cfg(not(tarpaulin_include))]
    pub fn with_capacity(n_subjects: usize) -> Self {
        HashGraph {
            nodes: HashMap::with_capacity(n_subjects),
        }
    }

    #[cfg(not(tarpaulin_include))]
    pub fn shrink_to_fit(&mut self) {
        self.nodes.retain(|_, relationships| {
            relationships.shrink_to_fit();
            !relationships.is_empty()
        })
    }
}