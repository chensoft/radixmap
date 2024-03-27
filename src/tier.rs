use super::def::*;
use super::node::*;

pub struct RadixTier<'a, V> {
    pub regular: SparseSet<RadixNode<'a, V>>,
    pub special: Vec<RadixNode<'a, V>>,
}

impl<'a, V> Default for RadixTier<'a, V> {
    fn default() -> Self {
        Self { regular: SparseSet::with_capacity(256), special: vec![] }
    }
}

impl<'a, V> RadixTier<'a, V> {
    pub fn is_empty(&self) -> bool {
        self.regular.is_empty() && self.special.is_empty()
    }

    pub fn insert(&mut self, size: &mut usize, part: &'a str) -> Result<&mut RadixNode<'a, V>> {
        // let bytes = part.as_bytes();
        // let first = match bytes.first() {
        //     Some(val) => *val,
        //     None => return Err(Error::PathEmpty.into())
        // };
        // 
        // // search sparse array
        // if let Some(edge) = self.regular.get_mut(first as usize) {
        //     let comm = edge.item.longest(part);
        // 
        //     //     match edge.item.pattern.len().cmp(&share.len()) {
        //     //         Ordering::Less => unreachable!(),
        //     //         Ordering::Equal => {
        //     //             match path.len().cmp(&share.len()) {
        //     //                 Ordering::Less => unreachable!(),
        //     //                 Ordering::Equal => edge.data = Some(data),
        //     //                 Ordering::Greater => edge.insert(&path[share.len()..], data),
        //     //             }
        //     //         }
        //     //         Ordering::Greater => {
        //     //             edge.divide(share.len());
        //     //             edge.insert(&path[share.len()..], data);
        //     //         }
        //     //     }
        // }
        // 
        // // regex, param, blob
        // for node in &self.special {
        //     todo!()
        // }
        // 
        // // add new node to self


        todo!()
    }
}