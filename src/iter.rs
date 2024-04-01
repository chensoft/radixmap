use super::def::*;
use super::node::*;
use super::pack::*;

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

// -----------------------------------------------------------------------------

pub type NodeRef<'a, V> = &'a RadixNode<'a, V>;
pub type PackRef<'a, V> = &'a RadixPack<'a, V>;

pub type RegularIter<'a, V> = std::slice::Iter<'a, sparseset::Entry<RadixNode<'a, V>>>;
pub type SpecialIter<'a, V> = indexmap::map::Values<'a, &'a str, RadixNode<'a, V>>;

/// Iterator adapter for nodes and packs
#[derive(Clone)]
pub enum Entity<'a, V> {
    Node(Option<NodeRef<'a, V>>),
    Pack(RegularIter<'a, V>, SpecialIter<'a, V>),
}

impl<'a, V> From<NodeRef<'a, V>> for Entity<'a, V> {
    /// Construct from the radix node
    fn from(node: NodeRef<'a, V>) -> Self {
        Self::Node(Some(node))
    }
}

impl<'a, V> From<PackRef<'a, V>> for Entity<'a, V> {
    /// Construct from the radix pack
    fn from(pack: PackRef<'a, V>) -> Self {
        Self::Pack(pack.regular.iter(), pack.special.values())
    }
}

impl<'a, V> Iterator for Entity<'a, V> {
    type Item = NodeRef<'a, V>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Entity::Node(node) => node.take(),
            Entity::Pack(regular, special) => {
                if let Some(node) = regular.next() {
                    return Some(node.value());
                }

                if let Some(node) = special.next() {
                    return Some(node);
                }

                None
            }
        }
    }
}

// -----------------------------------------------------------------------------

/// The iterator for radix tree
#[derive(Clone)]
pub struct Iter<'a, V> {
    start: NodeRef<'a, V>,
    queue: VecDeque<Peekable<Entity<'a, V>>>,
    visit: Vec<Peekable<Entity<'a, V>>>, // used in post-order only
    order: Order,
    empty: bool,
}

