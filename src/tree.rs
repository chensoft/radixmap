use crate::*;

#[derive(Default)]
pub struct RadixTree<'a, T> {
    root: RadixNode<'a, T>,
}

impl<'a, T> RadixTree<'a, T> {
    pub fn insert(&mut self, key: &'a str, val: T) -> &mut Self {
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