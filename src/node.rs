//! Node is the core tree element
use super::pack;
use super::defs::*;
use super::rule::*;

/// The basic element inside a tree
pub struct RadixNode<V> {
    /// The key of the radix map, valid in data-node only
    pub path: Bytes,

    /// The value of the radix map, valid in data-node only
    pub data: Option<V>,

    /// The pattern used for matching, supports plain text, named param, glob and regex
    pub rule: RadixRule,

    /// Node's children
    pub next: pack::RadixPack<V>,
}

impl<V> RadixNode<V> {
    /// Check if the node has no data
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_none()
    }

    /// Get path-data pair
    #[inline]
    pub fn item_ref(&self) -> Option<(&Bytes, &V)> {
        self.data.as_ref().map(|data| (&self.path, data))
    }

    /// Get path-data pair
    #[inline]
    pub fn item_mut(&mut self) -> Option<(&Bytes, &mut V)> {
        self.data.as_mut().map(|data| (&self.path, data))
    }

    /// An iterator for node
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Bytes;
    /// use radixmap::{node::RadixNode, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut node = RadixNode::default();
    ///     node.insert("/api", "api")?;
    ///     node.insert("/api/v1", "v1")?;
    ///     node.insert("/api/v2", "v2")?;
    ///
    ///     let mut iter = node.iter();
    ///
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some((&Bytes::from("/api"), &"api")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some((&Bytes::from("/api/v1"), &"v1")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some((&Bytes::from("/api/v2"), &"v2")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), None);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn iter(&self) -> Iter<V> {
        Iter::from(self)
    }

    /// A mutable iterator for node
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Bytes;
    /// use radixmap::{node::RadixNode, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut node = RadixNode::default();
    ///     node.insert("/api", 0)?;
    ///     node.insert("/api/v1", 1)?;
    ///     node.insert("/api/v2", 2)?;
    ///
    ///     for node in node.iter_mut() {
    ///         node.data = Some(node.data.unwrap_or_default() + 10);
    ///     }
    ///
    ///     let mut iter = node.iter_mut();
    ///
    ///     assert_eq!(iter.next().and_then(|node| node.item_mut()), Some((&Bytes::from("/api"), &mut 10)));
    ///     assert_eq!(iter.next().and_then(|node| node.item_mut()), Some((&Bytes::from("/api/v1"), &mut 11)));
    ///     assert_eq!(iter.next().and_then(|node| node.item_mut()), Some((&Bytes::from("/api/v2"), &mut 12)));
    ///     assert_eq!(iter.next().and_then(|node| node.item_mut()), None);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<V> {
        IterMut::from(self)
    }

    /// Iterator adapter for path
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Bytes;
    /// use radixmap::{node::RadixNode, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut node = RadixNode::default();
    ///     node.insert("/api", ())?;
    ///     node.insert("/api/v1", ())?;
    ///     node.insert("/api/v2", ())?;
    ///
    ///     let mut iter = node.keys();
    ///
    ///     assert_eq!(iter.next(), Some(&Bytes::from("/api")));
    ///     assert_eq!(iter.next(), Some(&Bytes::from("/api/v1")));
    ///     assert_eq!(iter.next(), Some(&Bytes::from("/api/v2")));
    ///     assert_eq!(iter.next(), None);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn keys(&self) -> Keys<V> {
        Keys::from(self)
    }

    /// Iterator adapter for data
    ///
    /// # Examples
    ///
    /// ```
    /// use radixmap::{node::RadixNode, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut node = RadixNode::default();
    ///     node.insert("/api", 0)?;
    ///     node.insert("/api/v1", 1)?;
    ///     node.insert("/api/v2", 2)?;
    ///
    ///     let mut iter = node.values();
    ///
    ///     assert_eq!(iter.next(), Some(&0));
    ///     assert_eq!(iter.next(), Some(&1));
    ///     assert_eq!(iter.next(), Some(&2));
    ///     assert_eq!(iter.next(), None);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn values(&self) -> Values<V> {
        Values::from(self)
    }

    /// Mutable iterator adapter for data
    ///
    /// # Examples
    ///
    /// ```
    /// use radixmap::{node::RadixNode, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut node = RadixNode::default();
    ///     node.insert("/api", 0)?;
    ///     node.insert("/api/v1", 1)?;
    ///     node.insert("/api/v2", 2)?;
    ///
    ///     for node in node.iter_mut() {
    ///         node.data = Some(node.data.unwrap_or_default() + 10);
    ///     }
    ///
    ///     let mut iter = node.values_mut();
    ///
    ///     assert_eq!(iter.next(), Some(&mut 10));
    ///     assert_eq!(iter.next(), Some(&mut 11));
    ///     assert_eq!(iter.next(), Some(&mut 12));
    ///     assert_eq!(iter.next(), None);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn values_mut(&mut self) -> ValuesMut<V> {
        ValuesMut::from(self)
    }

    /// Inserts a path and data into this node, which serves as the root node for the insertion.
    /// The method sequentially extracts path fragments and positions each node appropriately,
    /// ensuring that nodes with a common prefix share a single node in the tree.
    pub fn insert(&mut self, path: impl Into<Bytes>, data: V) -> RadixResult<Option<V>> {
        let path = path.into();
        let mut frag = path.clone();
        let mut slot = self;

        loop {
            // extract the next path fragment and insert it via pack
            let next = RadixRule::try_from(frag.clone())?;
            let used = next.origin().clone();
            slot = slot.next.insert(next)?;

            // encountering a data node indicates completion of insertion
            if used.len() == frag.len() {
                let prev = slot.data.take();
                slot.path = path;
                slot.data = Some(data);
                return Ok(prev);
            }

            frag = frag.slice(used.len()..);
        }
    }

    /// Finds the deepest node that matches the given path.
    ///
    /// - If `data` is true, the function returns the deepest node that is a data node and matches
    ///   the path exactly.
    /// - If `data` is false, the function returns the deepest node that matches the path as far as
    ///   possible, regardless of whether it is a data node or not.
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Bytes;
    /// use radixmap::{node::RadixNode, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut node = RadixNode::default();
    ///     node.insert("/api", "api")?;
    ///     node.insert("/api/v1", "v1")?;
    ///     node.insert("/api/v2", "v2")?;
    ///     node.insert("/api/v1/user/:id", "user1")?;
    ///     node.insert("/api/v2/user/{id:[^0-9]+}", "user2")?;
    ///     node.insert("/api/v3/user/*cde", "user3")?;
    ///
    ///     assert_eq!(node.lookup(b"/", false, &mut vec![], false).map(|node| node.rule.origin()), Some(&Bytes::from("/api")));
    ///     assert_eq!(node.lookup(b"/api", false, &mut vec![], false).map(|node| node.rule.origin()), Some(&Bytes::from("/api")));
    ///     assert_eq!(node.lookup(b"/api/v", false, &mut vec![], false).map(|node| node.rule.origin()), Some(&Bytes::from("/v")));
    ///     assert_eq!(node.lookup(b"/api/v1", false, &mut vec![], false).map(|node| node.rule.origin()), Some(&Bytes::from("1")));
    ///     assert_eq!(node.lookup(b"/api/v2", false, &mut vec![], false).map(|node| node.rule.origin()), Some(&Bytes::from("2")));
    ///     assert_eq!(node.lookup(b"/api/v3", false, &mut vec![], false).map(|node| node.rule.origin()), Some(&Bytes::from("3/user/")));
    ///
    ///     assert_eq!(node.lookup(b"/", true, &mut vec![], false).map(|node| node.rule.origin()), None);
    ///     assert_eq!(node.lookup(b"/api", true, &mut vec![], false).map(|node| node.rule.origin()), Some(&Bytes::from("/api")));
    ///     assert_eq!(node.lookup(b"/api/v", true, &mut vec![], false).map(|node| node.rule.origin()), None);
    ///     assert_eq!(node.lookup(b"/api/v1", true, &mut vec![], false).map(|node| node.rule.origin()), Some(&Bytes::from("1")));
    ///     assert_eq!(node.lookup(b"/api/v2", true, &mut vec![], false).map(|node| node.rule.origin()), Some(&Bytes::from("2")));
    ///     assert_eq!(node.lookup(b"/api/v1/user/12345", true, &mut vec![], false).map(|node| node.rule.origin()), Some(&Bytes::from(":id")));
    ///     assert_eq!(node.lookup(b"/api/v2/user/12345", true, &mut vec![], false).map(|node| node.rule.origin()), None);
    ///     assert_eq!(node.lookup(b"/api/v2/user/abcde", true, &mut vec![], false).map(|node| node.rule.origin()), Some(&Bytes::from("{id:[^0-9]+}")));
    ///     assert_eq!(node.lookup(b"/api/v3/user/12345", true, &mut vec![], false).map(|node| node.rule.origin()), None);
    ///     assert_eq!(node.lookup(b"/api/v3/user/abcde", true, &mut vec![], false).map(|node| node.rule.origin()), Some(&Bytes::from("*cde")));
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn lookup<'u>(&self, mut path: &'u [u8], data: bool, capture: &mut Vec<(Bytes, &'u [u8])>, enable: bool) -> Option<&RadixNode<V>> {
        let mut current = self;

        loop {
            // prefix must be part of the current node
            let share = match current.rule.longest(path) {
                Some(val) => val,
                None => return None,
            };
            let equal = current.rule.is_special() || current.rule.origin().len() == share.len();
            if share.len() != path.len() && !equal {
                return None
            }

            if enable {
                let ident = current.rule.identity();
                if !ident.is_empty() {
                    capture.push((ident.clone(), share));
                }
            }

            // trim the shared and continue lookup
            path = &path[share.len()..];

            let byte = match path.first() {
                Some(&val) => val as usize,
                None if data && (!equal || current.is_empty()) => 0, // data node must be an exact match
                None => return Some(current),
            };

            // find regular node by vector map
            if let Some(node) = current.next.regular.get(byte) {
                current = node;
                continue;
            }

            // find special node, if not then terminate
            for node in current.next.special.values() {
                if let Some(find) = node.lookup(path, data, capture, enable) {
                    return Some(find);
                }
            }

            return None;
        }
    }

    /// Same as lookup
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Bytes;
    /// use radixmap::{node::RadixNode, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut node = RadixNode::default();
    ///     node.insert("/api", "api")?;
    ///     node.insert("/api/v1", "v1")?;
    ///     node.insert("/api/v2", "v2")?;
    ///     node.insert("/api/v1/user/:id", "user1")?;
    ///     node.insert("/api/v2/user/{id:[^0-9]+}", "user2")?;
    ///     node.insert("/api/v3/user/*cde", "user3")?;
    ///
    ///     assert_eq!(node.lookup_mut(b"/", false, &mut vec![], false).map(|node| node.rule.origin()), Some(&Bytes::from("/api")));
    ///     assert_eq!(node.lookup_mut(b"/api", false, &mut vec![], false).map(|node| node.rule.origin()), Some(&Bytes::from("/api")));
    ///     assert_eq!(node.lookup_mut(b"/api/v", false, &mut vec![], false).map(|node| node.rule.origin()), Some(&Bytes::from("/v")));
    ///     assert_eq!(node.lookup_mut(b"/api/v1", false, &mut vec![], false).map(|node| node.rule.origin()), Some(&Bytes::from("1")));
    ///     assert_eq!(node.lookup_mut(b"/api/v2", false, &mut vec![], false).map(|node| node.rule.origin()), Some(&Bytes::from("2")));
    ///     assert_eq!(node.lookup_mut(b"/api/v3", false, &mut vec![], false).map(|node| node.rule.origin()), Some(&Bytes::from("3/user/")));
    ///
    ///     assert_eq!(node.lookup_mut(b"/", true, &mut vec![], false).map(|node| node.rule.origin()), None);
    ///     assert_eq!(node.lookup_mut(b"/api", true, &mut vec![], false).map(|node| node.rule.origin()), Some(&Bytes::from("/api")));
    ///     assert_eq!(node.lookup_mut(b"/api/v", true, &mut vec![], false).map(|node| node.rule.origin()), None);
    ///     assert_eq!(node.lookup_mut(b"/api/v1", true, &mut vec![], false).map(|node| node.rule.origin()), Some(&Bytes::from("1")));
    ///     assert_eq!(node.lookup_mut(b"/api/v2", true, &mut vec![], false).map(|node| node.rule.origin()), Some(&Bytes::from("2")));
    ///     assert_eq!(node.lookup_mut(b"/api/v1/user/12345", true, &mut vec![], false).map(|node| node.rule.origin()), Some(&Bytes::from(":id")));
    ///     assert_eq!(node.lookup_mut(b"/api/v2/user/12345", true, &mut vec![], false).map(|node| node.rule.origin()), None);
    ///     assert_eq!(node.lookup_mut(b"/api/v2/user/abcde", true, &mut vec![], false).map(|node| node.rule.origin()), Some(&Bytes::from("{id:[^0-9]+}")));
    ///     assert_eq!(node.lookup_mut(b"/api/v3/user/12345", true, &mut vec![], false).map(|node| node.rule.origin()), None);
    ///     assert_eq!(node.lookup_mut(b"/api/v3/user/abcde", true, &mut vec![], false).map(|node| node.rule.origin()), Some(&Bytes::from("*cde")));
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn lookup_mut<'u>(&mut self, mut path: &'u [u8], data: bool, capture: &mut Vec<(Bytes, &'u [u8])>, enable: bool) -> Option<&mut RadixNode<V>> {
        let mut current = self;

        loop {
            // prefix must be part of the current node
            let share = match current.rule.longest(path) {
                Some(val) => val,
                None => return None,
            };
            let equal = current.rule.is_special() || current.rule.origin().len() == share.len();
            if share.len() != path.len() && !equal {
                return None
            }

            if enable {
                let ident = current.rule.identity();
                if !ident.is_empty() {
                    capture.push((ident.clone(), share));
                }
            }

            // trim the shared and continue lookup
            path = &path[share.len()..];

            let byte = match path.first() {
                Some(&val) => val as usize,
                None if data && (!equal || current.is_empty()) => 0, // data node must be an exact match
                None => return Some(current),
            };

            // find regular node by vector map
            if let Some(node) = current.next.regular.get_mut(byte) {
                current = node;
                continue;
            }

            // find special node, if not then terminate
            for node in current.next.special.values_mut() {
                if let Some(find) = node.lookup_mut(path, data, capture, enable) {
                    return Some(find);
                }
            }

            return None;
        }
    }

    /// Divide the node into two parts
    ///
    /// # Examples
    ///
    /// ```
    /// use radixmap::{node::RadixNode, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut node = RadixNode::try_from(("/api", 12345))?;
    ///
    ///     assert_eq!(node.rule, b"/api");
    ///     assert_eq!(node.data, Some(12345));
    ///
    ///     let frag = node.divide(1)?;
    ///
    ///     assert_eq!(node.rule, b"/");
    ///     assert_eq!(node.data, None);
    ///     assert_eq!(frag.rule, b"api");
    ///     assert_eq!(frag.data, Some(12345));
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn divide(&mut self, len: usize) -> RadixResult<RadixNode<V>> {
        Ok(RadixNode {
            path: std::mem::take(&mut self.path),
            data: self.data.take(),

            rule: self.rule.divide(len)?,
            next: std::mem::take(&mut self.next),
        })
    }

    /// Clear the nodes but preserve its capacity
    ///
    /// # Examples
    ///
    /// ```
    /// use radixmap::{node::RadixNode, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut node = RadixNode::try_from(("/api", ()))?;
    ///     node.insert("/api/v1", ())?;
    ///
    ///     assert_eq!(node.is_empty(), false);
    ///
    ///     node.clear();
    ///
    ///     assert_eq!(node.is_empty(), true);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        self.path.clear();
        self.data = None;
        self.rule = RadixRule::default();
        self.next.clear();
    }
}

