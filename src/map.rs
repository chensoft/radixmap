//! Radix map implementation
use super::defs::*;
use super::node::{self, RadixNode};

/// The radix map where the key is &'k str and the value is arbitrary data
pub struct RadixMap<'k, V> {
    /// The root node, always empty
    root: RadixNode<'k, V>,

    /// The number of data nodes
    size: usize,
}

impl<'k, V> RadixMap<'k, V> {
    /// For consistency with the standard library, we provide this fn to create an empty map
    #[inline]
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
    ///     assert_eq!(map.is_empty(), true);
    ///
    ///     map.insert("/api/v1", ())?;
    ///     map.insert("/api/v2", ())?;
    ///
    ///     assert_eq!(map.is_empty(), false);
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
        self.root.search(path, true).and_then(|node| node.data.as_ref())
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
        self.root.search_mut(path, true).and_then(|node| node.data.as_mut())
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
        self.root.search(path, true).map_or(false, |node| !node.is_empty())
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
    #[inline]
    pub fn insert(&mut self, path: &'k str, data: V) -> RadixResult<Option<V>> {
        let ret = self.root.insert(path, data);
        if let Ok(None) = &ret {
            self.size += 1;
        }
        ret
    }

    /// Remove the nodes along the path, affecting data nodes only
    ///
    /// ```
    /// use radixmap::{RadixMap, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut map = RadixMap::new();
    ///     map.insert("/api/v1", "v1")?;
    ///     map.insert("/api/v2", "v2")?;
    ///     map.insert("/api", "api")?;
    ///
    ///     assert_eq!(map.len(), 3);
    ///     assert_eq!(map.remove("/"), None);                          // non-data node
    ///     assert_eq!(map.remove("/api"), Some(("/api", "api")));      // len - 1
    ///     assert_eq!(map.remove("/api/v2"), Some(("/api/v2", "v2"))); // len - 1
    ///     assert_eq!(map.len(), 1);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn remove(&mut self, path: &str) -> Option<(&'k str, V)> {
        let node = self.root.search_mut(path, true)?;
        let path = std::mem::take(&mut node.path);
        let data = std::mem::take(&mut node.data);

        self.size -= 1;

        Some((path, data?))
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
    ///     assert_eq!(map.is_empty(), true);
    ///     assert_eq!(map.len(), 0);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
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
///     assert_eq!(map.get("/api/v3"), None);
///
///     Ok(())
/// }
/// ```
impl<'k, V, const N: usize> TryFrom<[(&'k str, V); N]> for RadixMap<'k, V> {
    type Error = RadixError;

    #[inline]
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
    #[inline]
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
    #[inline]
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

        let mut iter_a = self.iter();
        let mut iter_b = other.iter();

        for _ in 0..self.len() {
            let item_a = iter_a.next();
            let item_b = iter_b.next();

            if item_a != item_b {
                return false;
            }
        }

        true
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
    #[inline]
    pub fn with_prefix(mut self, path: &str, data: bool) -> Self {
        self.iter = self.iter.with_prefix(path, data);
        self
    }

    /// Change the iterating order
    #[inline]
    pub fn with_order(mut self, order: Order) -> Self {
        self.iter = self.iter.with_order(order);
        self
    }
}

impl<'k, V> From<&'k RadixMap<'k, V>> for Iter<'k, V> {
    #[inline]
    fn from(value: &'k RadixMap<'k, V>) -> Self {
        Self { iter: node::Iter::from(&value.root) }
    }
}

impl<'k, V> Iterator for Iter<'k, V> {
    type Item = (&'k str, &'k V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().and_then(|node| node.item_ref())
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
    #[inline]
    pub fn with_prefix(mut self, path: &str, data: bool) -> Self {
        self.iter = self.iter.with_prefix(path, data);
        self
    }

    /// Change the iterating order
    #[inline]
    pub fn with_order(mut self, order: Order) -> Self {
        self.iter = self.iter.with_order(order);
        self
    }
}

impl<'n, 'k, V> From<&'n mut RadixMap<'k, V>> for IterMut<'n, 'k, V> {
    #[inline]
    fn from(value: &'n mut RadixMap<'k, V>) -> Self {
        Self { iter: node::IterMut::from(&mut value.root) }
    }
}

impl<'n, 'k, V> Iterator for IterMut<'n, 'k, V> {
    type Item = (&'n str, &'n mut V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().and_then(|node| node.item_mut())
    }
}

// -----------------------------------------------------------------------------

/// Path adapter
#[derive(Default, Clone)]
pub struct Keys<'k, V> {
    iter: Iter<'k, V>
}

impl<'k, V> Keys<'k, V> {
    /// Starting to iterate from the node with a specific prefix
    #[inline]
    pub fn with_prefix(mut self, path: &str, data: bool) -> Self {
        self.iter = self.iter.with_prefix(path, data);
        self
    }

    /// Change the iterating order
    #[inline]
    pub fn with_order(mut self, order: Order) -> Self {
        self.iter = self.iter.with_order(order);
        self
    }
}

impl<'k, V> From<&'k RadixMap<'k, V>> for Keys<'k, V> {
    #[inline]
    fn from(value: &'k RadixMap<'k, V>) -> Self {
        Self { iter: Iter::from(value) }
    }
}

impl<'k, V> Iterator for Keys<'k, V> {
    type Item = &'k str;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|item| item.0)
    }
}

// -----------------------------------------------------------------------------

/// Data adapter
#[derive(Default, Clone)]
pub struct Values<'k, V> {
    iter: Iter<'k, V>
}

impl<'k, V> Values<'k, V> {
    /// Starting to iterate from the node with a specific prefix
    #[inline]
    pub fn with_prefix(mut self, path: &str, data: bool) -> Self {
        self.iter = self.iter.with_prefix(path, data);
        self
    }

    /// Change the iterating order
    #[inline]
    pub fn with_order(mut self, order: Order) -> Self {
        self.iter = self.iter.with_order(order);
        self
    }
}

impl<'k, V> From<&'k RadixMap<'k, V>> for Values<'k, V> {
    #[inline]
    fn from(value: &'k RadixMap<'k, V>) -> Self {
        Self { iter: Iter::from(value) }
    }
}

impl<'k, V> Iterator for Values<'k, V> {
    type Item = &'k V;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|item| item.1)
    }
}

// -----------------------------------------------------------------------------

/// Mutable data adapter
#[derive(Default)]
pub struct ValuesMut<'n, 'k, V> {
    iter: IterMut<'n, 'k, V>
}

impl<'n, 'k, V> ValuesMut<'n, 'k, V> {
    /// Starting to iterate from the node with a specific prefix
    #[inline]
    pub fn with_prefix(mut self, path: &str, data: bool) -> Self {
        self.iter = self.iter.with_prefix(path, data);
        self
    }

    /// Change the iterating order
    #[inline]
    pub fn with_order(mut self, order: Order) -> Self {
        self.iter = self.iter.with_order(order);
        self
    }
}

impl<'n, 'k, V> From<&'n mut RadixMap<'k, V>> for ValuesMut<'n, 'k, V> {
    #[inline]
    fn from(value: &'n mut RadixMap<'k, V>) -> Self {
        Self { iter: IterMut::from(value) }
    }
}

impl<'n, 'k, V> Iterator for ValuesMut<'n, 'k, V> {
    type Item = &'n mut V;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|item| item.1)
    }
}