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

/// Iterator adapter for nodes and packs
#[derive(Clone)]
pub struct State<N, P: Iterator<Item = N>> {
    peek: Option<N>,
    node: Option<N>,
    pack: P,
}

impl<N, P: Iterator<Item = N> + Default> State<N, P> {
    /// Construct from the radix node
    pub fn from_node(node: N) -> Self {
        Self { peek: None, node: Some(node), pack: P::default() }
    }
}

impl<N, P: Iterator<Item = N>> State<N, P> {
    /// Construct from the radix pack
    pub fn from_pack(pack: P) -> Self {
        Self { peek: None, node: None, pack }
    }
}

impl<N: Clone, P: Iterator<Item = N>> State<N, P> {
    /// Peek the next item and retain it
    pub fn peek(&mut self) -> Option<N> {
        if self.peek.is_none() {
            self.peek = self.next();
        }

        self.peek.clone()
    }
}

impl<N, P: Iterator<Item = N>> Iterator for State<N, P> {
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.peek.take() {
            return Some(node);
        }

        if let Some(node) = self.node.take() {
            return Some(node);
        }

        self.pack.next()
    }
}

// -----------------------------------------------------------------------------

pub trait Bridge<N> {
    fn children(node: N) -> impl Iterator<Item = N>;
}

impl<'a, V> Bridge<&'a RadixNode<'a, V>> for &'a RadixNode<'a, V> {
    fn children(node: &'a RadixNode<'a, V>) -> impl Iterator<Item = &'a RadixNode<'a, V>> {
        node.next_ref().iter()
    }
}

impl<'a, V> Bridge<&'a mut RadixNode<'a, V>> for &'a mut RadixNode<'a, V> {
    fn children(node: &'a mut RadixNode<'a, V>) -> impl Iterator<Item = &'a mut RadixNode<'a, V>> {
        node.next_mut().iter_mut()
    }
}

// -----------------------------------------------------------------------------

/// The iterator for radix tree
#[derive(Clone)]
pub struct Base<N, P: Iterator<Item = N>> {
    start: N,
    queue: VecDeque<P>,
    visit: Vec<P>, // used in post-order only
    order: Order,
    empty: bool,
}

impl<'a, N: Clone, P: Iterator<Item = N> + Default> Base<N, State<N, P>> {
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
    pub fn new(start: N) -> Self {
        Self { start: start.clone(), queue: VecDeque::from([State::from_node(start)]), visit: vec![], order: Order::Pre, empty: false }
    }
}

impl<'a, N, P: Iterator<Item = N> + Default> Base<N, State<N, P>> {
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
    fn next_pre(&mut self) -> Option<N> {
        // loop {
        //     let back = match self.queue.back_mut() {
        //         Some(obj) => obj,
        //         None => return None,
        //     };
        // 
        //     match back.next() {
        //         Some(node) => {
        //             self.queue.push_back(State::from_pack(node.next_ref()));
        //             return Some(node);
        //         }
        //         None => { self.queue.pop_back(); }
        //     }
        // }
        todo!()
    }

    /// Internal use only, traversing nodes in post-order
    fn next_post(&mut self) -> Option<N> {
        // // traverse to the deepest leaf node, put all iters into the visit queue
        // if let Some(mut back) = self.queue.pop_back() {
        //     while let Some(node) = back.peek() {
        //         let pack = State::from_pack(node.next_ref());
        //         self.visit.push(back);
        //         back = pack;
        //     }
        // 
        //     return self.next_post();
        // }
        // 
        // // pop node from visit queue, re-push iter if the next node is not empty
        // loop {
        //     let mut back = match self.visit.pop() {
        //         Some(obj) => obj,
        //         None => return None,
        //     };
        // 
        //     if let Some(node) = back.next() {
        //         if back.peek().is_some() {
        //             self.queue.push_back(back);
        //         }
        // 
        //         return Some(node);
        //     }
        // }
        todo!()
    }

    /// Internal use only, traversing nodes in level-order
    fn next_level(&mut self) -> Option<N> {
        // loop {
        //     let front = match self.queue.front_mut() {
        //         Some(obj) => obj,
        //         None => return None,
        //     };
        // 
        //     match front.next() {
        //         Some(node) => {
        //             self.queue.push_back(State::from_pack(node.next_ref()));
        //             return Some(node);
        //         }
        //         None => { self.queue.pop_front(); }
        //     }
        // }
        todo!()
    }
}

