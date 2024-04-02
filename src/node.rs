use super::def::*;
use super::iter::*;
use super::rule::*;
use super::pack::*;

/// The basic element inside a tree
pub struct RadixNode<'a, V> {
    pub path: &'a str,
    pub data: Option<V>,

    pub rule: RadixRule<'a>,
    pub next: RadixPack<'a, V>,
}

impl<'a, V> RadixNode<'a, V> {
    pub fn is_leaf(&self) -> bool {
        self.next.is_empty()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_none()
    }

    pub fn iter(&'a self) -> Iter<'a, V> {
        Iter::from(self)
    }

    pub fn deepest(&'a self, path: &'a str) -> Option<&'a RadixNode<'a, V>> {
        // match self.iter().with_prefix(path) {
        //     Ok(mut iter) => iter.next(),
        //     Err(_) => None
        // }
        // todo
        None
    }

    pub fn deepest_mut(&mut self, path: &str) -> Option<&'a mut RadixNode<'a, V>> {
        todo!()
    }

    pub fn insert(&mut self, path: &'a str, data: V) -> RadixResult<Option<V>> {
        let mut frag = path;

        loop {
            // extract the next path fragment and insert it via pack
            let next = RadixRule::extract(frag)?;
            let slot = self.next.insert(next)?;

            // encountering a leaf node indicates completion of insertion
            if next.len() == frag.len() {
                let prev = slot.data.take();
                slot.path = path;
                slot.data = Some(data);

                return Ok(prev);
            }

            frag = &frag[next.len()..];
        }
    }

    /// Divide the node into two parts
    ///
    /// ```
    /// use radixmap::{rule::RadixRule, node::RadixNode};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut node = RadixNode::try_from(("/api", 12345))?;
    ///
    ///     assert_eq!(node.rule_ref(), "/api");
    ///     assert_eq!(node.data_ref(), Some(&12345));
    ///
    ///     let leaf = node.divide(1)?;
    ///
    ///     assert_eq!(node.rule_ref(), "/");
    ///     assert_eq!(node.data_ref(), None);
    ///     assert_eq!(leaf.rule_ref(), "api");
    ///     assert_eq!(leaf.data_ref(), Some(&12345));
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn divide(&mut self, len: usize) -> RadixResult<RadixNode<'a, V>> {
        Ok(RadixNode {
            path: std::mem::take(&mut self.path),
            data: self.data.take(),

            rule: self.rule.divide(len)?,
            next: std::mem::take(&mut self.next),
        })
    }

    pub fn clear(&mut self) {
        self.path = "";
        self.data = None;
        self.rule = RadixRule::default();
        self.next.clear();
    }
}

impl<'a, V> From<RadixRule<'a>> for RadixNode<'a, V> {
    fn from(rule: RadixRule<'a>) -> Self {
        Self { path: "", data: None, rule, next: Default::default() }
    }
}

impl<'a, V> TryFrom<(&'a str, V)> for RadixNode<'a, V> {
    type Error = RadixError;

    fn try_from((path, data): (&'a str, V)) -> RadixResult<Self> {
        Ok(Self { path, data: Some(data), rule: RadixRule::new(path)?, next: Default::default() })
    }
}

/// Debug trait
///
/// ```
/// use radixmap::{rule::RadixRule, node::RadixNode};
///
/// fn main() -> RadixResult<()> {
///     assert_eq!(format!("{:?}", RadixNode::<'_, ()>::from(RadixRule::new_plain(r"/api")?)), r"Plain(/api)".to_string());
///     assert_eq!(format!("{:?}", RadixNode::<'_, ()>::from(RadixRule::new_regex(r"{id:\d+}")?)), r"Regex({id:\d+})".to_string());
///     assert_eq!(format!("{:?}", RadixNode::<'_, ()>::from(RadixRule::new_param(r":id")?)), r"Param(:id)".to_string());
///     assert_eq!(format!("{:?}", RadixNode::<'_, ()>::from(RadixRule::new_glob(r"*")?)), r"Glob(*)".to_string());
///
///     Ok(())
/// }
/// ```
impl<'a, V> Debug for RadixNode<'a, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.rule.fmt(f)
    }
}

/// Create an empty node
impl<'a, V> Default for RadixNode<'a, V> {
    fn default() -> Self {
        Self { path: "", data: None, rule: RadixRule::default(), next: RadixPack::default() }
    }
}

impl<'a, V: Clone> Clone for RadixNode<'a, V> {
    fn clone(&self) -> Self {
        Self {
            path: self.path.clone(),
            data: self.data.clone(),
            rule: self.rule.clone(),
            next: self.next.clone(),
        }
    }
}

// -----------------------------------------------------------------------------

pub struct Keys<'a, V> {
    iter: Iter<'a, V>
}

impl<'a, V> From<NodeRef<'a, V>> for Keys<'a, V> {
    fn from(value: NodeRef<'a, V>) -> Self {
        Self { iter: Iter::from(value) }
    }
}

impl<'a, V> Iterator for Keys<'a, V> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().and_then(|node| Some(node.path))
    }
}

// -----------------------------------------------------------------------------

pub struct Values<'a, V> {
    iter: Iter<'a, V>
}

impl<'a, V> From<NodeRef<'a, V>> for Values<'a, V> {
    fn from(value: NodeRef<'a, V>) -> Self {
        Self { iter: Iter::from(value) }
    }
}

impl<'a, V> Iterator for Values<'a, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().and_then(|node| node.data.as_ref())
    }
}