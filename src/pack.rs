//! Pack represents a node's children
use super::defs::*;
use super::rule::*;
use super::node::RadixNode;

/// A group of regular and special nodes
#[derive(Clone)]
pub struct RadixPack<V> {
    /// The most common nodes, utilizing vector map to accelerate queries
    pub regular: VecMap<RadixNode<V>>,

    /// Nodes which need to be checked one by one to determine if they match
    pub special: IndexMap<Bytes, RadixNode<V>>,
}

impl<V> RadixPack<V> {
    /// Check if the group is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.regular.is_empty() && self.special.is_empty()
    }

    /// Iterate regular and special
    ///
    /// # Examples
    ///
    /// ```
    /// use radixmap::{pack::RadixPack, rule::RadixRule, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut pack = RadixPack::<()>::default();
    ///     pack.insert(RadixRule::try_from("/api")?)?;
    ///     pack.insert(RadixRule::try_from("{[0-9]+}")?)?;
    ///
    ///     let mut iter = pack.iter();
    ///     assert_eq!(iter.next().map(|node| &node.rule), Some(&RadixRule::from_plain("/api")?));
    ///     assert_eq!(iter.next().map(|node| &node.rule), Some(&RadixRule::from_regex("{[0-9]+}")?));
    ///     assert_eq!(iter.next().map(|node| &node.rule), None);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn iter(&self) -> Iter<V> {
        Iter::from(self)
    }

    /// Iterate regular and special
    ///
    /// # Examples
    ///
    /// ```
    /// use radixmap::{pack::RadixPack, rule::RadixRule, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut pack = RadixPack::<()>::default();
    ///     pack.insert(RadixRule::try_from("/api")?)?;
    ///     pack.insert(RadixRule::try_from("{[0-9]+}")?)?;
    ///
    ///     // test with multiple calls
    ///     let _ = pack.iter_mut();
    ///     let _ = pack.iter_mut();
    ///
    ///     // test the iteration method
    ///     let mut iter = pack.iter_mut();
    ///     assert_eq!(iter.next().map(|node| &node.rule), Some(&RadixRule::from_plain("/api")?));
    ///     assert_eq!(iter.next().map(|node| &node.rule), Some(&RadixRule::from_regex("{[0-9]+}")?));
    ///     assert_eq!(iter.next().map(|node| &node.rule), None);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<V> {
        IterMut::from(self)
    }

    /// Insert new node
    ///
    /// # Examples
    ///
    /// ```
    /// use radixmap::{pack::RadixPack, rule::RadixRule, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut pack = RadixPack::<()>::default();
    ///
    ///     // inserting different nodes into the pack
    ///     assert_eq!(pack.insert(RadixRule::from_plain("/api")?)?.rule, b"/api");
    ///     assert_eq!(pack.insert(RadixRule::from_param(":id")?)?.rule, b":id");
    ///     assert_eq!(pack.insert(RadixRule::from_regex("{[0-9]+}")?)?.rule, b"{[0-9]+}");
    ///
    ///     assert_eq!(pack.regular.len(), 1);
    ///     assert_eq!(pack.special.len(), 2);
    ///
    ///     // inserting duplicate nodes has no effect
    ///     assert_eq!(pack.insert(RadixRule::from_plain("/api")?)?.rule, b"/api");
    ///     assert_eq!(pack.insert(RadixRule::from_param(":id")?)?.rule, b":id");
    ///     assert_eq!(pack.insert(RadixRule::from_regex("{[0-9]+}")?)?.rule, b"{[0-9]+}");
    ///
    ///     assert_eq!(pack.regular.len(), 1);
    ///     assert_eq!(pack.special.len(), 2);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn insert(&mut self, rule: RadixRule) -> RadixResult<&mut RadixNode<V>> {
        // special nodes inserted directly into map
        let frag = rule.origin();
        if !matches!(rule, RadixRule::Plain { .. }) {
            return match self.special.contains_key(frag) {
                true => Ok(&mut self.special[frag]),
                false => Ok(self.special.entry(frag.clone()).or_insert(RadixNode::from(rule)))
            };
        }

        // Use vector map to find regular node. Since tree nodes
        // share prefixes, indexing only the first byte is sufficient
        let first = *frag.first().ok_or(RadixError::PathEmpty)? as usize;

        // insert regular node if no shared prefix
        if !self.regular.contains_key(first) {
            self.regular.insert(first, RadixNode::from(rule));
            return match self.regular.get_mut(first) {
                Some(node) => Ok(node),
                _ => unreachable!()
            };
        }

        // compare the path with the existing node
        let found = match self.regular.get_mut(first) {
            Some(node) => node,
            _ => unreachable!()
        };
        let share = found.rule.longest(frag.as_ref()).unwrap_or(b"");
        let equal = found.rule.is_special() || found.rule.origin().len() == share.len();

        // divide the node into two parts
        if !equal {
            let node = found.divide(share.len())?;
            let byte = node.rule.origin()[0] as usize;
            found.next.regular.insert(byte, node);
        }

        // insert the remaining path if found
        match frag.len() != share.len() {
            true => found.next.insert(RadixRule::try_from(frag.slice(share.len()..))?),
            false => Ok(found),
        }
    }

    /// Clear the nodes and preserve its capacity
    ///
    /// # Examples
    ///
    /// ```
    /// use radixmap::{pack::RadixPack, rule::RadixRule, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut pack = RadixPack::<()>::default();
    ///     pack.insert(RadixRule::from_plain("/api")?)?;
    ///     pack.insert(RadixRule::from_param(":id")?)?;
    ///     pack.insert(RadixRule::from_regex("{}")?)?;
    ///
    ///     assert_eq!(pack.regular.len(), 1);
    ///     assert_eq!(pack.special.len(), 2);
    ///
    ///     pack.clear();
    ///
    ///     assert_eq!(pack.is_empty(), true);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        self.regular.clear();
        self.special.clear();
    }
}