impl<'a, V> Iter<'a, V> {
    /// Creating a new iterator that visits nodes in pre-order by default
    ///
    /// ```
    /// use radixmap::{RadixMap};
    ///
    /// fn main() -> anyhow::Result<()> {
    ///     let mut map = RadixMap::new();
    ///     map.insert("/api", "/api")?;
    ///     map.insert("/api/v1", "/api/v1")?;
    ///
    ///     let mut iter = map.iter();
    ///     assert_eq!(iter.next().unwrap().data_ref(), Some(&"/api"));
    ///     assert_eq!(iter.next().unwrap().data_ref(), Some(&"/api/v1"));
    ///     assert!(iter.next().is_none());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn new(start: NodeRef<'a, V>) -> Self {
        Self { start, queue: VecDeque::from([Entity::from(start).peekable()]), visit: vec![], order: Order::Pre, empty: false }
    }

    /// Starting to iterate from the node with a specific prefix
    ///
    /// ```
    /// use radixmap::{RadixMap};
    ///
    /// fn main() -> anyhow::Result<()> {
    ///     let mut map = RadixMap::new();
    ///     map.insert("/api", "/api")?;
    ///     map.insert("/api/v1", "/api/v1")?;
    ///     map.insert("/api/v1/user1", "/api/v1/user1")?;
    ///     map.insert("/api/v2", "/api/v2")?;
    ///     map.insert("/api/v2/user2", "/api/v2/user2")?;
    ///
    ///     let mut iter = map.iter().with_prefix("/api/v1");
    ///     assert_eq!(iter.next().unwrap().data_ref(), Some(&"/api/v1"));
    ///     assert_eq!(iter.next().unwrap().data_ref(), Some(&"/api/v1/user1"));
    ///     assert!(iter.next().is_none());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn with_prefix(mut self, _prefix: &'a str) -> Self {
        // todo
        // if let Some(start) = self.start {
        //     self.start = start.deepest(prefix);
        // }

        self.queue.clear();
        // self.queue.push_back(Entity::Single(Some(start)));
        self.visit.clear();
        self
    }

    /// Change the iterating order
    ///
    /// ```
    /// use radixmap::{RadixMap, iter::Order};
    ///
    /// fn main() -> anyhow::Result<()> {
    ///     let mut map = RadixMap::new();
    ///     map.insert("/api", "/api")?;
    ///     map.insert("/api/v1", "/api/v1")?;
    ///     map.insert("/api/v1/user1", "/api/v1/user1")?;
    ///     map.insert("/api/v2", "/api/v2")?;
    ///     map.insert("/api/v2/user2", "/api/v2/user2")?;
    ///
    ///     let mut iter = map.iter(); // same as with_order(Order::Pre);
    ///     assert_eq!(iter.next().unwrap().data_ref(), Some(&"/api"));
    ///     assert_eq!(iter.next().unwrap().data_ref(), Some(&"/api/v1"));
    ///     assert_eq!(iter.next().unwrap().data_ref(), Some(&"/api/v1/user1"));
    ///     assert_eq!(iter.next().unwrap().data_ref(), Some(&"/api/v2"));
    ///     assert_eq!(iter.next().unwrap().data_ref(), Some(&"/api/v2/user2"));
    ///     assert!(iter.next().is_none());
    ///
    ///     let mut iter = map.iter().with_order(Order::Post);
    ///     assert_eq!(iter.next().unwrap().data_ref(), Some(&"/api/v1/user1"));
    ///     assert_eq!(iter.next().unwrap().data_ref(), Some(&"/api/v1"));
    ///     assert_eq!(iter.next().unwrap().data_ref(), Some(&"/api/v2/user2"));
    ///     assert_eq!(iter.next().unwrap().data_ref(), Some(&"/api/v2"));
    ///     assert_eq!(iter.next().unwrap().data_ref(), Some(&"/api"));
    ///     assert!(iter.next().is_none());
    ///
    ///     let mut iter = map.iter().with_order(Order::Level);
    ///     assert_eq!(iter.next().unwrap().data_ref(), Some(&"/api"));
    ///     assert_eq!(iter.next().unwrap().data_ref(), Some(&"/api/v1"));
    ///     assert_eq!(iter.next().unwrap().data_ref(), Some(&"/api/v2"));
    ///     assert_eq!(iter.next().unwrap().data_ref(), Some(&"/api/v1/user1"));
    ///     assert_eq!(iter.next().unwrap().data_ref(), Some(&"/api/v2/user2"));
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
    /// use radixmap::{RadixMap};
    ///
    /// macro_rules! check {
    ///     ($iter:expr, $orig:literal, $data:expr) => {{
    ///         let node = $iter.next().unwrap();
    ///         assert_eq!(node.rule_ref().origin(), $orig);
    ///         assert_eq!(node.data_ref(), $data);
    ///     }};
    /// }
    ///
    /// fn main() -> anyhow::Result<()> {
    ///     let mut map = RadixMap::new();
    ///     map.insert("/api", "/api")?;
    ///     map.insert("/api/v1", "/api/v1")?;
    ///     map.insert("/api/v1/user1", "/api/v1/user1")?;
    ///     map.insert("/api/v2", "/api/v2")?;
    ///     map.insert("/api/v2/user2", "/api/v2/user2")?;
    ///
    ///     let mut iter = map.iter().with_empty();
    ///     check!(iter, "", None);                        // the root node
    ///     check!(iter, "/api", Some(&"/api"));
    ///     check!(iter, "/v", None);                      // an internal node
    ///     check!(iter, "1", Some(&"/api/v1"));
    ///     check!(iter, "/user1", Some(&"/api/v1/user1"));
    ///     check!(iter, "2", Some(&"/api/v2"));
    ///     check!(iter, "/user2", Some(&"/api/v2/user2"));
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
    fn next_pre(&mut self) -> Option<NodeRef<'a, V>> {
        loop {
            let back = match self.queue.back_mut() {
                Some(obj) => obj,
                None => return None,
            };

            match back.next() {
                Some(node) => {
                    self.queue.push_back(Entity::from(node.next_ref()).peekable());
                    return Some(node);
                }
                None => { self.queue.pop_back(); }
            }
        }
    }

    /// Internal use only, traversing nodes in post-order
    fn next_post(&mut self) -> Option<NodeRef<'a, V>> {
        // traverse to the deepest leaf node, put all iters into the visit queue
        if let Some(mut back) = self.queue.pop_back() {
            while let Some(node) = back.peek() {
                let pack = Entity::from(node.next_ref()).peekable();
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
    fn next_level(&mut self) -> Option<NodeRef<'a, V>> {
        loop {
            let front = match self.queue.front_mut() {
                Some(obj) => obj,
                None => return None,
            };

            match front.next() {
                Some(node) => {
                    self.queue.push_back(Entity::from(node.next_ref()).peekable());
                    return Some(node);
                }
                None => { self.queue.pop_front(); }
            }
        }
    }
}

impl<'a, V> Iterator for Iter<'a, V> {
    type Item = NodeRef<'a, V>;

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