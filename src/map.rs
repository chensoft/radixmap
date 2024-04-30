//! Radix map implementation
use super::defs::*;
use super::node::{self, RadixNode};

/// The radix map where the key is Bytes and the value is arbitrary data
pub struct RadixMap<V> {
    /// The root node, always empty
    root: RadixNode<V>,

    /// The number of data nodes
    size: usize,
}

impl<V> RadixMap<V> {
    /// For consistency with the standard library, we provide this fn to create an empty map
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// The data nodes' count, note that RadixMap ignores empty nodes
    /// 
    /// # Examples
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
    /// # Examples
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
    /// # Examples
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
        self.root.lookup(path, true, &mut None).and_then(|node| node.data.as_ref())
    }

    /// Retrieve the corresponding mutable data
    /// 
    /// # Examples
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
        self.root.lookup_mut(path, true, &mut None).and_then(|node| node.data.as_mut())
    }

    /// Retrieve the corresponding data and collect named captures
    /// 
    /// # Examples
    ///
    /// ```
    /// use radixmap::{RadixMap, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut map = RadixMap::new();
    ///     map.insert("/api/v1/user/12345", "user1")?;
    ///     map.insert("/api/v2/user/:id", "user2")?;
    ///     map.insert("/api/v3/user/{id:[0-9]+}", "user3")?;
    ///     map.insert("/api/v4/user/{id:[^0-9]+}", "user4")?;
    ///     map.insert("/api/v5/user/*345", "user5")?;
    ///     map.insert("/blog/:date/{author:[^/]+}/*.html", "blog")?;
    ///     map.insert("/blog/:date/{author:[^/]+}/:date/*.html", "blog")?;
    ///
    ///     assert_eq!(map.capture("/api/v1/user/12345"), (Some(&"user1"), vec![]));
    ///     assert_eq!(map.capture("/api/v2/user/12345"), (Some(&"user2"), vec![("id".to_string(), "12345")]));
    ///     assert_eq!(map.capture("/api/v3/user/12345"), (Some(&"user3"), vec![("id".to_string(), "12345")]));
    ///     assert_eq!(map.capture("/api/v4/user/12345"), (None, vec![]));
    ///     assert_eq!(map.capture("/api/v5/user/12345"), (Some(&"user5"), vec![("*".to_string(), "12345")]));
    ///     assert_eq!(map.capture("/api/v6"), (None, vec![]));
    ///     assert_eq!(map.capture("/blog/2024-04-10/chensoft/index.php"), (None, vec![("date".to_string(), "2024-04-10"), ("author".to_string(), "chensoft")]));
    ///     assert_eq!(map.capture("/blog/2024-04-10/chensoft/index.html"), (Some(&"blog"), vec![("date".to_string(), "2024-04-10"), ("author".to_string(), "chensoft")]));
    ///     assert_eq!(map.capture("/blog/2024-04-10/chensoft/2024-05-01/index.html"), (Some(&"blog"), vec![("date".to_string(), "2024-04-10"), ("author".to_string(), "chensoft"), ("date".to_string(), "2024-05-01")]));
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn capture<'u>(&self, path: &'u str) -> (Option<&V>, Vec<(String, &'u str)>) {
        let mut capt = vec![];
        (self.root.lookup(path, true, &mut Some(&mut capt)).and_then(|node| node.data.as_ref()), capt)
    }

    /// Retrieve the corresponding mutable data and collect named captures
    /// 
    /// # Examples
    ///
    /// ```
    /// use radixmap::{RadixMap, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut map = RadixMap::new();
    ///     map.insert("/api/v1/user/12345", "user1")?;
    ///     map.insert("/api/v2/user/:id", "user2")?;
    ///     map.insert("/api/v3/user/{id:[0-9]+}", "user3")?;
    ///     map.insert("/api/v4/user/{id:[^0-9]+}", "user4")?;
    ///     map.insert("/api/v5/user/*345", "user5")?;
    ///     map.insert("/blog/:date/{author:[^/]+}/*.html", "blog")?;
    ///     map.insert("/blog/:date/{author:[^/]+}/:date/*.html", "blog")?;
    ///
    ///     assert_eq!(map.capture_mut("/api/v1/user/12345"), (Some(&mut "user1"), vec![]));
    ///     assert_eq!(map.capture_mut("/api/v2/user/12345"), (Some(&mut "user2"), vec![("id".to_string(), "12345")]));
    ///     assert_eq!(map.capture_mut("/api/v3/user/12345"), (Some(&mut "user3"), vec![("id".to_string(), "12345")]));
    ///     assert_eq!(map.capture_mut("/api/v4/user/12345"), (None, vec![]));
    ///     assert_eq!(map.capture_mut("/api/v5/user/12345"), (Some(&mut "user5"), vec![("*".to_string(), "12345")]));
    ///     assert_eq!(map.capture_mut("/api/v6"), (None, vec![]));
    ///     assert_eq!(map.capture_mut("/blog/2024-04-10/chensoft/index.php"), (None, vec![("date".to_string(), "2024-04-10"), ("author".to_string(), "chensoft")]));
    ///     assert_eq!(map.capture_mut("/blog/2024-04-10/chensoft/index.html"), (Some(&mut "blog"), vec![("date".to_string(), "2024-04-10"), ("author".to_string(), "chensoft")]));
    ///     assert_eq!(map.capture_mut("/blog/2024-04-10/chensoft/2024-05-01/index.html"), (Some(&mut "blog"), vec![("date".to_string(), "2024-04-10"), ("author".to_string(), "chensoft"), ("date".to_string(), "2024-05-01")]));
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn capture_mut<'u>(&mut self, path: &'u str) -> (Option<&mut V>, Vec<(String, &'u str)>) {
        let mut capt = vec![];
        (self.root.lookup_mut(path, true, &mut Some(&mut capt)).and_then(|node| node.data.as_mut()), capt)
    }

    /// Check if the tree contains specific path
    /// 
    /// # Examples
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
        self.root.lookup(path, true, &mut None).map_or(false, |node| !node.is_empty())
    }

    /// Check if the tree contains specific data
    /// 
    /// # Examples
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
        self.values().any(|value| value == data)
    }

    /// Iterate over the tree to retrieve nodes' path and data
    /// 
    /// # Examples
    ///
    /// ```
    /// use bytes::Bytes;
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
    ///     assert_eq!(iter.next(), Some((&Bytes::from("/api"), &"api")));
    ///     assert_eq!(iter.next(), Some((&Bytes::from("/api/v1"), &"v1")));
    ///     assert_eq!(iter.next(), Some((&Bytes::from("/api/v1/user"), &"user1")));
    ///     assert_eq!(iter.next(), Some((&Bytes::from("/api/v2"), &"v2")));
    ///     assert_eq!(iter.next(), Some((&Bytes::from("/api/v2/user"), &"user2")));
    ///     assert_eq!(iter.next(), None);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn iter(&self) -> Iter<V> {
        Iter::from(self)
    }

    /// Iterate over the tree to retrieve nodes' path and mutable data
    /// 
    /// # Examples
    ///
    /// ```
    /// use bytes::Bytes;
    /// use std::iter::Peekable;
    /// use radixmap::{RadixMap, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut map = RadixMap::new();
    ///     map.insert("/api", 0)?;
    ///
    ///     let mut iter = map.iter_mut().peekable();
    ///
    ///     assert_eq!(iter.peek(), Some(&(&Bytes::from("/api"), &mut 0)));
    ///
    ///     match iter.peek_mut() {
    ///         Some(node) => *node.1 = 1,
    ///         None => unreachable!()
    ///     }
    ///
    ///     assert_eq!(iter.next(), Some((&Bytes::from("/api"), &mut 1)));
    ///     assert_eq!(iter.next(), None);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<V> {
        IterMut::from(self)
    }

    /// Iterate over the tree to get nodes' path only
    /// 
    /// # Examples
    ///
    /// ```
    /// use bytes::Bytes;
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
    ///     assert_eq!(iter.next(), Some(&Bytes::from("/api")));
    ///     assert_eq!(iter.next(), Some(&Bytes::from("/api/v1")));
    ///     assert_eq!(iter.next(), Some(&Bytes::from("/api/v1/user")));
    ///     assert_eq!(iter.next(), Some(&Bytes::from("/api/v2")));
    ///     assert_eq!(iter.next(), Some(&Bytes::from("/api/v2/user")));
    ///     assert_eq!(iter.next(), None);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn keys(&self) -> Keys<V> {
        Keys::from(self)
    }

    /// Iterate over the tree to get nodes' data only
    /// 
    /// # Examples
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
    pub fn values(&self) -> Values<V> {
        Values::from(self)
    }

    /// Iterate over the tree to get nodes' mutable data
    /// 
    /// # Examples
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
    pub fn values_mut(&mut self) -> ValuesMut<V> {
        ValuesMut::from(self)
    }

    /// Insert into a pair of new data and return old if exist
    /// 
    /// # Examples
    ///
    /// ```
    /// use radixmap::{RadixMap, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut map = RadixMap::new();
    ///
    ///     assert_eq!(map.insert("12345678901234567890", 0)?, None);
    ///     assert_eq!(map.insert("12345678901234567890", 0)?, Some(0));
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
    pub fn insert(&mut self, path: impl Into<Bytes>, data: V) -> RadixResult<Option<V>> {
        let ret = self.root.insert(path.into(), data);
        if let Ok(None) = &ret {
            self.size += 1;
        }
        ret
    }

    /// Remove the nodes along the path, affecting data nodes only
    /// 
    /// # Examples
    ///
    /// ```
    /// use bytes::Bytes;
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
    ///     assert_eq!(map.remove("/api"), Some((Bytes::from("/api"), "api")));      // len - 1
    ///     assert_eq!(map.remove("/api/v2"), Some((Bytes::from("/api/v2"), "v2"))); // len - 1
    ///     assert_eq!(map.len(), 1);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn remove(&mut self, path: &str) -> Option<(Bytes, V)> {
        let node = self.root.lookup_mut(path, true, &mut None)?;
        let path = std::mem::take(&mut node.path);
        let data = std::mem::take(&mut node.data);

        self.size -= 1;

        Some((path, data?))
    }

    /// Clear the radix map but preserve its capacity
    /// 
    /// # Examples
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
/// # Examples
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
impl<V, const N: usize> TryFrom<[(Bytes, V); N]> for RadixMap<V> {
    type Error = RadixError;

