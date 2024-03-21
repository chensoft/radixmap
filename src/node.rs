use crate::*;

pub struct RadixNode<'a, T> {
    pub item: RadixItem<'a>,
    pub nest: Vec<RadixNode<'a, T>>, // todo use search table store first byte
    pub data: Option<T>,
}

impl<'a, T> Default for RadixNode<'a, T> {
    fn default() -> Self {
        Self { item: RadixItem::default(), nest: vec![], data: None }
    }
}

impl<'a, T> RadixNode<'a, T> {
    pub fn new(path: &'a str, data: T) -> Self {
        Self { item: RadixItem::new(path), nest: vec![], data: Some(data) }
    }

    pub fn longest(&self, path: &'a str) -> (&'a str, &'a str) {
        let min = std::cmp::min(self.item.pattern.len(), path.len());
        let mut len = 0;

        while len < min && self.item.pattern.as_bytes()[len] == path.as_bytes()[len] {
            len += 1;
        }

        (&path[..len], &path[len..])
    }

    // todo change name
    pub fn replace(&mut self, p: &'a str, c: &'a str) {
        let mut node = RadixNode::default();
        node.item = RadixItem::new(c);
        node.nest = std::mem::take(&mut self.nest);
        node.data = std::mem::take(&mut self.data);

        self.item = RadixItem::new(p);
        self.nest.push(node);
    }
}