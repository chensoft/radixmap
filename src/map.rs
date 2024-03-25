use super::def::*;
use super::node::*;

#[derive(Default)]
pub struct RadixMap<'a, T> {
    root: RadixNode<'a, T>,
}

// todo support chars-based search
impl<'a, T> RadixMap<'a, T> {
    // pub fn keys(&self) -> Keys<'a, T> {
    //     todo!()
    // }
    //
    // pub fn values(&self) -> Values<'a, T> {
    //     todo!()
    // }
    //
    // pub fn values_mut(&mut self) -> ValuesMut<'a, T> {
    //     todo!()
    // }

    pub fn iter(&'a self) -> RadixNodeIterator<'a, T> {
        RadixNodeIterator::new(&self.root)
    }

    pub fn iter_mut(&'a self) -> RadixNodeIterator<'a, T> {
        RadixNodeIterator::new(&self.root)
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }

    pub fn drain(&mut self) {
        todo!()
    }

    pub fn retain(&mut self) {
        todo!()
    }

    pub fn extract_if(&mut self) {
        todo!()
    }

    pub fn clear(&mut self) {
        todo!()
    }

    // todo get or insert, return Node
    pub fn entry(&mut self) {
        todo!()
    }

    pub fn get(&self) {
        todo!()
    }

    pub fn get_mut(&mut self) {
        todo!()
    }

    pub fn contains_key(&self) -> bool {
        todo!()
    }

    // todo O(n)
    pub fn contains_value(&self) -> bool {
        todo!()
    }

    pub fn insert(&mut self, path: &'a str, data: T) -> &mut Self {
        self.root.insert(path, data); // todo
        self
    }

    pub fn remove(&mut self) -> Option<T> {
        todo!()
    }

    pub fn search(&self, path: &'a str) -> Option<&T> {
        match self.iter().with_prefix(path).next() {
            Some(obj) => obj.data.as_ref(),
            None => None
        }
    }
}

impl<'a, T: Debug> Debug for RadixMap<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<'a, T: Clone> Clone for RadixMap<'a, T> {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl<'a, T: Eq> Eq for RadixMap<'a, T> {}

impl<'a, T: PartialEq> PartialEq for RadixMap<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl<'a, T> Index<&T> for RadixMap<'a, T> {
    type Output = T;

    fn index(&self, index: &T) -> &Self::Output {
        todo!()
    }
}

impl<'a, T> IndexMut<&T> for RadixMap<'a, T> {
    fn index_mut(&mut self, index: &T) -> &mut Self::Output {
        todo!()
    }
}

impl<'a, T, const N: usize> From<[(&'a str, T); N]> for RadixMap<'a, T> {
    fn from(value: [(&'a str, T); N]) -> Self {
        todo!()
    }
}

// impl<'a, T> IntoIterator for &'a RadixMap<'a, T> {
//     type Item = ();
//     type IntoIter = ();
//
//     fn into_iter(self) -> Self::IntoIter {
//         todo!()
//     }
// }
//
// impl<'a, T> IntoIterator for &'a mut RadixMap<'a, T> {
//     type Item = ();
//     type IntoIter = ();
//
//     fn into_iter(self) -> Self::IntoIter {
//         todo!()
//     }
// }
//
// impl<'a, T> IntoIterator for RadixMap<'a, T> {
//     type Item = ();
//     type IntoIter = ();
//
//     fn into_iter(self) -> Self::IntoIter {
//         todo!()
//     }
// }

// impl<'a, T> FromIterator<(&'a str, T)> for RadixMap<'a, T> {
//     fn from_iter<T: IntoIterator<Item=(&'a str, T)>>(iter: T) -> Self {
//         todo!()
//     }
// }

// impl<'a, T> Extend<(&'a str, T)> for RadixMap<'a, T> {
//     fn extend<T: IntoIterator<Item=(&'a str, T)>>(&mut self, iter: T) {
//         todo!()
//     }
// }

// -----------------------------------------------------------------------------

// pub struct Iter<'a, T> {}
//
// pub struct IterMut<'a, T> {}

// -----------------------------------------------------------------------------

// pub struct Keys<'a, T> {}

// -----------------------------------------------------------------------------

// pub struct Values<'a, T> {}
//
// pub struct ValuesMut<'a, T> {}

// todo Debug, Clone for Iter, Send for Mut