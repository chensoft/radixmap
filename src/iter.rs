use super::def::*;
use super::node::*;
use super::pack::*;

// -----------------------------------------------------------------------------

/// Iterator adapter for nodes and packs
#[derive(Clone)]
pub enum State<'a, V> {
    Node(Option<&'a RadixNode<'a, V>>),
    Pack(std::slice::Iter<'a, sparseset::Entry<RadixNode<'a, V>>>, indexmap::map::Values<'a, &'a str, RadixNode<'a, V>>),
}

impl<'a, V> State<'a, V> {
    /// Construct from radix node
    pub fn from_node(node: &'a RadixNode<'a, V>) -> Peekable<Self> {
        Self::Node(Some(node)).peekable()
    }

    /// Construct from radix pack
    pub fn from_pack(pack: &'a RadixPack<'a, V>) -> Peekable<Self> {
        Self::Pack(pack.regular.iter(), pack.special.values()).peekable()
    }
}

impl<'a, V> Iterator for State<'a, V> {
    type Item = &'a RadixNode<'a, V>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            State::Node(single) => single.take(),
            State::Pack(regular, special) => {
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

/// The iterator for radix tree
#[derive(Clone)]
pub struct Iter<'a, V> {
    start: &'a RadixNode<'a, V>,
    queue: VecDeque<Peekable<State<'a, V>>>,
    visit: Vec<Peekable<State<'a, V>>>, // used in post-order only
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
    ///     assert_eq!(iter.next().unwrap().data, Some("/api"));
    ///     assert_eq!(iter.next().unwrap().data, Some("/api/v1"));
    ///     assert!(iter.next().is_none());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn new(start: &'a RadixNode<'a, V>) -> Self {
        Self { start, queue: VecDeque::from([State::from_node(start)]), visit: vec![], order: Order::Pre, empty: false }
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
    ///     assert_eq!(iter.next().unwrap().data, Some("/api/v1"));
    ///     assert_eq!(iter.next().unwrap().data, Some("/api/v1/user1"));
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
        // self.queue.push_back(State::Single(Some(start)));
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

    /// Traverse all nodes, including the edge nodes which do not contain data
    ///
    /// ```
    /// use radixmap::{RadixMap};
    ///
    /// macro_rules! check {
    ///     ($iter:expr, $orig:literal, $data:expr) => {{
    ///         let node = $iter.next().unwrap();
    ///         assert_eq!(node.item.origin(), $orig);
    ///         assert_eq!(node.data, $data);
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
    ///     check!(iter, "/api", Some("/api"));
    ///     check!(iter, "/v", None);                      // an edge node
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
                    self.queue.push_back(State::from_pack(&node.next));
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
                let pack = State::from_pack(&node.next);
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
                    self.queue.push_back(State::from_pack(&node.next));
                    return Some(node);
                }
                None => { self.queue.pop_front(); }
            }
        }
    }
}

impl<'a, V> Iterator for Iter<'a, V> {
    type Item = &'a RadixNode<'a, V>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = match self.order {
            Order::Pre => self.next_pre(),
            Order::Post => self.next_post(),
            Order::Level => self.next_level(),
        };

        // check if user need to traverse empty node
        match node {
            Some(node) if !self.empty && node.is_empty() => self.next(),
            _ => node,
        }
    }
}

// -----------------------------------------------------------------------------

// pub struct Keys<'a, V> {}

// -----------------------------------------------------------------------------

/// Traverse the tree to retrieve all data
#[derive(Clone)]
pub struct Values<'a, V> {
    iter: Iter<'a, V>
}

impl<'a, V> Values<'a, V> {
    /// Construct a new iterator
    pub fn new(start: &'a RadixNode<'a, V>) -> Self {
        Self { iter: Iter::new(start) }
    }

    /// Construct with a order
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
    ///     let mut iter = map.values(); // same as with_order(Order::Pre);
    ///     assert_eq!(iter.next(), Some(&"/api"));
    ///     assert_eq!(iter.next(), Some(&"/api/v1"));
    ///     assert_eq!(iter.next(), Some(&"/api/v1/user1"));
    ///     assert_eq!(iter.next(), Some(&"/api/v2"));
    ///     assert_eq!(iter.next(), Some(&"/api/v2/user2"));
    ///     assert_eq!(iter.next(), None);
    ///
    ///     let mut iter = map.values().with_order(Order::Post);
    ///     assert_eq!(iter.next(), Some(&"/api/v1/user1"));
    ///     assert_eq!(iter.next(), Some(&"/api/v1"));
    ///     assert_eq!(iter.next(), Some(&"/api/v2/user2"));
    ///     assert_eq!(iter.next(), Some(&"/api/v2"));
    ///     assert_eq!(iter.next(), Some(&"/api"));
    ///     assert_eq!(iter.next(), None);
    ///
    ///     let mut iter = map.values().with_order(Order::Level);
    ///     assert_eq!(iter.next(), Some(&"/api"));
    ///     assert_eq!(iter.next(), Some(&"/api/v1"));
    ///     assert_eq!(iter.next(), Some(&"/api/v2"));
    ///     assert_eq!(iter.next(), Some(&"/api/v1/user1"));
    ///     assert_eq!(iter.next(), Some(&"/api/v2/user2"));
    ///     assert_eq!(iter.next(), None);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn with_order(mut self, order: Order) -> Self {
        self.iter = self.iter.with_order(order);
        self
    }
}

impl<'a, V> Iterator for Values<'a, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(node) => match &node.data {
                Some(data) => Some(data),
                None => self.next() // impossible
            }
            None => None
        }
    }
}