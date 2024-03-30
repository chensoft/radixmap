use super::def::*;
use super::iter::*;
use super::node::*;

// todo the 'a lifetime problem, use Cow?
pub struct RadixMap<'a, V> {
    /// The empty root node
    root: RadixNode<'a, V>,

    /// The count of leaf nodes
    size: usize,
}

impl<'a, V> Default for RadixMap<'a, V> {
    fn default() -> Self {
        Self { root: RadixNode::default(), size: 0 }
    }
}

impl<'a, V> RadixMap<'a, V> {
    pub fn new() -> Self {
        Default::default()
    }

    // pub fn keys(&self) -> Keys<'a, V> {
    //     todo!()
    // }

    pub fn values(&'a self) -> Values<'a, V> {
        Values::new(&self.root)
    }

    // pub fn values_mut(&mut self) -> ValuesMut<'a, V> {
    //     todo!()
    // }

    pub fn iter(&'a self) -> Iter<'a, V> {
        Iter::new(&self.root)
    }

    // pub fn iter_mut(&'a self) -> RadixNodeIterator<'a, V> {
    //     RadixNodeIterator::new(&self.root)
    // }

    #[inline]
    pub fn len(&self) -> usize {
        self.size
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.root.is_leaf()
    }

    // pub fn drain(&mut self) {
    //     todo!()
    // }
    // 
    // pub fn retain(&mut self) {
    //     todo!()
    // }
    // 
    // pub fn extract_if(&mut self) {
    //     todo!()
    // }

    pub fn clear(&mut self) {
        self.root.clear();
        self.size = 0;
    }

    // // todo get or insert, return Node
    // pub fn entry(&mut self) {
    //     todo!()
    // }

    pub fn get(&self, path: &'a str) -> Option<&V> {
        self.iter().with_prefix(path).next().and_then(|node| node.data_ref())
    }

    pub fn get_mut(&mut self, _path: &'a str) -> Option<&mut V> {
        // self.iter_mut().with_prefix(path).next().and_then(|node| node.data.as_mut())
        todo!()
    }

    pub fn contains_key(&self, path: &'a str) -> bool {
        self.iter().with_prefix(path).next().is_some()
    }

    pub fn contains_value(&self, data: &V) -> bool where V: PartialEq {
        for value in self.values() {
            if value == data {
                return true;
            }
        }

        false
    }

    pub fn insert(&mut self, path: &'a str, data: V) -> Result<Option<V>> {
        let ret = self.root.insert(path, data);
        if let Ok(None) = &ret {
            self.size += 1;
        }
        ret
    }

    // pub fn remove(&mut self, path: &'a str) -> Option<RadixNode<'a, V>> {
    //     todo!()
    // }
}

// impl<'a, V: Debug> Debug for RadixMap<'a, V> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
// }

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

impl<'a, V> IndexMut<&'a str> for RadixMap<'a, V> {
    fn index_mut(&mut self, path: &'a str) -> &mut Self::Output {
        match self.get_mut(path) {
            Some(data) => data,
            None => panic!("no entry found for path '{}'", path)
        }
    }
}

impl<'a, V, const N: usize> TryFrom<[(&'a str, V); N]> for RadixMap<'a, V> {
    type Error = anyhow::Error;

    fn try_from(value: [(&'a str, V); N]) -> std::result::Result<Self, Self::Error> {
        let mut map = RadixMap::default();

        for (path, data) in value {
            map.insert(path, data)?;
        }

        Ok(map)
    }
}

// impl<'a, V> IntoIterator for &'a RadixMap<'a, V> {
//     type Item = ();
//     type IntoIter = ();
//
//     fn into_iter(self) -> Self::IntoIter {
//         todo!()
//     }
// }
// 
// impl<'a, V> IntoIterator for &'a mut RadixMap<'a, V> {
//     type Item = ();
//     type IntoIter = ();
//
//     fn into_iter(self) -> Self::IntoIter {
//         todo!()
//     }
// }
//
// impl<'a, V> IntoIterator for RadixMap<'a, V> {
//     type Item = ();
//     type IntoIter = ();
//
//     fn into_iter(self) -> Self::IntoIter {
//         todo!()
//     }
// }

// impl<'a, V> FromIterator<(&'a str, V)> for RadixMap<'a, V> {
//     fn from_iter<T: IntoIterator<Item=(&'a str, V)>>(iter: V) -> Self {
//         todo!()
//     }
// }

// impl<'a, V> Extend<(&'a str, V)> for RadixMap<'a, V> {
//     fn extend<T: IntoIterator<Item=(&'a str, V)>>(&mut self, iter: V) {
//         todo!()
//     }
// }