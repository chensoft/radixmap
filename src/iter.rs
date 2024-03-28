use super::def::*;
use super::node::*;

// -----------------------------------------------------------------------------

pub enum State<'a, V> {
    Single(Option<&'a RadixNode<'a, V>>),
    Sparse(std::slice::Iter<'a, sparseset::Entry<RadixNode<'a, V>>>),
    Index(indexmap::map::Values<'a, &'a str, RadixNode<'a, V>>),
}

impl<'a, V> Iterator for State<'a, V> {
    type Item = &'a RadixNode<'a, V>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            State::Single(iter) => iter.take(),
            State::Sparse(iter) => match iter.next() {
                Some(node) => Some(node.value()),
                None => None
            }
            State::Index(iter) => iter.next()
        }
    }
}

pub enum StateMut<'a, V> {
    Single(Option<&'a mut RadixNode<'a, V>>),
    Sparse(std::slice::IterMut<'a, sparseset::Entry<RadixNode<'a, V>>>),
    Index(indexmap::map::ValuesMut<'a, &'a str, RadixNode<'a, V>>),
}

impl<'a, V> Iterator for StateMut<'a, V> {
    type Item = &'a mut RadixNode<'a, V>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            StateMut::Single(iter) => iter.take(),
            StateMut::Sparse(iter) => match iter.next() {
                Some(node) => Some(node.value_mut()),
                None => None
            }
            StateMut::Index(iter) => iter.next()
        }
    }
}

// -----------------------------------------------------------------------------

pub enum Order {
    Pre,
    In,
    Post,
    Level
}

pub struct Iter<'a, V> {
    start: &'a RadixNode<'a, V>,
    queue: VecDeque<State<'a, V>>,
    order: Order,
}

impl<'a, V> Iter<'a, V> {
    pub fn new(start: &'a RadixNode<'a, V>) -> Self {
        Self { start, queue: VecDeque::from([State::Single(Some(start))]), order: Order::Pre }
    }

    pub fn use_prefix(mut self, prefix: &'a str) -> Self {
        // if let Some(start) = self.start {
        //     self.start = start.deepest(prefix);
        // }

        self.queue.clear();
        // self.queue.push_back(State::Single(Some(start)));
        self
    }

    pub fn use_order(mut self, order: Order) -> Self {
        self.order = order;
        self
    }

    fn next_pre(&mut self) -> Option<&'a RadixNode<'a, V>> {
        let top = match self.queue.back_mut() {
            Some(obj) => obj,
            None => return None,
        };

        // todo
        // if let Some(top) = self.queue.back_mut() {
        //     match top {
        //         State::SparseSet(top) => {
        //             // match top.next() {
        //             //     Some(obj) => {
        //             //         self.stack.push(obj.value.next.iter());
        //             //         Some(obj.value())
        //             //     }
        //             //     None => {
        //             //         self.queue.pop_back();
        //             //         self.next()
        //             //     }
        //             // }
        //         }
        //         State::IndexMap(top) => {
        // 
        //         }
        //     }
        //     // let next = match top {
        //     //     State::SparseSet(top) => top.next().unwrap().value,
        //     //     State::IndexMap(top) => top.next(),
        //     // };
        // }

        None
    }

    fn next_in(&mut self) -> Option<&'a RadixNode<'a, V>> {
        todo!()
    }

    fn next_post(&mut self) -> Option<&'a RadixNode<'a, V>> {
        todo!()
    }

    fn next_level(&mut self) -> Option<&'a RadixNode<'a, V>> {
        todo!()
    }
}

impl<'a, V> Iterator for Iter<'a, V> {
    type Item = &'a RadixNode<'a, V>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.order {
            Order::Pre => self.next_pre(),
            Order::In => self.next_in(),
            Order::Post => self.next_post(),
            Order::Level => self.next_level(),
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