/// Create a node from a rule
///
/// # Examples
///
/// ```
/// use radixmap::{node::RadixNode, rule::RadixRule, RadixResult};
///
/// fn main() -> RadixResult<()> {
///     assert_eq!(RadixNode::<()>::from(RadixRule::try_from("/api")?).rule, b"/api");
///     assert_eq!(RadixNode::<()>::from(RadixRule::try_from(":id")?).rule, b":id");
///
///     Ok(())
/// }
/// ```
impl<V> From<RadixRule> for RadixNode<V> {
    #[inline]
    fn from(rule: RadixRule) -> Self {
        Self { path: Bytes::new(), data: None, rule, next: Default::default() }
    }
}

/// Create a node from (path, data)
///
/// # Examples
///
/// ```
/// use radixmap::{node::RadixNode, RadixResult};
///
/// fn main() -> RadixResult<()> {
///     assert_eq!(RadixNode::try_from(("/api", ()))?.rule, b"/api");
///     assert_eq!(RadixNode::try_from((":id", ()))?.rule, b":id");
///
///     Ok(())
/// }
/// ```
impl<V> TryFrom<(Bytes, V)> for RadixNode<V> {
    type Error = RadixError;

    #[inline]
    fn try_from((path, data): (Bytes, V)) -> RadixResult<Self> {
        Ok(Self { path: path.clone(), data: Some(data), rule: RadixRule::try_from(path)?, next: Default::default() })
    }
}

