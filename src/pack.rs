//! Pack represents a node's children
use super::defs::*;
use super::rule::*;
use super::node::RadixNode;

/// A group of regular and special nodes
pub struct RadixPack<'k, V> {
    /// The most common nodes, utilizing sparse arrays to accelerate queries
    pub regular: SparseSet<RadixNode<'k, V>>,

    /// Nodes which need to be checked one by one to determine if they match
    pub special: IndexMap<&'k str, RadixNode<'k, V>>,
}

impl<'k, V> RadixPack<'k, V> {
    /// Check if the group is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.regular.is_empty() && self.special.is_empty()
    }

    /// Iterate regular and special
    ///
    /// ```
    /// use radixmap::{pack::RadixPack, rule::RadixRule, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut pack = RadixPack::default();
    ///     pack.insert(RadixRule::try_from("/api")?)?;
    ///     pack.insert(RadixRule::try_from("{[0-9]+}")?)?;
    ///
    ///     let mut iter = pack.iter();
    ///     assert_eq!(iter.next().map(|node| node.rule), Some(RadixRule::from_plain("/api")));
    ///     assert_eq!(iter.next().map(|node| node.rule), Some(RadixRule::from_regex("{[0-9]+}")));
    ///     assert_eq!(iter.next(), None);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn iter(&self) -> Iter<'_, V> {
        Iter::from(self)
    }

    /// Iterate regular and special
    ///
    /// ```
    /// use radixmap::{pack::RadixPack, rule::RadixRule, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut pack = RadixPack::default();
    ///     pack.insert(RadixRule::try_from("/api")?)?;
    ///     pack.insert(RadixRule::try_from("{[0-9]+}")?)?;
    ///
    ///     let mut iter = pack.iter_mut();
    ///     assert_eq!(iter.next().map(|node| node.rule), Some(RadixRule::from_plain("/api")));
    ///     assert_eq!(iter.next().map(|node| node.rule), Some(RadixRule::from_regex("{[0-9]+}")));
    ///     assert_eq!(iter.next(), None);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn iter_mut(&'k mut self) -> IterMut<'_, V> {
        IterMut::from(self)
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
    ///     // inserting duplicate nodes has no effect
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
    pub fn insert(&mut self, rule: RadixRule<'k>) -> RadixResult<&mut RadixNode<'k, V>> {
        // special nodes inserted directly into map
        let frag = rule.origin();
        if !matches!(rule, RadixRule::Plain { .. }) {
            return match self.special.contains_key(frag) {
                true => Ok(&mut self.special[frag]),
                false => Ok(self.special.entry(frag).or_insert(RadixNode::from(rule)))
            };
        }

        // Use sparse array to find regular node. Since tree nodes
        // share prefixes, indexing only the first byte is sufficient
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

        // divide the node into two parts
        if order == Ordering::Greater {
            let node = found.divide(share.len())?;
            found.next.regular.insert(node.rule.origin().as_bytes()[0] as usize, node);
        }

        // insert the remaining path if found
        match frag.len().cmp(&share.len()) {
            Ordering::Greater => found.next.insert(RadixRule::try_from(&frag[share.len()..])?),
            Ordering::Equal => Ok(found),
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

/// Default Trait
impl<'k, V> Default for RadixPack<'k, V> {
    fn default() -> Self {
        Self { regular: SparseSet::with_capacity(256), special: IndexMap::new() }
    }
}

// todo Debug

/// Clone Trait
impl<'k, V: Clone> Clone for RadixPack<'k, V> {
    fn clone(&self) -> Self {
        let mut map = SparseSet::with_capacity(256);

        for obj in &self.regular {
            map.insert(obj.key(), obj.value.clone());
        }

        Self { regular: map, special: self.special.clone() }
    }
}

// todo Eq, PartialEq

// -----------------------------------------------------------------------------

/// Iterate regular and special
#[derive(Default, Clone)]
pub struct Iter<'k, V> {
    onetime: Option<&'k RadixNode<'k, V>>,
    regular: std::slice::Iter<'k, sparseset::Entry<RadixNode<'k, V>>>,
    special: indexmap::map::Values<'k, &'k str, RadixNode<'k, V>>,
}

impl<'k, V> From<&'k RadixNode<'k, V>> for Iter<'k, V> {
    fn from(value: &'k RadixNode<'k, V>) -> Self {
        Self { onetime: Some(value), regular: Default::default(), special: Default::default() }
    }
}

impl<'k, V> From<&'k RadixPack<'k, V>> for Iter<'k, V> {
    fn from(value: &'k RadixPack<'k, V>) -> Self {
        Self { onetime: None, regular: value.regular.iter(), special: value.special.values() }
    }
}

impl<'k, V> Iterator for Iter<'k, V> {
    type Item = &'k RadixNode<'k, V>;

    fn next(&mut self) -> Option<Self::Item> {
        self.onetime.take().or(self.regular.next().map(|node| node.value())).or(self.special.next())
    }
}

// -----------------------------------------------------------------------------

/// Iterate regular and special
#[derive(Default)]
pub struct IterMut<'k, V> {
    onetime: Option<&'k mut RadixNode<'k, V>>,
    regular: std::slice::IterMut<'k, sparseset::Entry<RadixNode<'k, V>>>,
    special: indexmap::map::ValuesMut<'k, &'k str, RadixNode<'k, V>>,
}

impl<'k, V> From<&'k mut RadixNode<'k, V>> for IterMut<'k, V> {
    fn from(value: &'k mut RadixNode<'k, V>) -> Self {
        Self { onetime: Some(value), regular: Default::default(), special: Default::default() }
    }
}

impl<'k, V> From<&'k mut RadixPack<'k, V>> for IterMut<'k, V> {
    fn from(value: &'k mut RadixPack<'k, V>) -> Self {
        Self { onetime: None, regular: value.regular.iter_mut(), special: value.special.values_mut() }
    }
}

impl<'k, V> Iterator for IterMut<'k, V> {
    type Item = &'k mut RadixNode<'k, V>;

    fn next(&mut self) -> Option<Self::Item> {
        self.onetime.take().or(self.regular.next().map(|node| node.value_mut())).or(self.special.next())
    }
}