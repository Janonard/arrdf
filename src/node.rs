use std::rc::Rc;

#[derive(Clone, Eq)]
pub struct Node {
    referent: Rc<str>,
}

impl<'a> From<&'a str> for Node {
    fn from(referent: &'a str) -> Self {
        Self {
            referent: Rc::from(referent),
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

impl Node {
    pub fn blank() -> Self {
        Self {
            referent: Rc::from(""),
        }
    }

    pub fn is_blank(&self) -> bool {
        self.referent.is_empty()
    }

    pub fn as_str(&self) -> &str {
        self.referent.as_ref()
    }

    pub fn internal(&self) -> &Rc<str> {
        &self.referent
    }
}