/// Create a node from (path, data)
impl<V> TryFrom<(&'static [u8], V)> for RadixNode<V> {
    type Error = RadixError;

    #[inline]
    fn try_from((path, data): (&'static [u8], V)) -> RadixResult<Self> {
        (Bytes::from(path), data).try_into()
    }
}

/// Create a node from (path, data)
impl<V> TryFrom<(&'static str, V)> for RadixNode<V> {
    type Error = RadixError;

    #[inline]
    fn try_from((path, data): (&'static str, V)) -> RadixResult<Self> {
        (Bytes::from(path), data).try_into()
    }
}

/// Default trait
/// ```
/// use radixmap::{node::RadixNode};
///
/// let mut node = RadixNode::default();
/// assert!(node.insert("/api", ()).is_ok());
/// ```
impl<V> Default for RadixNode<V> {
    #[inline]
    fn default() -> Self {
        Self { path: Bytes::new(), data: None, rule: RadixRule::default(), next: pack::RadixPack::default() }
    }
}

/// Debug trait
///
/// # Examples
///
/// ```
/// use radixmap::{node::RadixNode, RadixResult};
///
/// fn main() -> RadixResult<()> {
///     assert_eq!(format!("{:?}", RadixNode::try_from((r"/api", ()))?).as_str(), r"Plain(/api)");
///     assert_eq!(format!("{:?}", RadixNode::try_from((r":id", ()))?).as_str(), r"Param(:id)");
///     assert_eq!(format!("{:?}", RadixNode::try_from((r"*", ()))?).as_str(), r"Glob(*)");
///     assert_eq!(format!("{:?}", RadixNode::try_from((r"{id:\d+}", ()))?).as_str(), r"Regex({id:\d+})");
///
///     Ok(())
/// }
/// ```
impl<V> Debug for RadixNode<V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.rule.fmt(f)
    }
}

