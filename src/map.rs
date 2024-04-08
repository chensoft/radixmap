//! Radix map implementation
use super::defs::*;
use super::node;
use super::node::RadixNode;

/// The radix map where the key is &'k str and the value is arbitrary data
pub struct RadixMap<'k, V> {
    /// The root node, always empty
    root: RadixNode<'k, V>,

    /// The number of data nodes
    size: usize,
}

impl<'k, V> RadixMap<'k, V> {
    /// For consistency with the standard library, we provide this fn to create an empty map
    pub fn new() -> Self {
        Default::default()
    }

    /// The data nodes' count, note that RadixMap ignores empty nodes
    ///
    /// ```
    /// use radixmap::{RadixMap, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut map = RadixMap::new();
    ///     map.insert("/api/v1", ())?;
    ///     map.insert("/api/v2", ())?;
    ///
    ///     assert_eq!(map.len(), 2);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.size
    }

    /// Check if the tree has no data nodes
    ///
    /// ```
    /// use radixmap::{RadixMap, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut map = RadixMap::new();
    ///
    ///     assert!(map.is_empty());
    ///
    ///     map.insert("/api/v1", ())?;
    ///     map.insert("/api/v2", ())?;
    ///
    ///     assert!(!map.is_empty());
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Retrieve the corresponding data
    ///
    /// ```
    /// use radixmap::{RadixMap, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut map = RadixMap::new();
    ///     map.insert("/api/v1", 1)?;
    ///     map.insert("/api/v2", 2)?;
    ///
    ///     assert_eq!(map.get("/api/v1"), Some(&1));
    ///     assert_eq!(map.get("/api/v2"), Some(&2));
    ///     assert_eq!(map.get("/api/v3"), None);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn get(&self, path: &str) -> Option<&V> {
        self.values().with_prefix(path, true).next()
    }

    /// Retrieve the corresponding mutable data
    ///
    /// ```
    /// use radixmap::{RadixMap, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut map = RadixMap::new();
    ///     map.insert("/api/v1", 1)?;
    ///     map.insert("/api/v2", 2)?;
    ///
    ///     assert_eq!(map.get_mut("/api/v1"), Some(&mut 1));
    ///     assert_eq!(map.get_mut("/api/v2"), Some(&mut 2));
    ///     assert_eq!(map.get_mut("/api/v3"), None);
    ///
    ///     if let Some(data) = map.get_mut("/api/v1") {
    ///         *data = 3;
    ///     }
    ///
    ///     assert_eq!(map.get_mut("/api/v1"), Some(&mut 3));
    ///     assert_eq!(map.get_mut("/api/v2"), Some(&mut 2));
    ///     assert_eq!(map.get_mut("/api/v3"), None);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn get_mut(&mut self, path: &str) -> Option<&mut V> {
        self.values_mut().with_prefix(path, true).next()
    }

    /// Check if the tree contains specific key
    ///
    /// ```
    /// use radixmap::{RadixMap, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut map = RadixMap::new();
    ///     map.insert("/api/v1", ())?;
    ///     map.insert("/api/v2", ())?;
    ///
    ///     assert_eq!(map.contains_key("/api/v1"), true);
    ///     assert_eq!(map.contains_key("/api/v2"), true);
    ///     assert_eq!(map.contains_key("/api/v3"), false);
    ///     assert_eq!(map.contains_key("/api/v"), false);
    ///     assert_eq!(map.contains_key("/api"), false);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn contains_key(&self, path: &str) -> bool {
        self.iter().with_prefix(path, true).next().is_some()
    }

    /// Check if the tree contains specific value
    ///
    /// ```
    /// use radixmap::{RadixMap, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut map = RadixMap::new();
    ///     map.insert("/api/v1", 1)?;
    ///     map.insert("/api/v2", 2)?;
    ///     map.insert("/api/v3", 1)?;
    ///
    ///     assert_eq!(map.contains_value(&1), true);
    ///     assert_eq!(map.contains_value(&2), true);
    ///     assert_eq!(map.contains_value(&3), false);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn contains_value(&self, data: &V) -> bool where V: PartialEq {
        for value in self.values() {
            if value == data {
                return true;
            }
        }

        false
    }

    /// Iterate over the tree to retrieve nodes' key and value
    ///
    /// ```
    /// use radixmap::{RadixMap, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut map = RadixMap::new();
    ///     map.insert("/api/v1", "v1")?;
    ///     map.insert("/api/v1/user", "user1")?;
    ///     map.insert("/api/v2", "v2")?;
    ///     map.insert("/api/v2/user", "user2")?;
    ///     map.insert("/api", "api")?;
    ///
    ///     let mut iter = map.iter();
    ///
    ///     assert_eq!(iter.next(), Some(("/api", &"api")));
    ///     assert_eq!(iter.next(), Some(("/api/v1", &"v1")));
    ///     assert_eq!(iter.next(), Some(("/api/v1/user", &"user1")));
    ///     assert_eq!(iter.next(), Some(("/api/v2", &"v2")));
    ///     assert_eq!(iter.next(), Some(("/api/v2/user", &"user2")));
    ///     assert_eq!(iter.next(), None);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn iter(&self) -> Iter<'_, V> {
        Iter::from(self)
    }

    /// Iterate over the tree to retrieve nodes' key and mutable value
    ///
    /// ```
    /// use std::iter::Peekable;
    /// use radixmap::{RadixMap, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut map = RadixMap::new();
    ///     map.insert("/api", 0)?;
    ///
    ///     let mut iter = map.iter_mut().peekable();
    ///
    ///     assert_eq!(iter.peek(), Some(&("/api", &mut 0)));
    ///
    ///     match iter.peek_mut() {
    ///         Some(node) => *node.1 = 1,
    ///         None => unreachable!()
    ///     }
    ///
    ///     assert_eq!(iter.next(), Some(("/api", &mut 1)));
    ///     assert_eq!(iter.next(), None);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, 'k, V> {
        IterMut::from(self)
    }

    /// Iterate over the tree to get nodes' key only
    ///
    /// ```
    /// use radixmap::{RadixMap, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut map = RadixMap::new();
    ///     map.insert("/api", ())?;
    ///     map.insert("/api/v1", ())?;
    ///     map.insert("/api/v1/user", ())?;
    ///     map.insert("/api/v2", ())?;
    ///     map.insert("/api/v2/user", ())?;
    ///
    ///     let mut iter = map.keys();
    ///
    ///     assert_eq!(iter.next(), Some("/api"));
    ///     assert_eq!(iter.next(), Some("/api/v1"));
    ///     assert_eq!(iter.next(), Some("/api/v1/user"));
    ///     assert_eq!(iter.next(), Some("/api/v2"));
    ///     assert_eq!(iter.next(), Some("/api/v2/user"));
    ///     assert_eq!(iter.next(), None);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn keys(&self) -> Keys<'_, V> {
        Keys::from(self)
    }

    /// Iterate over the tree to get nodes' value only
    ///
    /// ```
    /// use radixmap::{RadixMap, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut map = RadixMap::new();
    ///     map.insert("/api", "api")?;
    ///     map.insert("/api/v1", "v1")?;
    ///     map.insert("/api/v1/user", "user1")?;
    ///     map.insert("/api/v2", "v2")?;
    ///     map.insert("/api/v2/user", "user2")?;
    ///
    ///     let mut iter = map.values();
    ///
    ///     assert_eq!(iter.next(), Some(&"api"));
    ///     assert_eq!(iter.next(), Some(&"v1"));
    ///     assert_eq!(iter.next(), Some(&"user1"));
    ///     assert_eq!(iter.next(), Some(&"v2"));
    ///     assert_eq!(iter.next(), Some(&"user2"));
    ///     assert_eq!(iter.next(), None);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn values(&self) -> Values<'_, V> {
        Values::from(self)
    }

    /// Iterate over the tree to get nodes' mutable value
    ///
    /// ```
    /// use std::iter::Peekable;
    /// use radixmap::{RadixMap, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut map = RadixMap::new();
    ///     map.insert("/api", 0)?;
    ///
    ///     let mut iter = map.values_mut().peekable();
    ///
    ///     assert_eq!(iter.peek(), Some(&&mut 0));
    ///
    ///     match iter.peek_mut() {
    ///         Some(node) => **node = 1,
    ///         None => unreachable!()
    ///     }
    ///
    ///     assert_eq!(iter.next(), Some(&mut 1));
    ///     assert_eq!(iter.next(), None);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn values_mut(&mut self) -> ValuesMut<'_, 'k, V> {
        ValuesMut::from(self)
    }

    /// Insert into a pair of new data and return old if exist
    ///
    /// ```
    /// use radixmap::{RadixMap, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut map = RadixMap::new();
    ///
    ///     assert_eq!(map.insert("/api/v1", 1)?, None);
    ///     assert_eq!(map.insert("/api/v2", 2)?, None);
    ///     assert_eq!(map.insert("/api/v1", 3)?, Some(1));
    ///     assert_eq!(map.insert("/api/v2", 4)?, Some(2));
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn insert(&mut self, path: &'k str, data: V) -> RadixResult<Option<V>> {
        let ret = self.root.insert(path, data);
        if let Ok(None) = &ret {
            self.size += 1;
        }
        ret
    }

    /// Remove the node of the path
    ///
    /// ```
    /// use radixmap::{RadixMap, RadixResult, rule::RadixRule};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut map = RadixMap::new();
    ///     map.insert("/api/v1", ())?;
    ///     map.insert("/api/v2", ())?;
    ///
    ///     assert_eq!(map.remove("/api").map(|node| node.rule), None);
    ///     assert_eq!(map.remove("/api/v1").map(|node| node.rule), Some(RadixRule::from_plain("/api/v1")?));
    ///     assert_eq!(map.remove("/api/v2").map(|node| node.rule), Some(RadixRule::from_plain("/api/v2")?));
    ///     assert_eq!(map.is_empty(), true);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn remove(&mut self, _path: &str) -> Option<RadixNode<'k, V>> {
        todo!()
    }

    /// Clear the radix map but preserve its capacity
    ///
    /// ```
    /// use radixmap::{RadixMap, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut map = RadixMap::new();
    ///     map.insert("/api/v1", ())?;
    ///     map.insert("/api/v2", ())?;
    ///
    ///     assert_eq!(map.len(), 2);
    ///
    ///     map.clear();
    ///
    ///     assert!(map.is_empty());
    ///     assert_eq!(map.len(), 0);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn clear(&mut self) {
        self.root.clear();
        self.size = 0;
    }
}

