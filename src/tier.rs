use super::def::*;
use super::node::*;

pub struct RadixTier<'a, V> {
    pub regular: SparseSet<RadixNode<'a, V>>,
    pub special: Vec<RadixNode<'a, V>>,
}

impl<'a, V> Default for RadixTier<'a, V> {
    fn default() -> Self {
        Self { regular: SparseSet::with_capacity(256), special: vec![] }
    }
}

impl<'a, V> RadixTier<'a, V> {
    pub fn is_empty(&self) -> bool {
        self.regular.is_empty() && self.special.is_empty()
    }
}