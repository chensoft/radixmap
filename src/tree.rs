use crate::*;

pub struct RadixTree<'a, T> {
    root: RadixNode<'a, T>,
}

impl<'a, T> Default for RadixTree<'a, T> {
    fn default() -> Self {
        Default::default()
    }
}

impl<'a, T> RadixTree<'a, T> {
    pub fn insert(&mut self, key: impl Into<Cow<'a, str>>, val: T) -> &mut Self {
        self
    }

    pub fn search(&mut self) {
        todo!()
    }

    pub fn remove(&mut self) {
        todo!()
    }

    pub fn prefix(&mut self) {
        // todo impl -> Iterator
        todo!()
    }
}