/// Default Trait
impl<V> Default for RadixPack<V> {
    #[inline]
    fn default() -> Self {
        Self { regular: VecMap::new(), special: IndexMap::new() }
    }
}

// -----------------------------------------------------------------------------

/// Iterate regular and special
#[derive(Default, Clone)]
pub struct Iter<'n, V> {
    onetime: Option<&'n RadixNode<V>>,
    regular: Option<vec_map::Values<'n, RadixNode<V>>>,
    special: indexmap::map::Values<'n, Bytes, RadixNode<V>>,
}

impl<'n, V> From<&'n RadixNode<V>> for Iter<'n, V> {
    #[inline]
    fn from(value: &'n RadixNode<V>) -> Self {
        Self { onetime: Some(value), regular: None, special: Default::default() }
    }
}

impl<'n, V> From<&'n RadixPack<V>> for Iter<'n, V> {
    #[inline]
    fn from(value: &'n RadixPack<V>) -> Self {
        Self { onetime: None, regular: Some(value.regular.values()), special: value.special.values() }
    }
}

impl<'n, V> Iterator for Iter<'n, V> {
    type Item = &'n RadixNode<V>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.onetime.take() {
            return Some(node);
        }

        if let Some(iter) = &mut self.regular {
            if let Some(node) = iter.next() {
                return Some(node);
            }
        }

        self.special.next()
    }
}

// -----------------------------------------------------------------------------

/// Iterate regular and special
#[derive(Default)]
pub struct IterMut<'n, V> {
    onetime: Option<&'n mut RadixNode<V>>,
    regular: Option<vec_map::ValuesMut<'n, RadixNode<V>>>,
    special: indexmap::map::ValuesMut<'n, Bytes, RadixNode<V>>,
}

impl<'n, V> From<&'n mut RadixNode<V>> for IterMut<'n, V> {
    #[inline]
    fn from(value: &'n mut RadixNode<V>) -> Self {
        Self { onetime: Some(value), regular: None, special: Default::default() }
    }
}

impl<'n, V> From<&'n mut RadixPack<V>> for IterMut<'n, V> {
    #[inline]
    fn from(value: &'n mut RadixPack<V>) -> Self {
        Self { onetime: None, regular: Some(value.regular.values_mut()), special: value.special.values_mut() }
    }
}

impl<'n, V> Iterator for IterMut<'n, V> {
    type Item = &'n mut RadixNode<V>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.onetime.take() {
            return Some(node);
        }

        if let Some(iter) = &mut self.regular {
            if let Some(node) = iter.next() {
                return Some(node);
            }
        }

        self.special.next()
    }
}