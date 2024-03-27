use super::def::*;
use super::item::*;
use super::node::*;

pub struct RadixTier<'a, V> {
    pub regular: SparseSet<RadixNode<'a, V>>,
    pub special: IndexMap<String, RadixNode<'a, V>>,
}

impl<'a, V> Default for RadixTier<'a, V> {
    fn default() -> Self {
        Self { regular: SparseSet::with_capacity(256), special: IndexMap::new() }
    }
}

impl<'a, V> RadixTier<'a, V> {
    pub fn insert(&mut self, size: &mut usize, item: RadixItem<'a>) -> Result<&mut RadixNode<'a, V>> {
        // use sparse array to find regular node
        if let RadixItem::Plain { text } = item {
            let bytes = text.as_bytes();
            let first = match bytes.first() {
                Some(val) => *val as usize,
                None => return Err(Error::PathEmpty.into())
            };

            if !self.regular.contains(first) {
                self.regular.insert(first, RadixNode::new(item, None).incr(size));
                return Ok(self.regular[first].value_mut());
            }

            let found = self.regular[first].value_mut();
            let (share, order) = found.item.longest(text);

            match order {
                Ordering::Less => unreachable!(),
                Ordering::Equal => {
                    match text.len().cmp(&share.len()) {
                        Ordering::Less => unreachable!(),
                        Ordering::Equal => return Ok(found),
                        Ordering::Greater => return found.next.insert(size, RadixItem::new_plain(&text[share.len()..])?),
                    }
                }
                Ordering::Greater => {
                    *size += 1;
                    found.divide(share.len());
                    return found.next.insert(size, RadixItem::new_plain(&text[share.len()..])?);
                }
            }
        }

        // special nodes inserted directly into map
        let origin = item.origin();

        match self.special.contains_key(origin) {
            true => Ok(&mut self.special[origin]),
            false => Ok(self.special.entry(origin.to_string()).or_insert(RadixNode::new(item, None).incr(size)))
        }
    }

    pub fn is_empty(&self) -> bool {
        self.regular.is_empty() && self.special.is_empty()
    }
}