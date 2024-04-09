//! Node is the core tree element
use super::pack;
use super::defs::*;
use super::rule::*;

/// The basic element inside a tree
pub struct RadixNode<'k, V> {
    /// The key of the radix map, valid in data-node only
    pub path: &'k str,

    /// The value of the radix map, valid in data-node only
    pub data: Option<V>,

    /// The pattern used for matching, supports plain text, named params, regex and glob
    pub rule: RadixRule<'k>,

    /// Node's children
    pub next: pack::RadixPack<'k, V>,
}

impl<'k, V> RadixNode<'k, V> {
    /// Check if the node has no data
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_none()
    }

    /// Get path-data pair
    #[inline]
    pub fn item_ref(&self) -> Option<(&str, &V)> {
        self.data.as_ref().map(|data| (self.path, data))
    }

    /// Get path-data pair
    #[inline]
    pub fn item_mut(&mut self) -> Option<(&str, &mut V)> {
        self.data.as_mut().map(|data| (self.path, data))
    }

    /// An iterator for node
    ///
    /// ```
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
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some(("/api", &"api")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some(("/api/v1", &"v1")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some(("/api/v2", &"v2")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), None);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn iter(&self) -> Iter<'_, V> {
        Iter::from(self)
    }

    /// A mutable iterator for node
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
    ///     let mut iter = node.iter_mut();
    ///
    ///     assert_eq!(iter.next().and_then(|node| node.item_mut()), Some(("/api", &mut 10)));
    ///     assert_eq!(iter.next().and_then(|node| node.item_mut()), Some(("/api/v1", &mut 11)));
    ///     assert_eq!(iter.next().and_then(|node| node.item_mut()), Some(("/api/v2", &mut 12)));
    ///     assert_eq!(iter.next().and_then(|node| node.item_mut()), None);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, 'k, V> {
        IterMut::from(self)
    }

    /// Iterator adapter for path
    ///
    /// ```
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
    ///     assert_eq!(iter.next(), Some("/api"));
    ///     assert_eq!(iter.next(), Some("/api/v1"));
    ///     assert_eq!(iter.next(), Some("/api/v2"));
    ///     assert_eq!(iter.next(), None);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn keys(&self) -> Keys<'_, V> {
        Keys::from(self)
    }

    /// Iterator adapter for data
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
    pub fn values(&self) -> Values<'_, V> {
        Values::from(self)
    }

    /// Mutable iterator adapter for data
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
    pub fn values_mut(&mut self) -> ValuesMut<'_, 'k, V> {
        ValuesMut::from(self)
    }

    /// Inserts a path and data into this node, which serves as the root node for the insertion.
    /// The method sequentially extracts path fragments and positions each node appropriately,
    /// ensuring that nodes with a common prefix share a single node in the tree.
    pub fn insert(&mut self, path: &'k str, data: V) -> RadixResult<Option<V>> {
        let mut frag = path;

        loop {
            // extract the next path fragment and insert it via pack
            let next = RadixRule::try_from(frag)?;
            let used = next.origin();
            let slot = self.next.insert(next)?;

            // encountering a data node indicates completion of insertion
            if used.len() == frag.len() {
                let prev = slot.data.take();
                slot.path = path;
                slot.data = Some(data);
                return Ok(prev);
            }

            frag = &frag[used.len()..];
        }
    }

    /// Finds the deepest node that matches the given path.
    ///
    /// - If `data` is true, the function returns the deepest node that is a data node and matches
    ///   the path exactly.
    /// - If `data` is false, the function returns the deepest node that matches the path as far as
    ///   possible, regardless of whether it is a data node or not.
    ///
    /// ```
    /// use radixmap::{node::RadixNode, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut node = RadixNode::default();
    ///     node.insert("/api", "api")?;
    ///     node.insert("/api/v1", "v1")?;
    ///     node.insert("/api/v2", "v2")?;
    ///
    ///     assert_eq!(node.search("/", false).map(|node| node.rule.origin()), Some("/api"));
    ///     assert_eq!(node.search("/api", false).map(|node| node.rule.origin()), Some("/api"));
    ///     assert_eq!(node.search("/api/v", false).map(|node| node.rule.origin()), Some("/v"));
    ///     assert_eq!(node.search("/api/v1", false).map(|node| node.rule.origin()), Some("1"));
    ///     assert_eq!(node.search("/api/v2", false).map(|node| node.rule.origin()), Some("2"));
    ///
    ///     assert_eq!(node.search("/", true).map(|node| node.rule.origin()), None);
    ///     assert_eq!(node.search("/api", true).map(|node| node.rule.origin()), Some("/api"));
    ///     assert_eq!(node.search("/api/v", true).map(|node| node.rule.origin()), None);
    ///     assert_eq!(node.search("/api/v1", true).map(|node| node.rule.origin()), Some("1"));
    ///     assert_eq!(node.search("/api/v2", true).map(|node| node.rule.origin()), Some("2"));
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn search(&self, mut path: &str, data: bool) -> Option<&RadixNode<'k, V>> {
        let mut cursor = self;

        loop {
            // prefix must be part of the current node
            let (share, order) = cursor.rule.longest(path);
            if share.len() != path.len() && order != Ordering::Equal {
                return None;
            }

            // trim the shared and continue search
            path = &path[share.len()..];

            let byte = match path.as_bytes().first() {
                Some(&val) => val as usize,
                None if data && (order == Ordering::Greater || cursor.is_empty()) => return None, // data node must be an exact match
                None => return Some(cursor),
            };

            // find regular node by vector map
            if let Some(node) = cursor.next.regular.get(byte) {
                cursor = node;
                continue;
            }

            // find special node, if not then terminate
            for node in cursor.next.special.values() {
                if let Some(find) = node.search(path, data) {
                    return Some(find);
                }
            }

            return None;
        }
    }

    /// Same as search
    ///
    /// ```
    /// use radixmap::{node::RadixNode, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut node = RadixNode::default();
    ///     node.insert("/api", "api")?;
    ///     node.insert("/api/v1", "v1")?;
    ///     node.insert("/api/v2", "v2")?;
    ///
    ///     assert_eq!(node.search_mut("/", false).map(|node| node.rule.origin()), Some("/api"));
    ///     assert_eq!(node.search_mut("/api", false).map(|node| node.rule.origin()), Some("/api"));
    ///     assert_eq!(node.search_mut("/api/v", false).map(|node| node.rule.origin()), Some("/v"));
    ///     assert_eq!(node.search_mut("/api/v1", false).map(|node| node.rule.origin()), Some("1"));
    ///     assert_eq!(node.search_mut("/api/v2", false).map(|node| node.rule.origin()), Some("2"));
    ///
    ///     assert_eq!(node.search_mut("/", true).map(|node| node.rule.origin()), None);
    ///     assert_eq!(node.search_mut("/api", true).map(|node| node.rule.origin()), Some("/api"));
    ///     assert_eq!(node.search_mut("/api/v", true).map(|node| node.rule.origin()), None);
    ///     assert_eq!(node.search_mut("/api/v1", true).map(|node| node.rule.origin()), Some("1"));
    ///     assert_eq!(node.search_mut("/api/v2", true).map(|node| node.rule.origin()), Some("2"));
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn search_mut(&mut self, mut path: &str, data: bool) -> Option<&mut RadixNode<'k, V>> {
        let mut cursor = self;

        loop {
            // prefix must be part of the current node
            let (share, order) = cursor.rule.longest(path);
            if share.len() != path.len() && order != Ordering::Equal {
                return None;
            }

            // trim the shared and continue search
            path = &path[share.len()..];

            let byte = match path.as_bytes().first() {
                Some(&val) => val as usize,
                None if data && (order == Ordering::Greater || cursor.is_empty()) => return None, // data node must be an exact match
                None => return Some(cursor),
            };

            // find regular node by vector map
            if let Some(node) = cursor.next.regular.get_mut(byte) {
                cursor = node;
                continue;
            }

            // find special node, if not then terminate
            for node in cursor.next.special.values_mut() {
                if let Some(find) = node.search_mut(path, data) {
                    return Some(find);
                }
            }

            return None;
        }
    }

    /// Divide the node into two parts
    ///
    /// ```
    /// use radixmap::{node::RadixNode, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut node = RadixNode::try_from(("/api", 12345))?;
    ///
    ///     assert_eq!(node.rule, "/api");
    ///     assert_eq!(node.data, Some(12345));
    ///
    ///     let frag = node.divide(1)?;
    ///
    ///     assert_eq!(node.rule, "/");
    ///     assert_eq!(node.data, None);
    ///     assert_eq!(frag.rule, "api");
    ///     assert_eq!(frag.data, Some(12345));
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn divide(&mut self, len: usize) -> RadixResult<RadixNode<'k, V>> {
        Ok(RadixNode {
            path: std::mem::take(&mut self.path),
            data: self.data.take(),

            rule: self.rule.divide(len)?,
            next: std::mem::take(&mut self.next),
        })
    }

    /// Clear the nodes but preserve its capacity
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
        self.path = "";
        self.data = None;
        self.rule = RadixRule::default();
        self.next.clear();
    }
}

