use super::def::*;
use super::iter::*;
use super::node::*;

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
    pub fn get(&self, _path: &str) -> Option<&V> {
        // self.root.deepest(path).filter(|node| node.path == path).and_then(|node| node.data.as_ref())
        todo!()
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
    pub fn get_mut(&mut self, _path: &str) -> Option<&mut V> {
        // self.root.deepest_mut(path).filter(|node| node.path == path).and_then(|node| node.data.as_mut())
        todo!()
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
    pub fn contains_key(&self, _path: &str) -> bool {
        todo!()
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
    pub fn iter(&self) -> Iter<'a, V> {
        todo!()
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
    pub fn iter_mut(&self) -> Iter<'a, V> {
        todo!()
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
    pub fn keys(&'a self) -> Keys<'a, V> {
        Keys::from(&self.root)
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
        Values::from(&self.root)
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
    pub fn values_mut(&mut self) -> Values<V> {
        todo!()
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

/// []
///
/// ```
/// use radixmap::{RadixMap, RadixResult};
///
/// fn main() -> RadixResult<()> {
///     let map = RadixMap::try_from([("/api/v1", 1), ("/api/v2", 2)])?;
///
///     assert_eq!(map["/api/v1"], &1);
///
///     Ok(())
/// }
/// ```
impl<'a, V> Index<&str> for RadixMap<'a, V> {
    type Output = V;

    fn index(&self, path: &str) -> &Self::Output {
        match self.get(path) {
            Some(data) => data,
            None => panic!("no entry found for path '{}'", path)
        }
    }
}

/// Mutable []
///
/// ```
/// use radixmap::{RadixMap, RadixResult};
///
/// fn main() -> RadixResult<()> {
///     let map = RadixMap::try_from([("/api/v1", 1), ("/api/v2", 2)])?;
///
///     assert_eq!(map["/api/v1"], &1);
///
///     map["/api/v1"] = 2;
///
///     assert_eq!(map["/api/v1"], &2);
///
///     Ok(())
/// }
/// ```
impl<'a, V> IndexMut<&str> for RadixMap<'a, V> {
    fn index_mut(&mut self, _path: &str) -> &mut Self::Output {
        todo!()
    }
}