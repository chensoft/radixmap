//! Radix map implementation
use super::defs::*;
use super::node;
use super::node::RadixNode;

/// The radix map where the key is &'a str and the value is arbitrary data
pub struct RadixMap<'a, V> {
    /// The root node, always empty
    root: RadixNode<'a, V>,

    /// The number of leaf nodes
    size: usize,
}

impl<'a, V> RadixMap<'a, V> {
    /// For consistency with the standard library, we provide this fn to create an empty map
    pub fn new() -> Self {
        Default::default()
    }

    /// The leaf nodes' count, note that this excludes the internal nodes
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

    /// Check if the tree contains only a root node
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
        self.root.is_leaf()
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
        match self.root.values().with_prefix(path) {
            Ok(mut iter) => iter.next(),
            Err(_) => None
        }
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
    ///     assert_eq!(map.get_mut("/api/v1"), Some(&1));
    ///     assert_eq!(map.get_mut("/api/v2"), Some(&2));
    ///     assert_eq!(map.get_mut("/api/v3"), None);
    ///
    ///     if let Some(data) = map.get_mut("/api/v1") {
    ///         *data = 3;
    ///     }
    ///
    ///     assert_eq!(map.get_mut("/api/v1"), Some(&3));
    ///     assert_eq!(map.get_mut("/api/v2"), Some(&2));
    ///     assert_eq!(map.get_mut("/api/v3"), None);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn get_mut(&'a mut self, path: &str) -> Option<&mut V> {
        match self.root.values_mut().with_prefix(path) {
            Ok(mut iter) => iter.next(),
            Err(_) => None
        }
    }

    /// Check if the tree contains specific key
    ///
    /// ```
    /// use radixmap::{RadixMap, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut map = RadixMap::new();
    ///     map.insert("/api/v1", 1)?;
    ///     map.insert("/api/v2", 2)?;
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
        self.root.values().with_prefix(path).is_ok()
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
    ///     map.insert("/api", 0)?;
    ///     map.insert("/api/v1", 1)?;
    ///     map.insert("/api/v1/user1", 11)?;
    ///     map.insert("/api/v2", 2)?;
    ///     map.insert("/api/v2/user2", 22)?;
    ///
    ///     let mut iter = map.iter();
    ///
    ///     assert_eq!(iter.next(), Some(("/api", &0)));
    ///     assert_eq!(iter.next(), Some(("/api/v1", &1)));
    ///     assert_eq!(iter.next(), Some(("/api/v1/user1", &11)));
    ///     assert_eq!(iter.next(), Some(("/api/v2", &2)));
    ///     assert_eq!(iter.next(), Some(("/api/v2/user2", &22)));
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
    /// use radixmap::{RadixMap, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut map = RadixMap::new();
    ///     map.insert("/api", 0)?;
    ///
    ///     let mut iter = map.iter_mut();
    ///
    ///     assert_eq!(iter.peek(), Some(("/api", &0)));
    ///
    ///     let mut next = iter.peek().unwrap();
    ///     *next.1 = 1;
    ///
    ///     assert_eq!(iter.next(), Some(("/api", &1)));
    ///     assert_eq!(iter.next(), None);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn iter_mut(&'a mut self) -> IterMut<'_, V> {
        IterMut::from(self)
    }

    /// Iterate over the tree to get nodes' key only
    ///
    /// ```
    /// use radixmap::{RadixMap, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut map = RadixMap::new();
    ///     map.insert("/api", 0)?;
    ///     map.insert("/api/v1", 1)?;
    ///     map.insert("/api/v1/user1", 11)?;
    ///     map.insert("/api/v2", 2)?;
    ///     map.insert("/api/v2/user2", 22)?;
    ///
    ///     let mut iter = map.keys();
    ///
    ///     assert_eq!(iter.next(), Some("/api"));
    ///     assert_eq!(iter.next(), Some("/api/v1"));
    ///     assert_eq!(iter.next(), Some("/api/v1/user1"));
    ///     assert_eq!(iter.next(), Some("/api/v2"));
    ///     assert_eq!(iter.next(), Some("/api/v2/user2"));
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
    ///     map.insert("/api", 0)?;
    ///     map.insert("/api/v1", 1)?;
    ///     map.insert("/api/v1/user1", 11)?;
    ///     map.insert("/api/v2", 2)?;
    ///     map.insert("/api/v2/user2", 22)?;
    ///
    ///     let mut iter = map.values();
    ///
    ///     assert_eq!(iter.next(), Some(&0));
    ///     assert_eq!(iter.next(), Some(&1));
    ///     assert_eq!(iter.next(), Some(&11));
    ///     assert_eq!(iter.next(), Some(&2));
    ///     assert_eq!(iter.next(), Some(&22));
    ///     assert_eq!(iter.next(), None);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn values(&self) -> Values<V> {
        Values::from(self)
    }

    /// Iterate over the tree to get nodes' mutable value
    ///
    /// ```
    /// use radixmap::{RadixMap, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut map = RadixMap::new();
    ///     map.insert("/api", 0)?;
    ///
    ///     let mut iter = map.values_mut();
    ///
    ///     assert_eq!(iter.peek(), Some(("/api", &0)));
    ///
    ///     let mut next = iter.peek().unwrap();
    ///     *next = 1;
    ///
    ///     assert_eq!(iter.next(), Some(("/api", &1)));
    ///     assert_eq!(iter.next(), None);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn values_mut(&'a mut self) -> ValuesMut<V> {
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
    pub fn insert(&mut self, path: &'a str, data: V) -> RadixResult<Option<V>> {
        let ret = self.root.insert(path, data);
        if let Ok(None) = &ret {
            self.size += 1;
        }
        ret
    }

    /// Remove the node of the path
    ///
    /// ```
    /// use radixmap::{RadixMap, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut map = RadixMap::new();
    ///     map.insert("/api/v1", 1)?;
    ///     map.insert("/api/v2", 2)?;
    ///
    ///     assert_eq!(map.remove("/api"), None);
    ///     assert_eq!(map.remove("/api/v1").unwrap().rule, "/api/v1");
    ///     assert_eq!(map.remove("/api/v2").unwrap().rule, "/api/v2");
    ///     assert_eq!(map.is_empty(), true);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn remove(&mut self, _path: &str) -> Option<RadixNode<'a, V>> {
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
impl<'a, V, const N: usize> TryFrom<[(&'a str, V); N]> for RadixMap<'a, V> {
    type Error = RadixError;

    fn try_from(value: [(&'a str, V); N]) -> Result<Self, Self::Error> {
        let mut map = RadixMap::default();

        for (path, data) in value {
            map.insert(path, data)?;
        }

        Ok(map)
    }
}

/// Default trait
impl<'a, V> Default for RadixMap<'a, V> {
    fn default() -> Self {
        Self { root: RadixNode::default(), size: 0 }
    }
}

/// Debug trait
impl<'a, V: Debug> Debug for RadixMap<'a, V> {
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
impl<'a, V: Clone> Clone for RadixMap<'a, V> {
    fn clone(&self) -> Self {
        Self { root: self.root.clone(), size: self.size }
    }
}

/// == & !=
impl<'a, V: Eq> Eq for RadixMap<'a, V> {}

/// == & !=
impl<'a, V: PartialEq> PartialEq for RadixMap<'a, V> {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        todo!()
    }
}

// -----------------------------------------------------------------------------

pub type Order = node::Order;

// -----------------------------------------------------------------------------

#[derive(Default, Clone)]
pub struct Iter<'a, V> {
    iter: node::Iter<'a, V>
}

