use crate::*;

pub struct RadixNode<'a, T> {
    item: RadixItem<'a>,
    nest: HashMap<u8, RadixNode<'a, T>>,
    data: Option<T>,
}

impl<'a, T> Default for RadixNode<'a, T> {
    fn default() -> Self {
        Self::new("", None)
    }
}

impl<'a, T> RadixNode<'a, T> {
    pub fn new(path: &'a str, data: Option<T>) -> Self {
        Self { item: RadixItem::new(path), nest: HashMap::new(), data }
    }

    pub fn insert(&mut self, path: &'a str, data: T) {
        if path.is_empty() {
            return;
        }

        let edge = match self.nest.get_mut(&path.as_bytes()[0]) {
            Some(obj) => obj,
            None => {
                self.nest.insert(path.as_bytes()[0], RadixNode::new(path, Some(data)));
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

    pub fn longest(&self, path: &'a str) -> &'a str {
        let min = std::cmp::min(self.item.pattern.len(), path.len());
        let mut len = 0;

        while len < min && self.item.pattern.as_bytes()[len] == path.as_bytes()[len] {
            len += 1;
        }

        &path[..len]
    }

    pub fn divide(&mut self, len: usize) {
        let child = RadixNode {
            item: RadixItem::new(&self.item.pattern[len..]),
            nest: std::mem::take(&mut self.nest),
            data: std::mem::take(&mut self.data),
        };

        self.item = RadixItem::new(&self.item.pattern[..len]);
        self.nest.insert(self.item.pattern.as_bytes()[0], child);
    }
}