/// Create a node from a rule
///
/// ```
/// use radixmap::{node::RadixNode, rule::RadixRule, RadixResult};
///
/// fn main() -> RadixResult<()> {
///     assert_eq!(RadixNode::<'_, ()>::from(RadixRule::try_from("/api")?).rule, "/api");
///     assert_eq!(RadixNode::<'_, ()>::from(RadixRule::try_from(":id")?).rule, ":id");
///
///     Ok(())
/// }
/// ```
impl<'k, V> From<RadixRule<'k>> for RadixNode<'k, V> {
    #[inline]
    fn from(rule: RadixRule<'k>) -> Self {
        Self { path: "", data: None, rule, next: Default::default() }
    }
}

/// Create a node from (path, data)
///
/// ```
/// use radixmap::{node::RadixNode, RadixResult};
///
/// fn main() -> RadixResult<()> {
///     assert_eq!(RadixNode::try_from(("/api", ()))?.rule, "/api");
///     assert_eq!(RadixNode::try_from((":id", ()))?.rule, ":id");
///
///     Ok(())
/// }
/// ```
impl<'k, V> TryFrom<(&'k str, V)> for RadixNode<'k, V> {
    type Error = RadixError;

    #[inline]
    fn try_from((path, data): (&'k str, V)) -> RadixResult<Self> {
        Ok(Self { path, data: Some(data), rule: RadixRule::try_from(path)?, next: Default::default() })
    }
}

