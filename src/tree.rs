use crate::*;

#[derive(Default)]
pub struct RadixTree<'a, T> {
    root: RadixNode<'a, T>,
}

// todo support chars-based search
impl<'a, T> RadixTree<'a, T> {
    pub fn insert(&mut self, path: &'a str, data: T) -> &mut Self {
        self.root.insert(path, data);
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