    #[inline]
    fn try_from(value: [(Bytes, V); N]) -> Result<Self, Self::Error> {
        let mut map = RadixMap::default();

        for (path, data) in value {
            map.insert(path, data)?;
        }

        Ok(map)
    }
}

/// Construct from an array of tuples
impl<V, const N: usize> TryFrom<[(&'static str, V); N]> for RadixMap<V> {
    type Error = RadixError;

    #[inline]
    fn try_from(value: [(&'static str, V); N]) -> Result<Self, Self::Error> {
        value.map(|(k, v)| (Bytes::from(k), v)).try_into()
    }
}

/// Default trait
impl<V> Default for RadixMap<V> {
    #[inline]
    fn default() -> Self {
        Self { root: RadixNode::default(), size: 0 }
    }
}

/// Clone trait
/// 
/// # Examples
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
impl<V: Clone> Clone for RadixMap<V> {
    #[inline]
    fn clone(&self) -> Self {
        Self { root: self.root.clone(), size: self.size }
    }
}

/// Debug trait
/// 
/// # Examples
///
/// ```
/// use radixmap::{RadixMap, RadixResult};
///
/// fn main() -> RadixResult<()> {
///     let map = RadixMap::try_from([("/api/v1", 1), ("/api/v2", 2)])?;
///
///     assert_eq!(format!("{:?}", map).as_str(), r#"{b"/api/v1": 1, b"/api/v2": 2}"#);
///
///     Ok(())
/// }
/// ```
impl<V: Debug> Debug for RadixMap<V> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

/// == & !=
impl<V: Eq> Eq for RadixMap<V> {}

/// == & !=
impl<V: PartialEq> PartialEq for RadixMap<V> {
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
pub struct Iter<'n, V> {
    iter: node::Iter<'n, V>
}

impl<'n, V> Iter<'n, V> {
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

impl<'n, V> From<&'n RadixMap<V>> for Iter<'n, V> {
    #[inline]
    fn from(value: &'n RadixMap<V>) -> Self {
        Self { iter: node::Iter::from(&value.root) }
    }
}

impl<'n, V> Iterator for Iter<'n, V> {
    type Item = (&'n Bytes, &'n V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().and_then(|node| node.item_ref())
    }
}

// -----------------------------------------------------------------------------

/// Mutable iterator for map
#[derive(Default)]
pub struct IterMut<'n, V> {
    iter: node::IterMut<'n, V>
}

impl<'n, V> IterMut<'n, V> {
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

impl<'n, V> From<&'n mut RadixMap<V>> for IterMut<'n, V> {
    #[inline]
    fn from(value: &'n mut RadixMap<V>) -> Self {
        Self { iter: node::IterMut::from(&mut value.root) }
    }
}

impl<'n, V> Iterator for IterMut<'n, V> {
    type Item = (&'n Bytes, &'n mut V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().and_then(|node| node.item_mut())
    }
}

// -----------------------------------------------------------------------------

/// Path adapter
#[derive(Default, Clone)]
pub struct Keys<'n, V> {
    iter: Iter<'n, V>
}

impl<'n, V> Keys<'n, V> {
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

impl<'n, V> From<&'n RadixMap<V>> for Keys<'n, V> {
    #[inline]
    fn from(value: &'n RadixMap<V>) -> Self {
        Self { iter: Iter::from(value) }
    }
}

impl<'n, V> Iterator for Keys<'n, V> {
    type Item = &'n Bytes;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|item| item.0)
    }
}

// -----------------------------------------------------------------------------

/// Data adapter
#[derive(Default, Clone)]
pub struct Values<'n, V> {
    iter: Iter<'n, V>
}

impl<'n, V> Values<'n, V> {
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

impl<'n, V> From<&'n RadixMap<V>> for Values<'n, V> {
    #[inline]
    fn from(value: &'n RadixMap<V>) -> Self {
        Self { iter: Iter::from(value) }
    }
}

impl<'n, V> Iterator for Values<'n, V> {
    type Item = &'n V;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|item| item.1)
    }
}

// -----------------------------------------------------------------------------

/// Mutable data adapter
#[derive(Default)]
pub struct ValuesMut<'n, V> {
    iter: IterMut<'n, V>
}

impl<'n, V> ValuesMut<'n, V> {
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

impl<'n, V> From<&'n mut RadixMap<V>> for ValuesMut<'n, V> {
    #[inline]
    fn from(value: &'n mut RadixMap<V>) -> Self {
        Self { iter: IterMut::from(value) }
    }
}

impl<'n, V> Iterator for ValuesMut<'n, V> {
    type Item = &'n mut V;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|item| item.1)
    }
}