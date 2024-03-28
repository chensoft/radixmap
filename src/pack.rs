use super::def::*;
use super::item::*;
use super::node::*;

/// A group of regular and special nodes
pub struct RadixPack<'a, V> {
    pub regular: SparseSet<RadixNode<'a, V>>,
    pub special: IndexMap<&'a str, RadixNode<'a, V>>,
}

/// Create a empty group
impl<'a, V> Default for RadixPack<'a, V> {
    fn default() -> Self {
        Self { regular: SparseSet::with_capacity(256), special: IndexMap::new() }
    }
}

impl<'a, V> RadixPack<'a, V> {
    /// Check if the group is empty
    pub fn is_empty(&self) -> bool {
        self.regular.is_empty() && self.special.is_empty()
    }

    pub fn insert(&mut self, size: &mut usize, frag: &'a str) -> Result<&mut RadixNode<'a, V>> {
        // special nodes inserted directly into map
        let item = RadixItem::new(frag)?;
        if !matches!(item, RadixItem::Plain { .. }) {
            return match self.special.contains_key(frag) {
                true => Ok(&mut self.special[frag]),
                false => Ok(self.special.entry(frag).or_insert(RadixNode::new(item, None).incr(size)))
            };
        }

        // use sparse array to find regular node
        let bytes = frag.as_bytes();
        let first = match bytes.first() {
            Some(val) => *val as usize,
            None => return Err(Error::PathEmpty.into())
        };

        if !self.regular.contains(first) {
            self.regular.insert(first, RadixNode::new(item, None).incr(size));
            return match self.regular.get_mut(first) {
                Some(node) => Ok(node),
                None => unreachable!()
            };
        }

        let found = match self.regular.get_mut(first) {
            Some(node) => node,
            None => unreachable!()
        };
        let (share, order) = found.item.longest(frag);

        match order {
            Ordering::Less => unreachable!(),
            Ordering::Equal => {
                match frag.len().cmp(&share.len()) {
                    Ordering::Less => unreachable!(),
                    Ordering::Equal => Ok(found),
                    Ordering::Greater => found.next.insert(size, &frag[share.len()..]),
                }
            }
            Ordering::Greater => {
                let node = found.divide(share.len())?.incr(size);
                let flag = node.item.origin().as_bytes()[0] as usize;
                found.next.regular.insert(flag, node);
                match found.next.regular.get_mut(flag) {
                    Some(node) => Ok(node),
                    None => unreachable!()
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.regular.clear();
        self.special.clear();
    }
}

impl<'a, V: Clone> Clone for RadixPack<'a, V> {
    fn clone(&self) -> Self {
        let mut map = SparseSet::with_capacity(256);

        for obj in &self.regular {
            map.insert(obj.key(), obj.value.clone());
        }

        Self { regular: map, special: self.special.clone() }
    }
}

// // -----------------------------------------------------------------------------
// 
// pub struct Iter<'a, V> {
//     regular: std::slice::Iter<'a, sparseset::Entry<RadixNode<'a, V>>>,
//     special: indexmap::map::Values<'a, &'a str, RadixNode<'a, V>>,
// }
// 
// impl<'a, V> Iter<'a, V> {
//     pub fn new(pack: &'a RadixPack<'a, V>) -> Self {
//         Self { regular: pack.regular.iter(), special: pack.special.values() }
//     }
// }
// 
// impl<'a, V> Iterator for Iter<'a, V> {
//     type Item = &'a RadixNode<'a, V>;
// 
//     fn next(&mut self) -> Option<Self::Item> {
//         if let Some(node) = self.regular.next() {
//             return Some(node.value());
//         }
// 
//         if let Some(node) = self.special.next() {
//             return Some(node);
//         }
// 
//         None
//     }
// }
// 
// // -----------------------------------------------------------------------------
// 
// pub struct IterMut<'a, V> {
//     regular: std::slice::IterMut<'a, sparseset::Entry<RadixNode<'a, V>>>,
//     special: indexmap::map::ValuesMut<'a, &'a str, RadixNode<'a, V>>,
// }
// 
// impl<'a, V> IterMut<'a, V> {
//     pub fn new(pack: &'a mut RadixPack<'a, V>) -> Self {
//         Self { regular: pack.regular.iter_mut(), special: pack.special.values_mut() }
//     }
// }
// 
// impl<'a, V> Iterator for IterMut<'a, V> {
//     type Item = &'a mut RadixNode<'a, V>;
// 
//     fn next(&mut self) -> Option<Self::Item> {
//         if let Some(node) = self.regular.next() {
//             return Some(node.value_mut());
//         }
// 
//         if let Some(node) = self.special.next() {
//             return Some(node);
//         }
// 
//         None
//     }
// }