/// Clone trait
/// ```
/// use radixmap::{node::RadixNode, RadixResult};
///
/// fn main() -> RadixResult<()> {
///     let mut node_a = RadixNode::try_from(("/api", 123))?;
///     let mut node_b = node_a.clone();
///
///     assert_eq!(node_a.path, node_b.path);
///     assert_eq!(node_a.data, node_b.data);
///     assert_eq!(node_a.rule, node_b.rule);
///
///     Ok(())
/// }
/// ```
impl<V: Clone> Clone for RadixNode<V> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            path: self.path.clone(),
            data: self.data.clone(),
            rule: self.rule.clone(),
            next: self.next.clone(),
        }
    }
}

// -----------------------------------------------------------------------------

/// Iterating order for radix tree
///
/// # Example
///
/// 1a - 2a - 3a
///    └ 2b
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Order {
    /// Pre-order traversal: 1a -> 2a -> 3a -> 2b
    Pre,

    /// Post-order traversal: 3a -> 2a -> 2b -> 1a
    ///
    /// Note that mutable iterators do not currently support this order
    Post,

    /// Level-order traversal: 1a -> 2a -> 2b -> 3a
    Level
}

impl Default for Order {
    #[inline]
    fn default() -> Self {
        Self::Pre
    }
}

// -----------------------------------------------------------------------------

