use super::def::*;
use super::iter::*;
use super::node::*;

pub struct RadixMap<'a, V> {
    /// The empty root node
    root: RadixNode<'a, V>,

    /// The count of leaf nodes
    size: usize,
}

impl<'a, V> RadixMap<'a, V> {
    pub fn new() -> Self {
        Default::default()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.size
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.root.is_leaf()
    }

    // todo (&str, V)
    pub fn iter(&'a self) -> Iter<'a, V> {
        todo!()
    }

    pub fn iter_mut(&'a self) -> Iter<'a, V> {
        todo!()
    }

    pub fn keys(&'a self) -> Keys<'a, V> {
        Keys::from(&self.root)
    }

    pub fn values(&'a self) -> Values<V> {
        Values::from(&self.root)
    }

    pub fn values_mut(&'a mut self) -> Values<V> {
        todo!()
    }

    pub fn clear(&mut self) {
        self.root.clear();
        self.size = 0;
    }

    pub fn get(&self, path: &'a str) -> Option<&V> {
        // self.root.deepest(path).filter(|node| node.path == path).and_then(|node| node.data.as_ref())
        todo!()
    }

    pub fn get_mut(&'a mut self, path: &'a str) -> Option<&mut V> {
        // self.root.deepest_mut(path).filter(|node| node.path == path).and_then(|node| node.data.as_mut())
        todo!()
    }

    pub fn contains_key(&self, path: &'a str) -> bool {
        // match self.root.deepest(path) {
        //     Some(node) => node.path == path,
        //     None => false
        // }
        todo!()
    }

    pub fn contains_value(&self, data: &V) -> bool where V: PartialEq {
        for value in self.values() {
            if value == data {
                return true;
            }
        }

        false
    }

    pub fn insert(&mut self, path: &'a str, data: V) -> RadixResult<Option<V>> {
        let ret = self.root.insert(path, data);
        if let Ok(None) = &ret {
            self.size += 1;
        }
        ret
    }

    pub fn remove(&mut self, path: &'a str) -> Option<RadixNode<'a, V>> {
        todo!()
    }

    pub fn erase(&mut self, path: &'a str) -> Option<V> {
        todo!()
    }
}

// -----------------------------------------------------------------------------

impl<'a, V, const N: usize> TryFrom<[(&'a str, V); N]> for RadixMap<'a, V> {
    type Error = RadixError;

    fn try_from(value: [(&'a str, V); N]) -> Result<Self, Self::Error> {
        let mut map = RadixMap::default();

        for (path, data) in value {
            map.insert(path, data)?;
        }

        Ok(map)
    }
}

impl<'a, V> Default for RadixMap<'a, V> {
    fn default() -> Self {
        Self { root: RadixNode::default(), size: 0 }
    }
}

impl<'a, V: Debug> Debug for RadixMap<'a, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<'a, V: Clone> Clone for RadixMap<'a, V> {
    fn clone(&self) -> Self {
        Self { root: self.root.clone(), size: self.size }
    }
}

impl<'a, V: Eq> Eq for RadixMap<'a, V> {}

impl<'a, V: PartialEq> PartialEq for RadixMap<'a, V> {
    fn eq(&self, _other: &Self) -> bool {
        todo!()
    }
}

impl<'a, V> Index<&'a str> for RadixMap<'a, V> {
    type Output = V;

    fn index(&self, path: &'a str) -> &Self::Output {
        match self.get(path) {
            Some(data) => data,
            None => panic!("no entry found for path '{}'", path)
        }
    }
}