use super::def::*;
use super::node::*;

// todo merge Mut and IMut
// -----------------------------------------------------------------------------

/// Iterator adapter for nodes and packs
pub enum State<'a, V> {
    Node(Option<&'a RadixNode<'a, V>>),
    Pack(std::slice::Iter<'a, sparseset::Entry<RadixNode<'a, V>>>, indexmap::map::Values<'a, &'a str, RadixNode<'a, V>>),
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

/// Iterate order for radix tree
pub enum Order {
    Pre,
    Post,
    Level
}

/// The iterator for radix tree
pub struct Iter<'a, V> {
    _start: &'a RadixNode<'a, V>,
    queue: VecDeque<Peekable<State<'a, V>>>,
    visit: Vec<Peekable<State<'a, V>>>,
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
    pub fn new(_start: &'a RadixNode<'a, V>) -> Self {
        Self { _start, queue: VecDeque::from([State::Node(Some(_start)).peekable()]), visit: vec![], order: Order::Pre, empty: false }
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

    /// Traverse all nodes, including the edge nodes, which do not contain data
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
    ///     let mut iter = map.iter().with_empty();
    ///     assert_eq!(iter.next().unwrap().item.origin(), "");       // data: None, the root node
    ///     assert_eq!(iter.next().unwrap().item.origin(), "/api");   // data: /api
    ///     assert_eq!(iter.next().unwrap().item.origin(), "/v");     // data: None, an edge node
    ///     assert_eq!(iter.next().unwrap().item.origin(), "1");      // data: /api/v1
    ///     assert_eq!(iter.next().unwrap().item.origin(), "/user1"); // data: /api/v1/user1
    ///     assert_eq!(iter.next().unwrap().item.origin(), "2");      // data: /api/v2
    ///     assert_eq!(iter.next().unwrap().item.origin(), "/user2"); // data: /api/v2/user2
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
        let back = match self.queue.back_mut() {
            Some(obj) => obj,
            None => return None,
        };

        let node = match back.next() {
            Some(obj) => obj,
            None => {
                self.queue.pop_back();
                return self.next_pre(); // todo test depth
            }
        };

        self.queue.push_back(State::Pack(node.next.regular.iter(), node.next.special.values()).peekable());

        Some(node)
    }

    /// Internal use only, traversing nodes in post-order
    fn next_post(&mut self) -> Option<&'a RadixNode<'a, V>> {
        if let Some(mut back) = self.queue.pop_back() {
            loop {
                let pack = match back.peek() {
                    Some(node) => State::Pack(node.next.regular.iter(), node.next.special.values()).peekable(),
                    None => break
                };

                self.visit.push(back);

                back = pack;
            }

            return self.next_post();
        }

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
        let front = match self.queue.front_mut() {
            Some(obj) => obj,
            None => return None,
        };

        let node = match front.next() {
            Some(obj) => obj,
            None => {
                self.queue.pop_front();
                return self.next_level();
            }
        };

        self.queue.push_back(State::Pack(node.next.regular.iter(), node.next.special.values()).peekable());

        Some(node)
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

        match node {
            Some(node) if !self.empty && node.is_empty() => self.next(),
            _ => node,
        }
    }
}

// pub struct IterMut<'a, V> {}

// -----------------------------------------------------------------------------

// pub struct Keys<'a, V> {}

// -----------------------------------------------------------------------------

// pub struct Values<'a, V> {}
//
// pub struct ValuesMut<'a, V> {}

// todo Debug, Clone for Iter, Send for Mut