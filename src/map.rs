use super::def::*;
use super::node::*;

// todo the 'a lifetime problem, use Cow?
#[derive(Default)]
pub struct RadixMap<'a, V> {
    root: RadixNode<'a, V>,
    size: usize,
}

impl<'a, V> RadixMap<'a, V> {
    // pub fn keys(&self) -> Keys<'a, V> {
    //     todo!()
    // }
    //
    // pub fn values(&self) -> Values<'a, V> {
    //     todo!()
    // }
    //
    // pub fn values_mut(&mut self) -> ValuesMut<'a, V> {
    //     todo!()
    // }

    // pub fn iter(&'a self) -> RadixNodeIterator<'a, V> {
    //     RadixNodeIterator::new(&self.root)
    // }
    // 
    // pub fn iter_mut(&'a self) -> RadixNodeIterator<'a, V> {
    //     RadixNodeIterator::new(&self.root)
    // }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.root.next.is_empty()
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
    // 
    // pub fn get(&self, path: &'a str) -> Option<&V> {
    //     // self.iter().with_prefix(path).next().and_then(|node| node.data.as_ref())
    //     todo!()
    // }
    // 
    // pub fn get_mut(&mut self, path: &'a str) -> Option<&mut V> {
    //     // self.iter_mut().with_prefix(path).next().and_then(|node| node.data.as_mut())
    //     todo!()
    // }
    // 
    // pub fn contains_key(&self, path: &'a str) -> bool {
    //     todo!()
    // }
    // 
    // // todo O(n)
    // pub fn contains_value(&self, data: &V) -> bool {
    //     todo!()
    // }

    pub fn insert(&mut self, path: &'a str, data: V) -> Result<Option<V>> {
        self.root.insert(&mut self.size, path, data)
    }

    // pub fn remove(&mut self, path: &'a str) -> Option<V> {
    //     todo!()
    // }
}

// impl<'a, V: Debug> Debug for RadixMap<'a, V> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
// }
// 
// impl<'a, V: Clone> Clone for RadixMap<'a, V> {
//     fn clone(&self) -> Self {
//         todo!()
//     }
// }
// 
// impl<'a, V: Eq> Eq for RadixMap<'a, V> {}
// 
// impl<'a, V: PartialEq> PartialEq for RadixMap<'a, V> {
//     fn eq(&self, other: &Self) -> bool {
//         todo!()
//     }
// }
// 
// impl<'a, V> Index<&V> for RadixMap<'a, V> {
//     type Output = V;
// 
//     fn index(&self, index: &V) -> &Self::Output {
//         todo!()
//     }
// }
// 
// impl<'a, V> IndexMut<&V> for RadixMap<'a, V> {
//     fn index_mut(&mut self, index: &V) -> &mut Self::Output {
//         todo!()
//     }
// }
// 
// impl<'a, V, const N: usize> From<[(&'a str, V); N]> for RadixMap<'a, V> {
//     fn from(value: [(&'a str, V); N]) -> Self {
//         todo!()
//     }
// }
// 
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

// -----------------------------------------------------------------------------

// pub struct Iter<'a, V> {}
//
// pub struct IterMut<'a, V> {}

// -----------------------------------------------------------------------------

// pub struct Keys<'a, V> {}

// -----------------------------------------------------------------------------

// pub struct Values<'a, V> {}
//
// pub struct ValuesMut<'a, V> {}

// todo Debug, Clone for Iter, Send for Mut


// type SparseIter<'a, T> = std::slice::Iter<'a, sparseset::Entry<RadixNode<'a, T>>>;
// 
// pub struct RadixNodeIterator<'a, T> {
//     start: Option<&'a RadixNode<'a, T>>,
//     stack: Vec<SparseIter<'a, T>>
// }
// 
// impl<'a, T> RadixNodeIterator<'a, T> {
//     pub fn new(start: &'a RadixNode<'a, T>) -> Self {
//         Self { start: Some(start), stack: vec![] }
//     }
// 
//     // todo 3 visit method
//     pub fn with_prefix(mut self, prefix: &'a str) -> Self {
//         if let Some(start) = self.start {
//             self.start = start.deepest(prefix);
//         }
// 
//         self
//     }
// }
// 
// impl<'a, T> Iterator for RadixNodeIterator<'a, T> {
//     type Item = &'a RadixNode<'a, T>;
// 
//     fn next(&mut self) -> Option<Self::Item> {
//         if let Some(top) = self.start {
//             self.start = None;
//             self.stack.push(top.next.iter());
//             return Some(top);
//         }
// 
//         match self.stack.last_mut() {
//             Some(top) => match top.next() {
//                 Some(obj) => {
//                     self.stack.push(obj.value.next.iter());
//                     Some(obj.value())
//                 }
//                 None => {
//                     self.stack.pop();
//                     self.next()
//                 }
//             }
//             None => None
//         }
//     }
// }