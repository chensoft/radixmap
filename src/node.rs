use super::def::*;
use super::item::*;

pub struct RadixNode<'a, T> {
    pub item: RadixItem<'a>,
    pub nest: SparseSet<RadixNode<'a, T>>,
    pub data: Option<T>,
}

impl<'a, T> Default for RadixNode<'a, T> {
    fn default() -> Self {
        Self::new("", None)
    }
}

impl<'a, T> RadixNode<'a, T> {
    pub fn new(path: &'a str, data: Option<T>) -> Self {
        Self { item: RadixItem::new(path), nest: SparseSet::with_capacity(256), data }
    }

    pub fn insert(&mut self, path: &'a str, data: T) {
        if path.is_empty() {
            self.data = Some(data);
            return;
        }

        let edge = match self.nest.get_mut(path.as_bytes()[0] as usize) {
            Some(obj) => obj,
            None => {
                self.nest.insert(path.as_bytes()[0] as usize, RadixNode::new(path, Some(data)));
                return;
            }
        };

        let share = edge.longest(path);

        match edge.item.pattern.len().cmp(&share.len()) {
            Ordering::Less => unreachable!(),
            Ordering::Equal => {
                match path.len().cmp(&share.len()) {
                    Ordering::Less => unreachable!(),
                    Ordering::Equal => edge.data = Some(data),
                    Ordering::Greater => edge.insert(&path[share.len()..], data),
                }
            }
            Ordering::Greater => {
                edge.divide(share.len());
                edge.insert(&path[share.len()..], data);
            }
        }
    }

    pub fn clear(&mut self) {
        todo!()
    }

    pub fn longest(&self, path: &'a str) -> &'a str {
        let min = std::cmp::min(self.item.pattern.len(), path.len());
        let mut len = 0;

        while len < min && self.item.pattern.as_bytes()[len] == path.as_bytes()[len] {
            len += 1;
        }

        &path[..len]
    }

    pub fn deepest(&self, path: &'a str) -> Option<&RadixNode<'a, T>> {
        if path.is_empty() {
            return Some(self);
        }

        match self.nest.get(path.as_bytes()[0] as usize) {
            Some(nest) if nest.longest(path).len() == nest.item.pattern.len() => nest.deepest(&path[nest.item.pattern.len()..]),
            _ => None
        }
    }

    pub fn divide(&mut self, len: usize) {
        let child = RadixNode {
            item: RadixItem::new(&self.item.pattern[len..]),
            nest: std::mem::replace(&mut self.nest, SparseSet::with_capacity(256)),
            data: std::mem::take(&mut self.data),
        };

        self.item = RadixItem::new(&self.item.pattern[..len]);
        self.nest.insert(child.item.pattern.as_bytes()[0] as usize, child);
    }
}

type SparseIter<'a, T> = std::slice::Iter<'a, sparseset::Entry<RadixNode<'a, T>>>;

pub struct RadixNodeIterator<'a, T> {
    start: Option<&'a RadixNode<'a, T>>,
    stack: Vec<SparseIter<'a, T>>
}

impl<'a, T> RadixNodeIterator<'a, T> {
    pub fn new(start: &'a RadixNode<'a, T>) -> Self {
        Self { start: Some(start), stack: vec![] }
    }

    // todo 3 visit method
    pub fn with_prefix(mut self, prefix: &'a str) -> Self {
        if let Some(start) = self.start {
            self.start = start.deepest(prefix);
        }

        self
    }
}

impl<'a, T> Iterator for RadixNodeIterator<'a, T> {
    type Item = &'a RadixNode<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(top) = self.start {
            self.start = None;
            self.stack.push(top.nest.iter());
            return Some(top);
        }

        match self.stack.last_mut() {
            Some(top) => match top.next() {
                Some(obj) => {
                    self.stack.push(obj.value.nest.iter());
                    Some(obj.value())
                }
                None => {
                    self.stack.pop();
                    self.next()
                }
            }
            None => None
        }
    }
}