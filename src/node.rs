use super::def::*;
use super::item::*;
use super::tier::*;

// todo impl clone and etc..., item, tier
pub struct RadixNode<'a, V> {
    pub item: RadixItem<'a>,
    pub data: Option<V>,
    pub next: RadixTier<'a, V>,
}

impl<'a, V> Default for RadixNode<'a, V> {
    fn default() -> Self {
        Self { item: RadixItem::default(), data: None, next: RadixTier::default() }
    }
}

impl<'a, V> RadixNode<'a, V> {
    pub fn new(item: RadixItem<'a>, data: Option<V>) -> Self {
        Self { item, data, next: RadixTier::default() }
    }

    pub fn incr(self, size: &mut usize) -> Self {
        *size += 1;
        self
    }

    pub fn insert(&mut self, size: &mut usize, path: &'a str, data: V) -> Result<Option<V>> {
        let next = RadixItem::extract(path)?;
        let edge = self.next.insert(size, next)?;

        if next.len() == path.len() {
            let prev = std::mem::take(&mut edge.data);
            edge.data = Some(data);
            return Ok(prev);
        }

        edge.insert(size, &path[next.len()..], data)
    }

    pub fn divide(&mut self, len: usize) -> Result<RadixNode<'a, V>> {
        Ok(RadixNode {
            item: self.item.divide(len)?,
            data: std::mem::take(&mut self.data),
            next: std::mem::take(&mut self.next),
        })
    }
}