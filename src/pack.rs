use super::def::*;
use super::node::*;
use super::rule::*;

/// A group of regular and special nodes
pub struct RadixPack<'a, V> {
    /// The most common nodes, utilizing sparse arrays to accelerate queries
    pub regular: SparseSet<RadixNode<'a, V>>,

    /// Nodes which need to be checked one by one to determine if they match
    pub special: IndexMap<&'a str, RadixNode<'a, V>>,
}

impl<'a, V> RadixPack<'a, V> {
    /// Check if the group is empty
    ///
    /// ```
    /// use radixmap::{pack::RadixPack};
    ///
    /// assert!(RadixPack::<'_, ()>::default().is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.regular.is_empty() && self.special.is_empty()
    }

    /// Insert new node
    ///
    /// ```
    /// use radixmap::{pack::RadixPack, rule::RadixRule, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut pack = RadixPack::<'_, ()>::default();
    ///
    ///     // inserting different nodes into the pack
    ///     assert_eq!(pack.insert(RadixRule::from_plain("/api")?)?.rule, "/api");
    ///     assert_eq!(pack.insert(RadixRule::from_param(":id")?)?.rule, ":id");
    ///     assert_eq!(pack.insert(RadixRule::from_regex("{}")?)?.rule, "{}");
    ///
    ///     assert_eq!(pack.regular.len(), 1);
    ///     assert_eq!(pack.special.len(), 2);
    ///
    ///     // inserting duplicate nodes is meaningless
    ///     assert_eq!(pack.insert(RadixRule::from_plain("/api")?)?.rule, "/api");
    ///     assert_eq!(pack.insert(RadixRule::from_param(":id")?)?.rule, ":id");
    ///     assert_eq!(pack.insert(RadixRule::from_regex("{}")?)?.rule, "{}");
    ///
    ///     assert_eq!(pack.regular.len(), 1);
    ///     assert_eq!(pack.special.len(), 2);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn insert(&mut self, rule: RadixRule<'a>) -> RadixResult<&mut RadixNode<'a, V>> {
        // special nodes inserted directly into map
        let frag = rule.origin();
        if !matches!(rule, RadixRule::Plain { .. }) {
            return match self.special.contains_key(frag) {
                true => Ok(&mut self.special[frag]),
                false => Ok(self.special.entry(frag).or_insert(RadixNode::from(rule)))
            };
        }

        // use sparse array to find regular node, since the nodes of the tree share
        // prefixes, here it is only necessary to use the first byte for indexing
        let first = match frag.as_bytes().first() {
            Some(val) => *val as usize,
            None => return Err(RadixError::PathEmpty)
        };

        // insert regular node if no shared prefix
        if !self.regular.contains(first) {
            self.regular.insert(first, RadixNode::from(rule));
            return match self.regular.get_mut(first) {
                Some(node) => Ok(node),
                None => unreachable!()
            };
        }

        // compare the path with the existing node
        let found = match self.regular.get_mut(first) {
            Some(node) => node,
            None => unreachable!()
        };
        let (share, order) = found.rule.longest(frag);

        match order {
            Ordering::Greater => {
                let node = found.divide(share.len())?;
                found.next.regular.insert(node.rule.origin().as_bytes()[0] as usize, node);
                found.next.insert(RadixRule::try_from(&frag[share.len()..])?)
            }
            Ordering::Equal => {
                match frag.len().cmp(&share.len()) {
                    Ordering::Greater => found.next.insert(RadixRule::try_from(&frag[share.len()..])?),
                    Ordering::Equal => Ok(found),
                    Ordering::Less => unreachable!(),
                }
            }
            Ordering::Less => unreachable!(),
        }
    }

    /// Clear the nodes and preserve its capacity
    ///
    /// ```
    /// use radixmap::{pack::RadixPack, rule::RadixRule, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut pack = RadixPack::<'_, ()>::default();
    ///     pack.insert(RadixRule::from_plain("/api")?)?;
    ///     pack.insert(RadixRule::from_param(":id")?)?;
    ///     pack.insert(RadixRule::from_regex("{}")?)?;
    ///
    ///     assert_eq!(pack.regular.len(), 1);
    ///     assert_eq!(pack.special.len(), 2);
    ///
    ///     pack.clear();
    ///
    ///     assert!(pack.is_empty());
    ///
    ///     Ok(())
    /// }
    /// ```
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

// todo Debug

/// Clone the group
impl<'a, V: Clone> Clone for RadixPack<'a, V> {
    fn clone(&self) -> Self {
        let mut map = SparseSet::with_capacity(256);

        for obj in &self.regular {
            map.insert(obj.key(), obj.value.clone());
        }

        Self { regular: map, special: self.special.clone() }
    }
}

// todo Eq, PartialEq