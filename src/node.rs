//! Node is the core tree element
use super::pack;
use super::def::*;
use super::rule::*;

/// The basic element inside a tree
pub struct RadixNode<'a, V> {
    /// The key of the radix map, valid only in a leaf node
    pub path: &'a str,

    /// The value of the radix map, valid only in a leaf node
    pub data: Option<V>,

    /// The pattern used for matching, supports named params, regex and glob
    pub rule: RadixRule<'a>,

    /// Node's children
    pub next: pack::RadixPack<'a, V>,
}

impl<'a, V> RadixNode<'a, V> {
    /// Check if the node is a leaf node
    #[inline]
    pub fn is_leaf(&self) -> bool {
        self.next.is_empty()
    }

    /// Check if the node has no data
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_none()
    }

    /// An iterator for node
    ///
    /// ```
    /// use radixmap::{node::RadixNode, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut node = RadixNode::default();
    ///     node.insert("/api", "/api")?;
    ///     node.insert("/api/v1", "/api/v1")?;
    ///     node.insert("/api/v2", "/api/v2")?;
    ///
    ///     let mut iter = node.iter();
    ///
    ///     assert_eq!(iter.next().unwrap().path, "/api");
    ///     assert_eq!(iter.next().unwrap().path, "/api/v1");
    ///     assert_eq!(iter.next().unwrap().path, "/api/v1");
    ///     assert_eq!(iter.next(), None);
    ///
    ///     Ok(())
    /// }
    /// ```
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
    ///     let mut iter = node.iter_mut();
    ///
    ///     assert_eq!(iter.next().unwrap().path, "/api");
    ///     assert_eq!(iter.next().unwrap().path, "/api/v1");
    ///     assert_eq!(iter.next().unwrap().path, "/api/v1");
    ///     assert_eq!(iter.next(), None);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn iter_mut(&'a mut self) -> IterMut<'_, V> {
        IterMut::from(self)
    }

    /// Iterator adapter for path
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
    ///     let mut iter = node.keys();
    ///
    ///     assert_eq!(iter.next().unwrap(), "/api");
    ///     assert_eq!(iter.next().unwrap(), "/api/v1");
    ///     assert_eq!(iter.next().unwrap(), "/api/v1");
    ///     assert_eq!(iter.next(), None);
    ///
    ///     Ok(())
    /// }
    /// ```
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
    ///     assert_eq!(iter.next().unwrap(), 0);
    ///     assert_eq!(iter.next().unwrap(), 1);
    ///     assert_eq!(iter.next().unwrap(), 2);
    ///     assert_eq!(iter.next(), None);
    ///
    ///     Ok(())
    /// }
    /// ```
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
    ///     let mut iter = node.values_mut();
    ///
    ///     assert_eq!(iter.next().unwrap(), 0);
    ///     assert_eq!(iter.next().unwrap(), 1);
    ///     assert_eq!(iter.next().unwrap(), 2);
    ///     assert_eq!(iter.next(), None);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn values_mut(&'a mut self) -> ValuesMut<'_, V> {
        ValuesMut::from(self)
    }

    /// Inserts a path and data into this node, which serves as the root node for the insertion.
    /// The method sequentially extracts path fragments and positions each node appropriately,
    /// ensuring that nodes with a common prefix share a single node in the tree
    pub fn insert(&mut self, path: &'a str, data: V) -> RadixResult<Option<V>> {
        let mut frag = path;

        loop {
            // extract the next path fragment and insert it via pack
            let next = RadixRule::try_from(frag)?;
            let used = next.origin();
            let slot = self.next.insert(next)?;

            // encountering a leaf node indicates completion of insertion
            if used.len() == frag.len() {
                let prev = slot.data.take();
                slot.path = path;
                slot.data = Some(data);
                return Ok(prev);
            }

            frag = &frag[used.len()..];
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
    ///     let leaf = node.divide(1)?;
    ///
    ///     assert_eq!(node.rule, "/");
    ///     assert_eq!(node.data, None);
    ///     assert_eq!(leaf.rule, "api");
    ///     assert_eq!(leaf.data, Some(12345));
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn divide(&mut self, len: usize) -> RadixResult<RadixNode<'a, V>> {
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
    ///     assert!(!node.is_leaf());
    ///     assert!(!node.is_empty());
    ///
    ///     node.clear();
    ///
    ///     assert!(node.is_leaf());
    ///     assert!(node.is_empty());
    ///
    ///     Ok(())
    /// }
    /// ```
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
impl<'a, V> From<RadixRule<'a>> for RadixNode<'a, V> {
    fn from(rule: RadixRule<'a>) -> Self {
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
impl<'a, V> TryFrom<(&'a str, V)> for RadixNode<'a, V> {
    type Error = RadixError;

    fn try_from((path, data): (&'a str, V)) -> RadixResult<Self> {
        Ok(Self { path, data: Some(data), rule: RadixRule::try_from(path)?, next: Default::default() })
    }
}

/// Default trait
/// ```
/// use radixmap::{node::RadixNode, RadixResult};
///
/// fn main() -> RadixResult<()> {
///     let mut node = RadixNode::default();
///     assert!(node.insert("/api", ()).is_ok());
///
///     Ok(())
/// }
/// ```
impl<'a, V> Default for RadixNode<'a, V> {
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
impl<'a, V> Debug for RadixNode<'a, V> {
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
impl<'a, V: Clone> Clone for RadixNode<'a, V> {
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
    Post,

    /// Level-order traversal: 1a -> 2a -> 2b -> 3a
    Level
}

impl Default for Order {
    fn default() -> Self {
        Self::Pre
    }
}

// -----------------------------------------------------------------------------

/// The iterator for radix tree
#[derive(Default, Clone)]
pub struct Iter<'a, V> {
    queue: VecDeque<Peekable<pack::Iter<'a, V>>>,
    visit: Vec<Peekable<pack::Iter<'a, V>>>, // used in post-order only
    order: Order,
    empty: bool,
}

impl<'a, V> Iter<'a, V> {
    /// Starting to iterate from the node with a specific prefix
    ///
    /// ```
    /// use radixmap::{RadixMap, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut map = RadixMap::new();
    ///     map.insert("/api", "/api")?;
    ///     map.insert("/api/v1", "/api/v1")?;
    ///     map.insert("/api/v1/user1", "/api/v1/user1")?;
    ///     map.insert("/api/v2", "/api/v2")?;
    ///     map.insert("/api/v2/user2", "/api/v2/user2")?;
    ///
    ///     let mut iter = map.iter().with_prefix("/api/v1")?;
    ///     assert_eq!(iter.next().unwrap().data, Some("/api/v1"));
    ///     assert_eq!(iter.next().unwrap().data, Some("/api/v1/user1"));
    ///     assert_eq!(iter.next(), None);
    ///
    ///     let mut iter = map.iter().with_prefix("/api/v")?;
    ///     assert_eq!(iter.next().unwrap().data, Some("/api/v1"));
    ///     assert_eq!(iter.next().unwrap().data, Some("/api/v1/user1"));
    ///     assert_eq!(iter.next().unwrap().data, Some("/api/v2"));
    ///     assert_eq!(iter.next().unwrap().data, Some("/api/v2/user2"));
    ///     assert_eq!(iter.next(), None);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn with_prefix(self, _prefix: &str) -> RadixResult<Self> {
        // todo iterate self and then rewind
        // let start = match self.start.deepest(prefix) {
        //     Some(node) => node,
        //     None => return Err(RadixError::PathNotFound),
        // };
        // 
        // self.start = start;
        // self.queue.clear();
        // self.queue.push_back(EntityRef::from(self.start).peekable());
        // self.visit.clear();

        Ok(self)
    }

    /// Change the iterating order
    ///
    /// ```
    /// use radixmap::{RadixMap, RadixResult, node::Order};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut map = RadixMap::new();
    ///     map.insert("/api", "/api")?;
    ///     map.insert("/api/v1", "/api/v1")?;
    ///     map.insert("/api/v1/user1", "/api/v1/user1")?;
    ///     map.insert("/api/v2", "/api/v2")?;
    ///     map.insert("/api/v2/user2", "/api/v2/user2")?;
    ///
    ///     let mut iter = map.iter(); // same as with_order(Order::Pre);
    ///     assert_eq!(iter.next().unwrap().data, Some("/api"));
    ///     assert_eq!(iter.next().unwrap().data, Some("/api/v1"));
    ///     assert_eq!(iter.next().unwrap().data, Some("/api/v1/user1"));
    ///     assert_eq!(iter.next().unwrap().data, Some("/api/v2"));
    ///     assert_eq!(iter.next().unwrap().data, Some("/api/v2/user2"));
    ///     assert!(iter.next().is_none());
    ///
    ///     let mut iter = map.iter().with_order(Order::Post);
    ///     assert_eq!(iter.next().unwrap().data, Some("/api/v1/user1"));
    ///     assert_eq!(iter.next().unwrap().data, Some("/api/v1"));
    ///     assert_eq!(iter.next().unwrap().data, Some("/api/v2/user2"));
    ///     assert_eq!(iter.next().unwrap().data, Some("/api/v2"));
    ///     assert_eq!(iter.next().unwrap().data, Some("/api"));
    ///     assert!(iter.next().is_none());
    ///
    ///     let mut iter = map.iter().with_order(Order::Level);
    ///     assert_eq!(iter.next().unwrap().data, Some("/api"));
    ///     assert_eq!(iter.next().unwrap().data, Some("/api/v1"));
    ///     assert_eq!(iter.next().unwrap().data, Some("/api/v2"));
    ///     assert_eq!(iter.next().unwrap().data, Some("/api/v1/user1"));
    ///     assert_eq!(iter.next().unwrap().data, Some("/api/v2/user2"));
    ///     assert!(iter.next().is_none());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn with_order(mut self, order: Order) -> Self {
        self.order = order;
        self
    }

    /// Traverse all nodes, including the internal nodes which do not contain data
    ///
    /// ```
    /// use radixmap::{RadixMap, RadixResult};
    ///
    /// macro_rules! check {
    ///     ($iter:expr, $orig:literal, $data:expr) => {{
    ///         let node = $iter.next().unwrap();
    ///         assert_eq!(node.rule.origin(), $orig);
    ///         assert_eq!(node.data, $data);
    ///     }};
    /// }
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut map = RadixMap::new();
    ///     map.insert("/api", "/api")?;
    ///     map.insert("/api/v1", "/api/v1")?;
    ///     map.insert("/api/v1/user1", "/api/v1/user1")?;
    ///     map.insert("/api/v2", "/api/v2")?;
    ///     map.insert("/api/v2/user2", "/api/v2/user2")?;
    ///
    ///     let mut iter = map.iter().with_empty();
    ///     check!(iter, "", None);                        // the root node
    ///     check!(iter, "/api", Some("/api"));
    ///     check!(iter, "/v", None);                      // an internal node
    ///     check!(iter, "1", Some("/api/v1"));
    ///     check!(iter, "/user1", Some("/api/v1/user1"));
    ///     check!(iter, "2", Some("/api/v2"));
    ///     check!(iter, "/user2", Some("/api/v2/user2"));
    ///     assert!(iter.next().is_none());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn with_empty(mut self) -> Self {
        self.empty = true;
        self
    }

    /// Internal use only, traversing nodes in pre-order
    fn next_pre(&mut self) -> Option<&'a RadixNode<'a, V>> {
        loop {
            let back = match self.queue.back_mut() {
                Some(obj) => obj,
                None => return None,
            };

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
    fn next_post(&mut self) -> Option<&'a RadixNode<'a, V>> {
        // traverse to the deepest leaf node, put all iters into the visit queue
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
            let mut back = match self.visit.pop() {
                Some(obj) => obj,
                None => return None,
            };

            if let Some(node) = back.next() {
                if back.peek().is_some() {
                    self.queue.push_back(back);
                }

                return Some(node);
            }
        }
    }

    /// Internal use only, traversing nodes in level-order
    fn next_level(&mut self) -> Option<&'a RadixNode<'a, V>> {
        loop {
            let front = match self.queue.front_mut() {
                Some(obj) => obj,
                None => return None,
            };

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

/// Creating a new iterator that visits nodes in pre-order by default
///
/// ```
/// use radixmap::{RadixMap, RadixResult};
///
/// fn main() -> RadixResult<()> {
///     let mut map = RadixMap::new();
///     map.insert("/api", "/api")?;
///     map.insert("/api/v1", "/api/v1")?;
///
///     let mut iter = map.iter();
///     assert_eq!(iter.next().unwrap().data, Some("/api"));
///     assert_eq!(iter.next().unwrap().data, Some("/api/v1"));
///     assert!(iter.next().is_none());
///
///     Ok(())
/// }
/// ```
impl<'a, V> From<&'a RadixNode<'a, V>> for Iter<'a, V> {
    fn from(start: &'a RadixNode<'a, V>) -> Self {
        Self { queue: VecDeque::from([pack::Iter::from(start).peekable()]), visit: vec![], order: Order::Pre, empty: false }
    }
}

impl<'a, V> Iterator for Iter<'a, V> {
    type Item = &'a RadixNode<'a, V>;

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
pub struct IterMut<'a, V> {
    queue: VecDeque<Peekable<pack::IterMut<'a, V>>>,
    order: Order,
    empty: bool,
}

impl<'a, V> IterMut<'a, V> {
    /// Starting to iterate from the node with a specific prefix
    ///
    /// ```
    /// use radixmap::{RadixMap, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut map = RadixMap::new();
    ///     map.insert("/api", "/api")?;
    ///     map.insert("/api/v1", "/api/v1")?;
    ///     map.insert("/api/v1/user1", "/api/v1/user1")?;
    ///     map.insert("/api/v2", "/api/v2")?;
    ///     map.insert("/api/v2/user2", "/api/v2/user2")?;
    ///
    ///     let mut iter = map.iter_mut().with_prefix("/api/v1")?;
    ///     assert_eq!(iter.next().unwrap().data, Some("/api/v1"));
    ///     assert_eq!(iter.next().unwrap().data, Some("/api/v1/user1"));
    ///     assert_eq!(iter.next(), None);
    ///
    ///     let mut iter = map.iter_mut().with_prefix("/api/v")?;
    ///     assert_eq!(iter.next().unwrap().data, Some("/api/v1"));
    ///     assert_eq!(iter.next().unwrap().data, Some("/api/v1/user1"));
    ///     assert_eq!(iter.next().unwrap().data, Some("/api/v2"));
    ///     assert_eq!(iter.next().unwrap().data, Some("/api/v2/user2"));
    ///     assert_eq!(iter.next(), None);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn with_prefix(self, _prefix: &str) -> RadixResult<Self> {
        // todo iterate self and then rewind
        // let start = match self.start.deepest(prefix) {
        //     Some(node) => node,
        //     None => return Err(RadixError::PathNotFound),
        // };
        // 
        // self.start = start;
        // self.queue.clear();
        // self.queue.push_back(EntityMut::from(self.start).peekable());
        // self.visit.clear();

        Ok(self)
    }

    /// Change the iterating order
    ///
    /// ```
    /// use radixmap::{RadixMap, RadixResult, node::Order};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut map = RadixMap::new();
    ///     map.insert("/api", "/api")?;
    ///     map.insert("/api/v1", "/api/v1")?;
    ///     map.insert("/api/v1/user1", "/api/v1/user1")?;
    ///     map.insert("/api/v2", "/api/v2")?;
    ///     map.insert("/api/v2/user2", "/api/v2/user2")?;
    ///
    ///     let mut iter = map.iter_mut(); // same as with_order(Order::Pre);
    ///     assert_eq!(iter.next().unwrap().data, Some("/api"));
    ///     assert_eq!(iter.next().unwrap().data, Some("/api/v1"));
    ///     assert_eq!(iter.next().unwrap().data, Some("/api/v1/user1"));
    ///     assert_eq!(iter.next().unwrap().data, Some("/api/v2"));
    ///     assert_eq!(iter.next().unwrap().data, Some("/api/v2/user2"));
    ///     assert!(iter.next().is_none());
    ///
    ///     let mut iter = map.iter_mut().with_order(Order::Post);
    ///     assert_eq!(iter.next().unwrap().data, Some("/api/v1/user1"));
    ///     assert_eq!(iter.next().unwrap().data, Some("/api/v1"));
    ///     assert_eq!(iter.next().unwrap().data, Some("/api/v2/user2"));
    ///     assert_eq!(iter.next().unwrap().data, Some("/api/v2"));
    ///     assert_eq!(iter.next().unwrap().data, Some("/api"));
    ///     assert!(iter.next().is_none());
    ///
    ///     let mut iter = map.iter_mut().with_order(Order::Level);
    ///     assert_eq!(iter.next().unwrap().data, Some("/api"));
    ///     assert_eq!(iter.next().unwrap().data, Some("/api/v1"));
    ///     assert_eq!(iter.next().unwrap().data, Some("/api/v2"));
    ///     assert_eq!(iter.next().unwrap().data, Some("/api/v1/user1"));
    ///     assert_eq!(iter.next().unwrap().data, Some("/api/v2/user2"));
    ///     assert!(iter.next().is_none());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn with_order(mut self, order: Order) -> Self {
        self.order = order;
        self
    }

    /// Traverse all nodes, including the internal nodes which do not contain data
    ///
    /// ```
    /// use radixmap::{RadixMap, RadixResult};
    ///
    /// macro_rules! check {
    ///     ($iter:expr, $orig:literal, $data:expr) => {{
    ///         let node = $iter.next().unwrap();
    ///         assert_eq!(node.rule.origin(), $orig);
    ///         assert_eq!(node.data, $data);
    ///     }};
    /// }
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut map = RadixMap::new();
    ///     map.insert("/api", "/api")?;
    ///     map.insert("/api/v1", "/api/v1")?;
    ///     map.insert("/api/v1/user1", "/api/v1/user1")?;
    ///     map.insert("/api/v2", "/api/v2")?;
    ///     map.insert("/api/v2/user2", "/api/v2/user2")?;
    ///
    ///     let mut iter = map.iter_mut().with_empty();
    ///     check!(iter, "", None);                        // the root node
    ///     check!(iter, "/api", Some("/api"));
    ///     check!(iter, "/v", None);                      // an internal node
    ///     check!(iter, "1", Some("/api/v1"));
    ///     check!(iter, "/user1", Some("/api/v1/user1"));
    ///     check!(iter, "2", Some("/api/v2"));
    ///     check!(iter, "/user2", Some("/api/v2/user2"));
    ///     assert!(iter.next().is_none());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn with_empty(mut self) -> Self {
        self.empty = true;
        self
    }

    /// Internal use only, traversing nodes in pre-order
    ///
    /// # Safety
    ///
    /// DO NOT MODIFY THE RETURNED NODE'S `next` FIELD
    fn next_pre(&mut self) -> Option<&'a mut RadixNode<'a, V>> {
        loop {
            let back = match self.queue.back_mut() {
                Some(obj) => obj,
                None => return None,
            };

            match back.next() {
                Some(node) => {
                    let ptr = node as *mut RadixNode<'a, V>;
                    self.queue.push_back(node.next.iter_mut().peekable());
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
    fn next_level(&mut self) -> Option<&'a mut RadixNode<'a, V>> {
        loop {
            let front = match self.queue.front_mut() {
                Some(obj) => obj,
                None => return None,
            };

            match front.next() {
                Some(node) => {
                    let ptr = node as *mut RadixNode<'a, V>;
                    self.queue.push_back(node.next.iter_mut().peekable());
                    unsafe { return Some(&mut *ptr); }
                }
                None => { self.queue.pop_front(); }
            }
        }
    }
}

/// Creating a new iterator that visits nodes in pre-order by default
///
/// ```
/// use radixmap::{RadixMap, RadixResult};
///
/// fn main() -> RadixResult<()> {
///     let mut map = RadixMap::new();
///     map.insert("/api", "/api")?;
///     map.insert("/api/v1", "/api/v1")?;
///
///     let mut iter = map.iter_mut();
///     assert_eq!(iter.next().unwrap().data, Some("/api"));
///     assert_eq!(iter.next().unwrap().data, Some("/api/v1"));
///     assert!(iter.next().is_none());
///
///     Ok(())
/// }
/// ```
impl<'a, V> From<&'a mut RadixNode<'a, V>> for IterMut<'a, V> {
    fn from(start: &'a mut RadixNode<'a, V>) -> Self {
        Self { queue: VecDeque::from([pack::IterMut::from(start).peekable()]), order: Order::Pre, empty: false }
    }
}

impl<'a, V> Iterator for IterMut<'a, V> {
    type Item = &'a mut RadixNode<'a, V>;

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

impl<'a, V> From<&'a RadixNode<'a, V>> for Keys<'a, V> {
    fn from(value: &'a RadixNode<'a, V>) -> Self {
        Self { iter: Iter::from(value) }
    }
}

impl<'a, V> Iterator for Keys<'a, V> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|node| node.path)
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

impl<'a, V> From<&'a RadixNode<'a, V>> for Values<'a, V> {
    fn from(value: &'a RadixNode<'a, V>) -> Self {
        Self { iter: Iter::from(value) }
    }
}

impl<'a, V> Iterator for Values<'a, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().and_then(|node| node.data.as_ref())
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

impl<'a, V> From<&'a mut RadixNode<'a, V>> for ValuesMut<'a, V> {
    fn from(value: &'a mut RadixNode<'a, V>) -> Self {
        Self { iter: IterMut::from(value) }
    }
}

impl<'a, V> Iterator for ValuesMut<'a, V> {
    type Item = &'a mut V;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().and_then(|node| node.data.as_mut())
    }
}