// -----------------------------------------------------------------------------

/// Construct from an array of tuples
///
/// ```
/// use radixmap::{RadixMap, RadixResult};
///
/// fn main() -> RadixResult<()> {
///     let map = RadixMap::try_from([("/api/v1", 1), ("/api/v2", 2)])?;
///
///     assert_eq!(map.len(), 2);
///     assert_eq!(map.get("/api/v1"), Some(&1));
///     assert_eq!(map.get("/api/v2"), Some(&2));
///
///     Ok(())
/// }
/// ```
impl<'k, V, const N: usize> TryFrom<[(&'k str, V); N]> for RadixMap<'k, V> {
    type Error = RadixError;

    fn try_from(value: [(&'k str, V); N]) -> Result<Self, Self::Error> {
        let mut map = RadixMap::default();

        for (path, data) in value {
            map.insert(path, data)?;
        }

        Ok(map)
    }
}

/// Default trait
impl<'k, V> Default for RadixMap<'k, V> {
    fn default() -> Self {
        Self { root: RadixNode::default(), size: 0 }
    }
}

/// Debug trait
impl<'k, V: Debug> Debug for RadixMap<'k, V> {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

/// Clone trait
///
/// ```
/// use radixmap::{RadixMap, RadixResult};
///
/// fn main() -> RadixResult<()> {
///     let map_a = RadixMap::try_from([("/api/v1", 1), ("/api/v2", 2)])?;
///     let map_b = map_a.clone();
///
///     assert_eq!(map_a, map_b);
///
///     Ok(())
/// }
/// ```
impl<'k, V: Clone> Clone for RadixMap<'k, V> {
    fn clone(&self) -> Self {
        Self { root: self.root.clone(), size: self.size }
    }
}

/// == & !=
impl<'k, V: Eq> Eq for RadixMap<'k, V> {}

/// == & !=
impl<'k, V: PartialEq> PartialEq for RadixMap<'k, V> {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        todo!()
    }
}

// -----------------------------------------------------------------------------

/// Re-import Order
pub type Order = node::Order;

// -----------------------------------------------------------------------------

/// Iterator for map
#[derive(Default, Clone)]
pub struct Iter<'k, V> {
    iter: node::Iter<'k, V>
}

