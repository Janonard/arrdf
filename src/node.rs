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

    #[cfg(not(tarpaulin_include))]
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

    #[cfg(not(tarpaulin_include))]
    pub fn internal(&self) -> &Arc<str> {
        &self.referent
    }
}