/// The iterator for radix tree
#[derive(Default, Clone)]
pub struct Iter<'n, V> {
    queue: VecDeque<Peekable<pack::Iter<'n, V>>>,
    visit: Vec<Peekable<pack::Iter<'n, V>>>, // used in post-order only
    order: Order,
    empty: bool,
}

impl<'n, V> Iter<'n, V> {
    /// Starting to iterate from the node with a specific prefix
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Bytes;
    /// use radixmap::{node::RadixNode, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut node = RadixNode::default();
    ///     node.insert("/api", "api")?;
    ///     node.insert("/api/v1", "v1")?;
    ///     node.insert("/api/v1/user", "user1")?;
    ///     node.insert("/api/v2", "v2")?;
    ///     node.insert("/api/v2/user", "user2")?;
    ///
    ///     let mut iter = node.iter().with_prefix(b"/api/v1", false);
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some((&Bytes::from("/api/v1"), &"v1")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some((&Bytes::from("/api/v1/user"), &"user1")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), None);
    ///
    ///     let mut iter = node.iter().with_prefix(b"/api/", false); // exclude /api
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some((&Bytes::from("/api/v1"), &"v1")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some((&Bytes::from("/api/v1/user"), &"user1")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some((&Bytes::from("/api/v2"), &"v2")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some((&Bytes::from("/api/v2/user"), &"user2")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), None);
    ///
    ///     let mut iter = node.iter().with_prefix(b"/api/v3", false); // not exist
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), None);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn with_prefix(mut self, path: &[u8], data: bool) -> Self {
        let cursor = self.queue.pop_front();

        self.queue.clear();
        self.visit.clear();

        let cursor = cursor
            .and_then(|mut iter| iter.next())
            .and_then(|node| match !path.is_empty() {
                true => node.lookup(path, data, &mut vec![], false),
                false => None,
            });

        if let Some(cursor) = cursor {
            self.queue.push_front(pack::Iter::from(cursor).peekable());
        }

        self
    }