impl<N, P: Iterator<Item = N> + Default> Iterator for Base<N, State<N, P>> {
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        // loop {
        //     let node = match self.order {
        //         Order::Pre => self.next_pre(),
        //         Order::Post => self.next_post(),
        //         Order::Level => self.next_level(),
        //     };
        // 
        //     // check if user need to traverse empty node
        //     match node {
        //         Some(node) if !self.empty && node.is_empty() => continue,
        //         _ => return node,
        //     }
        // }
        todo!()
    }
}

// -----------------------------------------------------------------------------

// -----------------------------------------------------------------------------

// pub struct Keys<'a, V> {}

// -----------------------------------------------------------------------------

// todo change name to Data, Path, impl Keys, Values in map
// /// Traverse the tree to retrieve all data
// #[derive(Clone)]
// pub struct Values<'a, V> {
//     iter: Iter<'a, V>
// }
// 
// impl<'a, V> Values<'a, V> {
//     /// Construct a new iterator
//     pub fn new(start: &'a RadixNode<'a, V>) -> Self {
//         Self { iter: Iter::new(start) }
//     }
// 
//     /// Construct with a order
//     ///
//     /// ```
//     /// use radixmap::{RadixMap, iter::Order};
//     ///
//     /// fn main() -> anyhow::Result<()> {
//     ///     let mut map = RadixMap::new();
//     ///     map.insert("/api", "/api")?;
//     ///     map.insert("/api/v1", "/api/v1")?;
//     ///     map.insert("/api/v1/user1", "/api/v1/user1")?;
//     ///     map.insert("/api/v2", "/api/v2")?;
//     ///     map.insert("/api/v2/user2", "/api/v2/user2")?;
//     ///
//     ///     let mut iter = map.values(); // same as with_order(Order::Pre);
//     ///     assert_eq!(iter.next(), Some(&"/api"));
//     ///     assert_eq!(iter.next(), Some(&"/api/v1"));
//     ///     assert_eq!(iter.next(), Some(&"/api/v1/user1"));
//     ///     assert_eq!(iter.next(), Some(&"/api/v2"));
//     ///     assert_eq!(iter.next(), Some(&"/api/v2/user2"));
//     ///     assert_eq!(iter.next(), None);
//     ///
//     ///     let mut iter = map.values().with_order(Order::Post);
//     ///     assert_eq!(iter.next(), Some(&"/api/v1/user1"));
//     ///     assert_eq!(iter.next(), Some(&"/api/v1"));
//     ///     assert_eq!(iter.next(), Some(&"/api/v2/user2"));
//     ///     assert_eq!(iter.next(), Some(&"/api/v2"));
//     ///     assert_eq!(iter.next(), Some(&"/api"));
//     ///     assert_eq!(iter.next(), None);
//     ///
//     ///     let mut iter = map.values().with_order(Order::Level);
//     ///     assert_eq!(iter.next(), Some(&"/api"));
//     ///     assert_eq!(iter.next(), Some(&"/api/v1"));
//     ///     assert_eq!(iter.next(), Some(&"/api/v2"));
//     ///     assert_eq!(iter.next(), Some(&"/api/v1/user1"));
//     ///     assert_eq!(iter.next(), Some(&"/api/v2/user2"));
//     ///     assert_eq!(iter.next(), None);
//     ///
//     ///     Ok(())
//     /// }
//     /// ```
//     pub fn with_order(mut self, order: Order) -> Self {
//         self.iter = self.iter.with_order(order);
//         self
//     }
// }
// 
// impl<'a, V> Iterator for Values<'a, V> {
//     type Item = &'a V;
// 
//     // todo add test for internal nodes without data
//     fn next(&mut self) -> Option<Self::Item> {
//         self.iter.next().and_then(|node| node.data_ref())
//     }
// }