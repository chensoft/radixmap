use super::def::*;
use super::item::*;
use super::tier::*;

pub struct RadixNode<'a, V> {
    pub item: RadixItem<'a>,
    pub data: Option<V>,
    pub prev: Option<&'a RadixNode<'a, V>>,
    pub next: RadixTier<'a, V>,
}

impl<'a, V> Default for RadixNode<'a, V> {
    fn default() -> Self {
        Self { item: RadixItem::default(), data: None, prev: None, next: RadixTier::default() }
    }
}

impl<'a, V> RadixNode<'a, V> {
    pub fn fullpath(&self) -> String {
        todo!()
    }

    pub fn longest(&self, path: &'a str) -> &'a str {
        self.item.longest(path)
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

    pub fn insert(&mut self, size: &mut usize, path: &'a str, data: V) -> Result<Option<V>> {
        if path.is_empty() {
            return Err(Error::PathEmpty.into());
        }

        //     let edge = match self.next.get_mut(path.as_bytes()[0] as usize) {
        //         Some(obj) => obj,
        //         None => {
        //             self.next.insert(path.as_bytes()[0] as usize, RadixNode::new(path, Some(data)));
        //             return;
        //         }
        //     };
        // 
        //     let share = edge.longest(path);
        // 
        //     match edge.item.pattern.len().cmp(&share.len()) {
        //         Ordering::Less => unreachable!(),
        //         Ordering::Equal => {
        //             match path.len().cmp(&share.len()) {
        //                 Ordering::Less => unreachable!(),
        //                 Ordering::Equal => edge.data = Some(data),
        //                 Ordering::Greater => edge.insert(&path[share.len()..], data),
        //             }
        //         }
        //         Ordering::Greater => {
        //             edge.divide(share.len());
        //             edge.insert(&path[share.len()..], data);
        //         }
        //     }
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
}