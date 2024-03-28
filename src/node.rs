use super::def::*;
use super::item::*;
use super::pack::*;

/// The basic element inside a tree
pub struct RadixNode<'a, V> {
    pub item: RadixItem<'a>,
    pub data: Option<V>,
    pub next: RadixPack<'a, V>,
}

/// Create an empty node
impl<'a, V> Default for RadixNode<'a, V> {
    fn default() -> Self {
        Self { item: RadixItem::default(), data: None, next: RadixPack::default() }
    }
}

impl<'a, V> RadixNode<'a, V> {
    /// Create a radix node
    ///
    /// ```
    /// use radixmap::{item::RadixItem, node::RadixNode};
    ///
    /// fn main() -> anyhow::Result<()> {
    ///     let node = RadixNode::new(RadixItem::new_plain(r"/api")?, Some(12345));
    ///
    ///     assert_eq!(node.item.origin(), "/api");
    ///     assert_eq!(node.data, Some(12345));
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn new(item: RadixItem<'a>, data: Option<V>) -> Self {
        Self { item, data, next: RadixPack::default() }
    }

    /// Internal use for recording how many nodes are created
    ///
    /// ```
    /// use radixmap::{node::RadixNode};
    ///
    /// let mut size = 0;
    /// RadixNode::<'_, ()>::default().incr(&mut size);
    ///
    /// assert_eq!(size, 1);
    /// ```
    pub fn incr(self, size: &mut usize) -> Self {
        *size += 1;
        self
    }

    pub fn insert(&mut self, size: &mut usize, path: &'a str, data: V) -> Result<Option<V>> {
        let next = RadixItem::extract(path)?;
        let edge = self.next.insert(size, next)?;

        if next.len() == path.len() {
            let prev = std::mem::take(&mut edge.data);
            edge.data = Some(data);
            return Ok(prev);
        }

        edge.insert(size, &path[next.len()..], data)
    }

    /// Divide the node into two parts
    ///
    /// ```
    /// use radixmap::{item::RadixItem, node::RadixNode};
    ///
    /// fn main() -> anyhow::Result<()> {
    ///     let mut node = RadixNode::new(RadixItem::new_plain(r"/api")?, Some(12345));
    ///     let leaf = node.divide(1)?;
    ///
    ///     assert_eq!(node.item.origin(), "/");
    ///     assert_eq!(node.data, None);
    ///     assert_eq!(leaf.item.origin(), "api");
    ///     assert_eq!(leaf.data, Some(12345));
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn divide(&mut self, len: usize) -> Result<RadixNode<'a, V>> {
        Ok(RadixNode {
            item: self.item.divide(len)?,
            data: std::mem::take(&mut self.data),
            next: std::mem::take(&mut self.next),
        })
    }

    pub fn clear(&mut self) {
        self.item = RadixItem::default();
        self.data = None;
        self.next.clear();
    }
}

/// Debug trait
///
/// ```
/// use radixmap::{item::RadixItem, node::RadixNode};
///
/// fn main() -> anyhow::Result<()> {
///     assert_eq!(format!("{:?}", RadixNode::new(RadixItem::new_plain(r"/api")?, None::<()>)), r"Plain(/api)".to_string());
///     assert_eq!(format!("{:?}", RadixNode::new(RadixItem::new_regex(r"{id:\d+}")?, None::<()>)), r"Regex({id:\d+})".to_string());
///     assert_eq!(format!("{:?}", RadixNode::new(RadixItem::new_param(r":id")?, None::<()>)), r"Param(:id)".to_string());
///     assert_eq!(format!("{:?}", RadixNode::new(RadixItem::new_glob(r"*")?, None::<()>)), r"Glob(*)".to_string());
///
///     Ok(())
/// }
/// ```
impl<'a, V> Debug for RadixNode<'a, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.item.fmt(f)
    }
}