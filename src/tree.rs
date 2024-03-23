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

    pub fn search(&self, path: &'a str) -> Option<&T> {
        match self.prefix(path).next() {
            Some(obj) => obj.data.as_ref(),
            None => None
        }
    }

    pub fn remove(&mut self) -> Option<T> {
        todo!()
    }

    // todo iter().with_prefix()
    pub fn prefix(&'a self, path: &'a str) -> RadixNodeIterator<'a, T> {
        RadixNodeIterator::new(&self.root, path)
    }
}