/// Default trait
/// ```
/// use radixmap::{node::RadixNode};
///
/// let mut node = RadixNode::default();
/// assert!(node.insert("/api", ()).is_ok());
/// ```
impl<'k, V> Default for RadixNode<'k, V> {
    #[inline]
    fn default() -> Self {
        Self { path: "", data: None, rule: RadixRule::default(), next: pack::RadixPack::default() }
    }
}

/// Debug trait
///
/// ```
/// use radixmap::{node::RadixNode, RadixResult};
///
/// fn main() -> RadixResult<()> {
///     assert_eq!(format!("{:?}", RadixNode::try_from((r"/api", ()))?).as_str(), r"Plain(/api)");
///     assert_eq!(format!("{:?}", RadixNode::try_from((r":id", ()))?).as_str(), r"Param(:id)");
///     assert_eq!(format!("{:?}", RadixNode::try_from((r"{id:\d+}", ()))?).as_str(), r"Regex({id:\d+})");
///     assert_eq!(format!("{:?}", RadixNode::try_from((r"*", ()))?).as_str(), r"Glob(*)");
///
///     Ok(())
/// }
/// ```
impl<'k, V> Debug for RadixNode<'k, V> {
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
impl<'k, V: Clone> Clone for RadixNode<'k, V> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            path: self.path,
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
pub struct Iter<'k, V> {
    queue: VecDeque<Peekable<pack::Iter<'k, V>>>,
    visit: Vec<Peekable<pack::Iter<'k, V>>>, // used in post-order only
    order: Order,
    empty: bool,
}