    /// Change the iterating order
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Bytes;
    /// use radixmap::{node::{RadixNode, Order}, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut node = RadixNode::default();
    ///     node.insert("/api", "api")?;
    ///     node.insert("/api/v1", "v1")?;
    ///     node.insert("/api/v1/user", "user1")?;
    ///     node.insert("/api/v2", "v2")?;
    ///     node.insert("/api/v2/user", "user2")?;
    ///
    ///     let mut iter = node.iter(); // same as with_order(Order::Pre);
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some((&Bytes::from("/api"), &"api")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some((&Bytes::from("/api/v1"), &"v1")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some((&Bytes::from("/api/v1/user"), &"user1")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some((&Bytes::from("/api/v2"), &"v2")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some((&Bytes::from("/api/v2/user"), &"user2")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), None);
    ///
    ///     let mut iter = node.iter().with_order(Order::Post);
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some((&Bytes::from("/api/v1/user"), &"user1")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some((&Bytes::from("/api/v1"), &"v1")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some((&Bytes::from("/api/v2/user"), &"user2")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some((&Bytes::from("/api/v2"), &"v2")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some((&Bytes::from("/api"), &"api")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), None);
    ///
    ///     let mut iter = node.iter().with_order(Order::Level);
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some((&Bytes::from("/api"), &"api")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some((&Bytes::from("/api/v1"), &"v1")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some((&Bytes::from("/api/v2"), &"v2")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some((&Bytes::from("/api/v1/user"), &"user1")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some((&Bytes::from("/api/v2/user"), &"user2")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), None);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn with_order(mut self, order: Order) -> Self {
        self.order = order;
        self
    }

    /// Traverse all nodes, including the internal nodes which do not contain data
    ///
    /// # Examples
    ///
    /// ```
    /// use radixmap::{node::RadixNode, RadixResult};
    ///
    /// macro_rules! verify {
    ///     ($iter:expr, $orig:literal, $data:expr) => {{
    ///         let node = match $iter.next() {
    ///             Some(node) => node,
    ///             None => unreachable!()
    ///         };
    ///         assert_eq!(node.rule, $orig);
    ///         assert_eq!(node.data, $data);
    ///     }};
    /// }
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut node = RadixNode::default();
    ///     node.insert("/api", "api")?;
    ///     node.insert("/api/v1", "v1")?;
    ///     node.insert("/api/v1/user", "user1")?;
    ///     node.insert("/api/v2", "v2")?;
    ///     node.insert("/api/v2/user", "user2")?;
    ///
    ///     let mut iter = node.iter().with_empty();
    ///     verify!(iter, b"", None);                        // the root node
    ///     verify!(iter, b"/api", Some("api"));
    ///     verify!(iter, b"/v", None);                      // an internal node
    ///     verify!(iter, b"1", Some("v1"));
    ///     verify!(iter, b"/user", Some("user1"));
    ///     verify!(iter, b"2", Some("v2"));
    ///     verify!(iter, b"/user", Some("user2"));
    ///     assert!(iter.next().is_none());
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn with_empty(mut self) -> Self {
        self.empty = true;
        self
    }

    /// Internal use only, traversing nodes in pre-order
    fn next_pre(&mut self) -> Option<&'n RadixNode<V>> {
        loop {
            let back = self.queue.back_mut()?;
            match back.next() {
                Some(node) => {
                    self.queue.push_back(node.next.iter().peekable());
                    return Some(node);
                }
                None => { self.queue.pop_back(); }
            }
        }
    }

    /// Internal use only, traversing nodes in post-order
    fn next_post(&mut self) -> Option<&'n RadixNode<V>> {
        // traverse to the deepest data node, put all iters into the visit queue
        if let Some(mut back) = self.queue.pop_back() {
            while let Some(node) = back.peek() {
                let pack = node.next.iter().peekable();
                self.visit.push(back);
                back = pack;
            }

            return self.next_post();
        }

        // pop node from visit queue, re-push iter if the next node is not empty
        loop {
            let mut back = self.visit.pop()?;
            if let Some(node) = back.next() {
                if back.peek().is_some() {
                    self.queue.push_back(back);
                }

                return Some(node);
            }
        }
    }

    /// Internal use only, traversing nodes in level-order
    fn next_level(&mut self) -> Option<&'n RadixNode<V>> {
        loop {
            let front = self.queue.front_mut()?;
            match front.next() {
                Some(node) => {
                    self.queue.push_back(node.next.iter().peekable());
                    return Some(node);
                }
                None => { self.queue.pop_front(); }
            }
        }
    }
}

impl<'n, V> From<&'n RadixNode<V>> for Iter<'n, V> {
    #[inline]
    fn from(start: &'n RadixNode<V>) -> Self {
        Self { queue: VecDeque::from([pack::Iter::from(start).peekable()]), visit: vec![], order: Order::Pre, empty: false }
    }
}

impl<'n, V> Iterator for Iter<'n, V> {
    type Item = &'n RadixNode<V>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let node = match self.order {
                Order::Pre => self.next_pre(),
                Order::Post => self.next_post(),
                Order::Level => self.next_level(),
            };

            // check if user need to traverse empty node
            match node {
                Some(node) if !self.empty && node.is_empty() => continue,
                _ => return node,
            }
        }
    }
}

// -----------------------------------------------------------------------------

/// The iterator for radix tree
#[derive(Default)]
pub struct IterMut<'n, V> {
    queue: VecDeque<pack::IterMut<'n, V>>,
    order: Order,
    empty: bool,
}

impl<'n, V> IterMut<'n, V> {
    /// Starting to iterate from the node with a specific prefix
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Bytes;
    /// use radixmap::{node::RadixNode, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut node = RadixNode::default();
    ///     node.insert("/api", "api")?;
    ///     node.insert("/api/v1", "v1")?;
    ///     node.insert("/api/v1/user", "user1")?;
    ///     node.insert("/api/v2", "v2")?;
    ///     node.insert("/api/v2/user", "user2")?;
    ///
    ///     let mut iter = node.iter_mut().with_prefix(b"/api/v1", false);
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some((&Bytes::from("/api/v1"), &"v1")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some((&Bytes::from("/api/v1/user"), &"user1")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), None);
    ///
    ///     let mut iter = node.iter_mut().with_prefix(b"/api/", false); // exclude /api
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some((&Bytes::from("/api/v1"), &"v1")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some((&Bytes::from("/api/v1/user"), &"user1")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some((&Bytes::from("/api/v2"), &"v2")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some((&Bytes::from("/api/v2/user"), &"user2")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), None);
    ///
    ///     let mut iter = node.iter_mut().with_prefix(b"/api/v3", false); // not exist
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), None);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn with_prefix(mut self, path: &[u8], data: bool) -> Self {
        let cursor = self.queue.pop_front();

        self.queue.clear();

        let cursor = cursor
            .and_then(|mut iter| iter.next())
            .and_then(|node| match !path.is_empty() {
                true => node.lookup_mut(path, data, &mut vec![], false),
                false => None,
            });

        if let Some(cursor) = cursor {
            self.queue.push_front(pack::IterMut::from(cursor));
        }

        self
    }