impl<'k, V> Iter<'k, V> {
    /// Starting to iterate from the node with a specific prefix
    pub fn with_prefix(mut self, path: &str, data: bool) -> Self {
        self.iter = self.iter.with_prefix(path, data);
        self
    }

    /// Change the iterating order
    pub fn with_order(mut self, order: Order) -> Self {
        self.iter = self.iter.with_order(order);
        self
    }
}

impl<'k, V> From<&'k RadixMap<'k, V>> for Iter<'k, V> {
    fn from(value: &'k RadixMap<'k, V>) -> Self {
        Self { iter: node::Iter::from(&value.root) }
    }
}

impl<'k, V> Iterator for Iter<'k, V> {
    type Item = (&'k str, &'k V);

    fn next(&mut self) -> Option<Self::Item> {
        for node in self.iter.by_ref() {
            if node.data.as_ref().is_some() {
                return node.item_ref();
            }
        }

        None
    }
}

// -----------------------------------------------------------------------------

/// Mutable iterator for map
#[derive(Default)]
pub struct IterMut<'n, 'k, V> {
    iter: node::IterMut<'n, 'k, V>
}

impl<'n, 'k, V> IterMut<'n, 'k, V> {
    /// Starting to iterate from the node with a specific prefix
    pub fn with_prefix(mut self, path: &str, data: bool) -> Self {
        self.iter = self.iter.with_prefix(path, data);
        self
    }

