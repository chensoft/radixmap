use super::def::*;
use super::item::*;
use super::tier::*;

pub struct RadixNode<'a, V> {
    pub item: RadixItem<'a>,
    pub data: Option<V>,
    pub next: RadixTier<'a, V>,
}

// todo impl clone and etc..., item, tier
impl<'a, V> Default for RadixNode<'a, V> {
    fn default() -> Self {
        Self { item: RadixItem::default(), data: None, next: RadixTier::default() }
    }
}

impl<'a, V> RadixNode<'a, V> {
    pub fn insert(&mut self, size: &mut usize, mut path: &'a str, data: V) -> Result<Option<V>> {
        // let part = RadixItem::segment(path)?;
        // let edge = self.next.insert(size, part)?;

        // todo if no more parts then set data
        todo!()
    }

    pub fn divide(&mut self, len: usize) {
        // let child = RadixNode {
        //     item: RadixItem::new(&self.item.pattern[len..]),
        //     next: std::mem::replace(&mut self.next, SparseSet::with_capacity(256)),
        //     data: std::mem::take(&mut self.data),
        // };
        // 
        // self.item = RadixItem::new(&self.item.pattern[..len]);
        // self.next.insert(child.item.pattern.as_bytes()[0] as usize, child);
        todo!()
    }

    pub fn deepest(&self, path: &'a str) -> Option<&RadixNode<'a, V>> {
        // if path.is_empty() {
        //     return Some(self);
        // }
        //
        // match self.next.get(path.as_bytes()[0] as usize) {
        //     Some(next) if next.longest(path).len() == next.item.pattern.len() => next.deepest(&path[next.item.pattern.len()..]),
        //     _ => None
        // }
        todo!()
    }
}