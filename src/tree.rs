use crate::*;

#[derive(Default)]
pub struct RadixTree<'a, T> {
    root: RadixNode<'a, T>,
}

// todo support chars-based search
impl<'a, T> RadixTree<'a, T> {
    pub fn insert(&mut self, mut path: &'a str, data: T) -> &mut Self {
        // todo comment give some sample
        let mut edge = &mut self.root;

        while !edge.nest.is_empty() {
            for idx in 0..edge.nest.len() {
                let node = &edge.nest[idx];
                let (both, rest) = node.longest(path);
                if both.is_empty() {
                    continue;
                }

                edge = &mut edge.nest[idx];
                path = rest;

                if both.len() < edge.item.pattern.len() {
                    edge.replace(both, &edge.item.pattern[both.len()..]);
                    break;
                }
            }
        }

        edge.nest.push(RadixNode::new(path, data));

        self
    }

    pub fn search(&mut self) {
        todo!()
    }

    pub fn remove(&mut self) {
        todo!()
    }

    pub fn prefix(&mut self) {
        // todo impl -> Iterator
        todo!()
    }
}