    /// Change the iterating order
    pub fn with_order(mut self, order: Order) -> Self {
        self.iter = self.iter.with_order(order);
        self
    }
}

impl<'n, 'k, V> From<&'n mut RadixMap<'k, V>> for IterMut<'n, 'k, V> {
    fn from(value: &'n mut RadixMap<'k, V>) -> Self {
        Self { iter: node::IterMut::from(&mut value.root) }
    }
}

impl<'n, 'k, V> Iterator for IterMut<'n, 'k, V> {
    type Item = (&'n str, &'n mut V);

    fn next(&mut self) -> Option<Self::Item> {
        for node in self.iter.by_ref() {
            if node.data.as_ref().is_some() {
                return node.item_mut();
            }
        }

        None
    }
}

// -----------------------------------------------------------------------------

/// Path adapter
#[derive(Clone)]
pub struct Keys<'k, V> {
    iter: Iter<'k, V>
}

impl<'k, V> Keys<'k, V> {
    /// Starting to iterate from the node with a specific prefix
    pub fn with_prefix(mut self, path: &str, data: bool) -> Self {
        self.iter = self.iter.with_prefix(path, data);
        self
    }

    /// Change the iterating order
    pub fn with_order(mut self, order: Order) -> Self {
        self.iter = self.iter.with_order(order);
        self
    }
}

impl<'k, V> From<&'k RadixMap<'k, V>> for Keys<'k, V> {
    fn from(value: &'k RadixMap<'k, V>) -> Self {
        Self { iter: Iter::from(value) }
    }
}

impl<'k, V> Iterator for Keys<'k, V> {
    type Item = &'k str;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|item| item.0)
    }
}

// -----------------------------------------------------------------------------

/// Data adapter
#[derive(Clone)]
pub struct Values<'k, V> {
    iter: Iter<'k, V>
}

impl<'k, V> Values<'k, V> {
    /// Starting to iterate from the node with a specific prefix
    pub fn with_prefix(mut self, path: &str, data: bool) -> Self {
        self.iter = self.iter.with_prefix(path, data);
        self
    }

    /// Change the iterating order
    pub fn with_order(mut self, order: Order) -> Self {
        self.iter = self.iter.with_order(order);
        self
    }
}

impl<'k, V> From<&'k RadixMap<'k, V>> for Values<'k, V> {
    fn from(value: &'k RadixMap<'k, V>) -> Self {
        Self { iter: Iter::from(value) }
    }
}

impl<'k, V> Iterator for Values<'k, V> {
    type Item = &'k V;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|item| item.1)
    }
}

// -----------------------------------------------------------------------------

/// Mutable data adapter
pub struct ValuesMut<'n, 'k, V> {
    iter: IterMut<'n, 'k, V>
}

impl<'n, 'k, V> ValuesMut<'n, 'k, V> {
    /// Starting to iterate from the node with a specific prefix
    pub fn with_prefix(mut self, path: &str, data: bool) -> Self {
        self.iter = self.iter.with_prefix(path, data);
        self
    }

    /// Change the iterating order
    pub fn with_order(mut self, order: Order) -> Self {
        self.iter = self.iter.with_order(order);
        self
    }
}

impl<'n, 'k, V> From<&'n mut RadixMap<'k, V>> for ValuesMut<'n, 'k, V> {
    fn from(value: &'n mut RadixMap<'k, V>) -> Self {
        Self { iter: IterMut::from(value) }
    }
}

impl<'n, 'k, V> Iterator for ValuesMut<'n, 'k, V> {
    type Item = &'n mut V;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|item| item.1)
    }
}