impl<'a, V> Iter<'a, V> {
    /// Starting to iterate from the node with a specific prefix
    pub fn with_prefix(mut self, prefix: &str) -> RadixResult<Self> {
        self.iter = self.iter.with_prefix(prefix)?;
        Ok(self)
    }

    /// Change the iterating order
    pub fn with_order(mut self, order: Order) -> Self {
        self.iter = self.iter.with_order(order);
        self
    }
}

impl<'a, V> From<&'a RadixMap<'a, V>> for Iter<'a, V> {
    fn from(value: &'a RadixMap<'a, V>) -> Self {
        Self { iter: node::Iter::from(&value.root) }
    }
}

impl<'a, V> Iterator for Iter<'a, V> {
    type Item = (&'a str, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        for node in self.iter.by_ref() {
            if let Some(data) = node.data.as_ref() {
                return Some((node.path, data));
            }
        }
        
        None
    }
}

// -----------------------------------------------------------------------------

#[derive(Default)]
pub struct IterMut<'a, V> {
    iter: node::IterMut<'a, V>
}

impl<'a, V> IterMut<'a, V> {
    /// Starting to iterate from the node with a specific prefix
    pub fn with_prefix(mut self, prefix: &str) -> RadixResult<Self> {
        self.iter = self.iter.with_prefix(prefix)?;
        Ok(self)
    }

