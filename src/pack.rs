use super::def::*;
use super::item::*;
use super::node::*;

pub struct RadixPack<'a, V> {
    pub regular: SparseSet<RadixNode<'a, V>>,
    pub special: IndexMap<&'a str, RadixNode<'a, V>>,
}

impl<'a, V> Default for RadixPack<'a, V> {
    fn default() -> Self {
        Self { regular: SparseSet::with_capacity(256), special: IndexMap::new() }
    }
}

impl<'a, V> RadixPack<'a, V> {
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

    pub fn is_empty(&self) -> bool {
        self.regular.is_empty() && self.special.is_empty()
    }
}