    /// Change the iterating order
    ///
    /// # Examples
    ///
    /// ```
    /// use bytes::Bytes;
    /// use radixmap::{node::{RadixNode, Order}, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut node = RadixNode::default();
    ///     node.insert("/api", "api")?;
    ///     node.insert("/api/v1", "v1")?;
    ///     node.insert("/api/v1/user", "user1")?;
    ///     node.insert("/api/v2", "v2")?;
    ///     node.insert("/api/v2/user", "user2")?;
    ///
    ///     let mut iter = node.iter_mut(); // same as with_order(Order::Pre);
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some((&Bytes::from("/api"), &"api")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some((&Bytes::from("/api/v1"), &"v1")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some((&Bytes::from("/api/v1/user"), &"user1")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some((&Bytes::from("/api/v2"), &"v2")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some((&Bytes::from("/api/v2/user"), &"user2")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), None);
    ///
    ///     let mut iter = node.iter_mut().with_order(Order::Level);
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some((&Bytes::from("/api"), &"api")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some((&Bytes::from("/api/v1"), &"v1")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some((&Bytes::from("/api/v2"), &"v2")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some((&Bytes::from("/api/v1/user"), &"user1")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some((&Bytes::from("/api/v2/user"), &"user2")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), None);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn with_order(mut self, order: Order) -> Self {
        self.order = order;
        self
    }

    /// Traverse all nodes, including the internal nodes which do not contain data
    ///
    /// # Examples
    ///
    /// ```
    /// use radixmap::{node::RadixNode, RadixResult};
    ///
    /// macro_rules! verify {
    ///     ($iter:expr, $orig:literal, $data:expr) => {{
    ///         let node = match $iter.next() {
    ///             Some(node) => node,
    ///             None => unreachable!()
    ///         };
    ///         assert_eq!(node.rule, $orig);
    ///         assert_eq!(node.data, $data);
    ///     }};
    /// }
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut node = RadixNode::default();
    ///     node.insert("/api", "api")?;
    ///     node.insert("/api/v1", "v1")?;
    ///     node.insert("/api/v1/user", "user1")?;
    ///     node.insert("/api/v2", "v2")?;
    ///     node.insert("/api/v2/user", "user2")?;
    ///
    ///     let mut iter = node.iter_mut().with_empty();
    ///     verify!(iter, b"", None);                        // the root node
    ///     verify!(iter, b"/api", Some("api"));
    ///     verify!(iter, b"/v", None);                      // an internal node
    ///     verify!(iter, b"1", Some("v1"));
    ///     verify!(iter, b"/user", Some("user1"));
    ///     verify!(iter, b"2", Some("v2"));
    ///     verify!(iter, b"/user", Some("user2"));
    ///     assert!(iter.next().is_none());
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn with_empty(mut self) -> Self {
        self.empty = true;
        self
    }

    /// Internal use only, traversing nodes in pre-order
    ///
    /// # Safety
    ///
    /// DO NOT MODIFY THE RETURNED NODE'S `next` FIELD
    fn next_pre(&mut self) -> Option<&'n mut RadixNode<V>> {
        loop {
            let back = self.queue.back_mut()?;
            match back.next() {
                Some(node) => {
                    let ptr = node as *mut RadixNode<V>;
                    self.queue.push_back(node.next.iter_mut());
                    unsafe { return Some(&mut *ptr); }
                }
                None => { self.queue.pop_back(); }
            }
        }
    }

    /// Internal use only, traversing nodes in level-order
    ///
    /// # Safety
    ///
    /// DO NOT MODIFY THE RETURNED NODE'S `next` FIELD
    fn next_level(&mut self) -> Option<&'n mut RadixNode<V>> {
        loop {
            let front = self.queue.front_mut()?;
            match front.next() {
                Some(node) => {
                    let ptr = node as *mut RadixNode<V>;
                    self.queue.push_back(node.next.iter_mut());
                    unsafe { return Some(&mut *ptr); }
                }
                None => { self.queue.pop_front(); }
            }
        }
    }
}

impl<'n, V> From<&'n mut RadixNode<V>> for IterMut<'n, V> {
    #[inline]
    fn from(start: &'n mut RadixNode<V>) -> Self {
        Self { queue: VecDeque::from([pack::IterMut::from(start)]), order: Order::Pre, empty: false }
    }
}

impl<'n, V> Iterator for IterMut<'n, V> {
    type Item = &'n mut RadixNode<V>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let node = match self.order {
                Order::Pre => self.next_pre(),
                Order::Level => self.next_level(),
                _ => unimplemented!()
            };

            // check if user need to traverse empty node
            match node {
                Some(node) if !self.empty && node.is_empty() => continue,
                _ => return node,
            }
        }
    }
}

// -----------------------------------------------------------------------------

/// Iterator adapter for path
#[derive(Default, Clone)]
pub struct Keys<'n, V> {
    iter: Iter<'n, V>
}