impl<'k, V> Iter<'k, V> {
    /// Starting to iterate from the node with a specific prefix
    ///
    /// ```
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
    ///     let mut iter = node.iter().with_prefix("/api/v1", false);
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some(("/api/v1", &"v1")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some(("/api/v1/user", &"user1")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), None);
    ///
    ///     let mut iter = node.iter().with_prefix("/api/", false); // exclude /api
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some(("/api/v1", &"v1")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some(("/api/v1/user", &"user1")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some(("/api/v2", &"v2")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some(("/api/v2/user", &"user2")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), None);
    ///
    ///     let mut iter = node.iter().with_prefix("/api/v3", false); // not exist
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), None);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn with_prefix(mut self, path: &str, data: bool) -> Self {
        let cursor = self.queue.pop_front();

        self.queue.clear();
        self.visit.clear();

        let cursor = cursor
            .and_then(|mut iter| iter.next())
            .and_then(|node| match !path.is_empty() {
                true => node.search(path, data),
                false => None,
            });

        if let Some(cursor) = cursor {
            self.queue.push_front(pack::Iter::from(cursor).peekable());
        }

        self
    }

    /// Change the iterating order
    ///
    /// ```
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
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some(("/api", &"api")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some(("/api/v1", &"v1")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some(("/api/v1/user", &"user1")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some(("/api/v2", &"v2")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some(("/api/v2/user", &"user2")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), None);
    ///
    ///     let mut iter = node.iter().with_order(Order::Post);
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some(("/api/v1/user", &"user1")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some(("/api/v1", &"v1")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some(("/api/v2/user", &"user2")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some(("/api/v2", &"v2")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some(("/api", &"api")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), None);
    ///
    ///     let mut iter = node.iter().with_order(Order::Level);
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some(("/api", &"api")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some(("/api/v1", &"v1")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some(("/api/v2", &"v2")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some(("/api/v1/user", &"user1")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some(("/api/v2/user", &"user2")));
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
    ///     verify!(iter, "", None);                        // the root node
    ///     verify!(iter, "/api", Some("api"));
    ///     verify!(iter, "/v", None);                      // an internal node
    ///     verify!(iter, "1", Some("v1"));
    ///     verify!(iter, "/user", Some("user1"));
    ///     verify!(iter, "2", Some("v2"));
    ///     verify!(iter, "/user", Some("user2"));
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
    fn next_pre(&mut self) -> Option<&'k RadixNode<'k, V>> {
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
    fn next_post(&mut self) -> Option<&'k RadixNode<'k, V>> {
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
    fn next_level(&mut self) -> Option<&'k RadixNode<'k, V>> {
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

impl<'k, V> From<&'k RadixNode<'k, V>> for Iter<'k, V> {
    #[inline]
    fn from(start: &'k RadixNode<'k, V>) -> Self {
        Self { queue: VecDeque::from([pack::Iter::from(start).peekable()]), visit: vec![], order: Order::Pre, empty: false }
    }
}

impl<'k, V> Iterator for Iter<'k, V> {
    type Item = &'k RadixNode<'k, V>;

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
pub struct IterMut<'n, 'k, V> {
    queue: VecDeque<pack::IterMut<'n, 'k, V>>,
    order: Order,
    empty: bool,
}

impl<'n, 'k, V> IterMut<'n, 'k, V> {
    /// Starting to iterate from the node with a specific prefix
    ///
    /// ```
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
    ///     let mut iter = node.iter_mut().with_prefix("/api/v1", false);
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some(("/api/v1", &"v1")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some(("/api/v1/user", &"user1")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), None);
    ///
    ///     let mut iter = node.iter_mut().with_prefix("/api/", false); // exclude /api
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some(("/api/v1", &"v1")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some(("/api/v1/user", &"user1")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some(("/api/v2", &"v2")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some(("/api/v2/user", &"user2")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), None);
    ///
    ///     let mut iter = node.iter_mut().with_prefix("/api/v3", false); // not exist
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), None);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn with_prefix(mut self, path: &str, data: bool) -> Self {
        let cursor = self.queue.pop_front();

        self.queue.clear();

        let cursor = cursor
            .and_then(|mut iter| iter.next())
            .and_then(|node| match !path.is_empty() {
                true => node.search_mut(path, data),
                false => None,
            });

        if let Some(cursor) = cursor {
            self.queue.push_front(pack::IterMut::from(cursor));
        }

        self
    }

    /// Change the iterating order
    ///
    /// ```
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
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some(("/api", &"api")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some(("/api/v1", &"v1")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some(("/api/v1/user", &"user1")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some(("/api/v2", &"v2")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some(("/api/v2/user", &"user2")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), None);
    ///
    ///     let mut iter = node.iter_mut().with_order(Order::Level);
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some(("/api", &"api")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some(("/api/v1", &"v1")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some(("/api/v2", &"v2")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some(("/api/v1/user", &"user1")));
    ///     assert_eq!(iter.next().and_then(|node| node.item_ref()), Some(("/api/v2/user", &"user2")));
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
    ///     verify!(iter, "", None);                        // the root node
    ///     verify!(iter, "/api", Some("api"));
    ///     verify!(iter, "/v", None);                      // an internal node
    ///     verify!(iter, "1", Some("v1"));
    ///     verify!(iter, "/user", Some("user1"));
    ///     verify!(iter, "2", Some("v2"));
    ///     verify!(iter, "/user", Some("user2"));
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
    fn next_pre(&mut self) -> Option<&'n mut RadixNode<'k, V>> {
        loop {
            let back = self.queue.back_mut()?;
            match back.next() {
                Some(node) => {
                    let ptr = node as *mut RadixNode<'k, V>;
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
    fn next_level(&mut self) -> Option<&'n mut RadixNode<'k, V>> {
        loop {
            let front = self.queue.front_mut()?;
            match front.next() {
                Some(node) => {
                    let ptr = node as *mut RadixNode<'k, V>;
                    self.queue.push_back(node.next.iter_mut());
                    unsafe { return Some(&mut *ptr); }
                }
                None => { self.queue.pop_front(); }
            }
        }
    }
}

impl<'n, 'k, V> From<&'n mut RadixNode<'k, V>> for IterMut<'n, 'k, V> {
    #[inline]
    fn from(start: &'n mut RadixNode<'k, V>) -> Self {
        Self { queue: VecDeque::from([pack::IterMut::from(start)]), order: Order::Pre, empty: false }
    }
}

impl<'n, 'k, V> Iterator for IterMut<'n, 'k, V> {
    type Item = &'n mut RadixNode<'k, V>;

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

impl<'k, V> From<&'k RadixNode<'k, V>> for Keys<'k, V> {
    #[inline]
    fn from(value: &'k RadixNode<'k, V>) -> Self {
        Self { iter: Iter::from(value) }
    }
}

impl<'k, V> Iterator for Keys<'k, V> {
    type Item = &'k str;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|node| node.path)
    }
}

// -----------------------------------------------------------------------------

/// Iterator adapter for data
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

impl<'k, V> From<&'k RadixNode<'k, V>> for Values<'k, V> {
    #[inline]
    fn from(value: &'k RadixNode<'k, V>) -> Self {
        Self { iter: Iter::from(value) }
    }
}

impl<'k, V> Iterator for Values<'k, V> {
    type Item = &'k V;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().and_then(|node| node.data.as_ref())
    }
}

// -----------------------------------------------------------------------------

/// Mutable iterator adapter for data
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

impl<'n, 'k, V> From<&'n mut RadixNode<'k, V>> for ValuesMut<'n, 'k, V> {
    #[inline]
    fn from(value: &'n mut RadixNode<'k, V>) -> Self {
        Self { iter: IterMut::from(value) }
    }
}

impl<'n, 'k, V> Iterator for ValuesMut<'n, 'k, V> {
    type Item = &'n mut V;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().and_then(|node| node.data.as_mut())
    }
}