    /// Change the iterating order
    pub fn with_order(mut self, order: Order) -> Self {
        self.iter = self.iter.with_order(order);
        self
    }
}

impl<'a, V> From<&'a mut RadixMap<'a, V>> for IterMut<'a, V> {
    fn from(value: &'a mut RadixMap<'a, V>) -> Self {
        Self { iter: node::IterMut::from(&mut value.root) }
    }
}

impl<'a, V> Iterator for IterMut<'a, V> {
    type Item = (&'a str, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        for node in self.iter.by_ref() {
            if let Some(data) = node.data.as_mut() {
                return Some((node.path, data));
            }
        }

        None
    }
}

// -----------------------------------------------------------------------------

#[derive(Clone)]
pub struct Keys<'a, V> {
    iter: Iter<'a, V>
}

impl<'a, V> Keys<'a, V> {
    /// Starting to iterate from the node with a specific prefix
    pub fn with_prefix(mut self, prefix: &str) -> RadixResult<Self> {
        self.iter = self.iter.with_prefix(prefix)?;
        Ok(self)
    }

    /// Change the iterating order
    pub fn with_order(mut self, order: Order) -> Self {
        self.iter = self.iter.with_order(order);
        self
    }
}

impl<'a, V> From<&'a RadixMap<'a, V>> for Keys<'a, V> {
    fn from(value: &'a RadixMap<'a, V>) -> Self {
        Self { iter: Iter::from(value) }
    }
}

impl<'a, V> Iterator for Keys<'a, V> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|item| item.0)
    }
}

// -----------------------------------------------------------------------------

#[derive(Clone)]
pub struct Values<'a, V> {
    iter: Iter<'a, V>
}

impl<'a, V> Values<'a, V> {
    /// Starting to iterate from the node with a specific prefix
    pub fn with_prefix(mut self, prefix: &str) -> RadixResult<Self> {
        self.iter = self.iter.with_prefix(prefix)?;
        Ok(self)
    }

    /// Change the iterating order
    pub fn with_order(mut self, order: Order) -> Self {
        self.iter = self.iter.with_order(order);
        self
    }
}

impl<'a, V> From<&'a RadixMap<'a, V>> for Values<'a, V> {
    fn from(value: &'a RadixMap<'a, V>) -> Self {
        Self { iter: Iter::from(value) }
    }
}

impl<'a, V> Iterator for Values<'a, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|item| item.1)
    }
}

// -----------------------------------------------------------------------------

pub struct ValuesMut<'a, V> {
    iter: IterMut<'a, V>
}

impl<'a, V> ValuesMut<'a, V> {
    /// Starting to iterate from the node with a specific prefix
    pub fn with_prefix(mut self, prefix: &str) -> RadixResult<Self> {
        self.iter = self.iter.with_prefix(prefix)?;
        Ok(self)
    }

    /// Change the iterating order
    pub fn with_order(mut self, order: Order) -> Self {
        self.iter = self.iter.with_order(order);
        self
    }
}

impl<'a, V> From<&'a mut RadixMap<'a, V>> for ValuesMut<'a, V> {
    fn from(value: &'a mut RadixMap<'a, V>) -> Self {
        Self { iter: IterMut::from(value) }
    }
}

impl<'a, V> Iterator for ValuesMut<'a, V> {
    type Item = &'a mut V;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|item| item.1)
    }
}