impl<'n, V> Keys<'n, V> {
    /// Starting to iterate from the node with a specific prefix
    #[inline]
    pub fn with_prefix(mut self, path: &[u8], data: bool) -> Self {
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

impl<'n, V> From<&'n RadixNode<V>> for Keys<'n, V> {
    #[inline]
    fn from(value: &'n RadixNode<V>) -> Self {
        Self { iter: Iter::from(value) }
    }
}

impl<'n, V> Iterator for Keys<'n, V> {
    type Item = &'n Bytes;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|node| &node.path)
    }
}

// -----------------------------------------------------------------------------

/// Iterator adapter for data
#[derive(Default, Clone)]
pub struct Values<'n, V> {
    iter: Iter<'n, V>
}

impl<'n, V> Values<'n, V> {
    /// Starting to iterate from the node with a specific prefix
    #[inline]
    pub fn with_prefix(mut self, path: &[u8], data: bool) -> Self {
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

impl<'n, V> From<&'n RadixNode<V>> for Values<'n, V> {
    #[inline]
    fn from(value: &'n RadixNode<V>) -> Self {
        Self { iter: Iter::from(value) }
    }
}

impl<'n, V> Iterator for Values<'n, V> {
    type Item = &'n V;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().and_then(|node| node.data.as_ref())
    }
}

// -----------------------------------------------------------------------------

/// Mutable iterator adapter for data
#[derive(Default)]
pub struct ValuesMut<'n, V> {
    iter: IterMut<'n, V>
}

impl<'n, V> ValuesMut<'n, V> {
    /// Starting to iterate from the node with a specific prefix
    #[inline]
    pub fn with_prefix(mut self, path: &[u8], data: bool) -> Self {
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

impl<'n, V> From<&'n mut RadixNode<V>> for ValuesMut<'n, V> {
    #[inline]
    fn from(value: &'n mut RadixNode<V>) -> Self {
        Self { iter: IterMut::from(value) }
    }
}

impl<'n, V> Iterator for ValuesMut<'n, V> {
    type Item = &'n mut V;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().and_then(|node| node.data.as_mut())
    }
}