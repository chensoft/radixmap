use super::def::*;
use super::rule::*;
use super::node::*;

/// A group of regular and special nodes
pub struct RadixPack<'a, V> {
    pub regular: SparseSet<RadixNode<'a, V>>,
    pub special: IndexMap<&'a str, RadixNode<'a, V>>,
}

impl<'a, V> RadixPack<'a, V> {
    /// Check if the group is empty
    pub fn is_empty(&self) -> bool {
        self.regular.is_empty() && self.special.is_empty()
    }

    pub fn insert(&mut self, frag: &'a str) -> RadixResult<&mut RadixNode<'a, V>> {
        // special nodes inserted directly into map
        let rule = RadixRule::new(frag)?;
        if !matches!(rule, RadixRule::Plain { .. }) {
            return match self.special.contains_key(frag) {
                true => Ok(&mut self.special[frag]),
                false => Ok(self.special.entry(frag).or_insert(RadixNode::from(rule)))
            };
        }

        // use sparse array to find regular node
        let bytes = frag.as_bytes();
        let first = match bytes.first() {
            Some(val) => *val as usize,
            None => return Err(RadixError::PathEmpty)
        };

        if !self.regular.contains(first) {
            self.regular.insert(first, RadixNode::from(rule));
            return match self.regular.get_mut(first) {
                Some(node) => Ok(node),
                None => unreachable!()
            };
        }

        let found = match self.regular.get_mut(first) {
            Some(node) => node,
            None => unreachable!()
        };
        let (share, order) = found.rule_ref().longest(frag);

        match order {
            Ordering::Less => unreachable!(),
            Ordering::Equal => {
                match frag.len().cmp(&share.len()) {
                    Ordering::Less => unreachable!(),
                    Ordering::Equal => Ok(found),
                    Ordering::Greater => found.next_mut().insert(&frag[share.len()..]),
                }
            }
            Ordering::Greater => {
                let node = found.divide(share.len())?;
                found.next_mut().regular.insert(node.rule_ref().origin().as_bytes()[0] as usize, node);
                found.next_mut().insert(&frag[share.len()..])
            }
        }
    }

    pub fn clear(&mut self) {
        self.regular.clear();
        self.special.clear();
    }
}

/// Create a empty group
impl<'a, V> Default for RadixPack<'a, V> {
    fn default() -> Self {
        Self { regular: SparseSet::with_capacity(256), special: IndexMap::new() }
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