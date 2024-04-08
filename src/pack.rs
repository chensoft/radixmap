//! Pack represents a node's children
use super::defs::*;
use super::rule::*;
use super::node::RadixNode;

/// A group of regular and special nodes
#[derive(Clone)]
pub struct RadixPack<'k, V> {
    /// The most common nodes, utilizing sparse arrays to accelerate queries
    pub regular: Vec<Option<RadixNode<'k, V>>>,

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
    ///     let mut pack = RadixPack::<'_, ()>::default();
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
    pub fn iter(&self) -> Iter<'_, V> {
        Iter::from(self)
    }

    /// Iterate regular and special
    ///
    /// ```
    /// use radixmap::{pack::RadixPack, rule::RadixRule, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut pack = RadixPack::<'_, ()>::default();
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
    pub fn iter_mut(&mut self) -> IterMut<'_, 'k, V> {
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
    ///     assert_eq!(pack.insert(RadixRule::from_regex("{[0-9]+}")?)?.rule, "{[0-9]+}");
    ///
    ///     assert_eq!(pack.regular.len(), 1);
    ///     assert_eq!(pack.special.len(), 2);
    ///
    ///     // inserting duplicate nodes has no effect
    ///     assert_eq!(pack.insert(RadixRule::from_plain("/api")?)?.rule, "/api");
    ///     assert_eq!(pack.insert(RadixRule::from_param(":id")?)?.rule, ":id");
    ///     assert_eq!(pack.insert(RadixRule::from_regex("{[0-9]+}")?)?.rule, "{[0-9]+}");
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
        self.ensure(first);

        if self.regular[first].is_none() {
            self.regular[first] = Some(RadixNode::from(rule));
            return match &mut self.regular[first] {
                Some(node) => Ok(node),
                _ => unreachable!()
            };
        }

        // compare the path with the existing node
        let found = match &mut self.regular[first] {
            Some(node) => node,
            _ => unreachable!()
        };
        let (share, order) = found.rule.longest(frag);

        // divide the node into two parts
        if order == Ordering::Greater {
            let node = found.divide(share.len())?;
            let byte = node.rule.origin().as_bytes()[0] as usize;

            found.next.ensure(byte);
            found.next.regular[byte] = Some(node);
        }

        // insert the remaining path if found
        match frag.len().cmp(&share.len()) {
            Ordering::Greater => found.next.insert(RadixRule::try_from(&frag[share.len()..])?),
            Ordering::Equal => Ok(found),
            Ordering::Less => unreachable!(),
        }
    }

    /// Expand regular container if below `byte + 1` size
    #[inline]
    pub fn ensure(&mut self, byte: usize) {
        if self.regular.len() < byte + 1 {
            self.regular.resize_with(byte + 1, || None);
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
impl<'k, V> Default for RadixPack<'k, V> {
    #[inline]
    fn default() -> Self {
        Self { regular: vec![], special: IndexMap::new() }
    }
}

/// Debug trait
impl<'k, V: Debug> Debug for RadixPack<'k, V> {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

// -----------------------------------------------------------------------------

/// Iterate regular and special
#[derive(Default, Clone)]
pub struct Iter<'k, V> {
    onetime: Option<&'k RadixNode<'k, V>>,
    regular: std::slice::Iter<'k, Option<RadixNode<'k, V>>>, // todo improve
    special: indexmap::map::Values<'k, &'k str, RadixNode<'k, V>>,
}

impl<'k, V> From<&'k RadixNode<'k, V>> for Iter<'k, V> {
    #[inline]
    fn from(value: &'k RadixNode<'k, V>) -> Self {
        Self { onetime: Some(value), regular: Default::default(), special: Default::default() }
    }
}

impl<'k, V> From<&'k RadixPack<'k, V>> for Iter<'k, V> {
    #[inline]
    fn from(value: &'k RadixPack<'k, V>) -> Self {
        Self { onetime: None, regular: value.regular.iter(), special: value.special.values() }
    }
}

impl<'k, V> Iterator for Iter<'k, V> {
    type Item = &'k RadixNode<'k, V>;

    #[inline]
    #[allow(clippy::while_let_on_iterator)]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.onetime.take() {
            return Some(node);
        }

        while let Some(node) = self.regular.next() {
            if let Some(node) = node {
                return Some(node);
            }
        }

        self.special.next()
    }
}

// -----------------------------------------------------------------------------

/// Iterate regular and special
#[derive(Default)]
pub struct IterMut<'n, 'k, V> {
    onetime: Option<&'n mut RadixNode<'k, V>>,
    regular: std::slice::IterMut<'n, Option<RadixNode<'k, V>>>,
    special: indexmap::map::ValuesMut<'n, &'k str, RadixNode<'k, V>>,
}

impl<'n, 'k, V> From<&'n mut RadixNode<'k, V>> for IterMut<'n, 'k, V> {
    #[inline]
    fn from(value: &'n mut RadixNode<'k, V>) -> Self {
        Self { onetime: Some(value), regular: Default::default(), special: Default::default() }
    }
}

impl<'n, 'k, V> From<&'n mut RadixPack<'k, V>> for IterMut<'n, 'k, V> {
    #[inline]
    fn from(value: &'n mut RadixPack<'k, V>) -> Self {
        Self { onetime: None, regular: value.regular.iter_mut(), special: value.special.values_mut() }
    }
}

impl<'n, 'k, V> Iterator for IterMut<'n, 'k, V> {
    type Item = &'n mut RadixNode<'k, V>;

    #[inline]
    #[allow(clippy::while_let_on_iterator)]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.onetime.take() {
            return Some(node);
        }

        while let Some(node) = self.regular.next() {
            if let Some(node) = node {
                return Some(node);
            }
        }

        self.special.next()
    }
}