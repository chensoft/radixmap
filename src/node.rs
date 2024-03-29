use super::def::*;
use super::rule::*;
use super::pack::*;

/// The basic element inside a tree
pub struct RadixNode<'a, V> {
    pub rule: RadixRule<'a>,
    pub data: Option<V>, // todo Item -> path & data
    pub next: RadixPack<'a, V>,
}

/// Create an empty node
impl<'a, V> Default for RadixNode<'a, V> {
    fn default() -> Self {
        Self { rule: RadixRule::default(), data: None, next: RadixPack::default() }
    }
}

impl<'a, V> RadixNode<'a, V> {
    /// Create a radix node
    ///
    /// ```
    /// use radixmap::{rule::RadixRule, node::RadixNode};
    ///
    /// fn main() -> anyhow::Result<()> {
    ///     let node = RadixNode::new(RadixRule::new_plain(r"/api")?, Some(12345));
    ///
    ///     assert_eq!(node.rule.origin(), "/api");
    ///     assert_eq!(node.data, Some(12345));
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn new(rule: RadixRule<'a>, data: Option<V>) -> Self {
        Self { rule, data, next: RadixPack::default() }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_none()
    }

    pub fn insert(&mut self, path: &'a str, data: V) -> Result<Option<V>> {
        let next = RadixRule::extract(path)?;
        let edge = self.next.insert(next)?;

        if next.len() == path.len() {
            let prev = std::mem::take(&mut edge.data);
            edge.data = Some(data);
            return Ok(prev);
        }

        edge.insert(&path[next.len()..], data)
    }

    /// Divide the node into two parts
    ///
    /// ```
    /// use radixmap::{rule::RadixRule, node::RadixNode};
    ///
    /// fn main() -> anyhow::Result<()> {
    ///     let mut node = RadixNode::new(RadixRule::new_plain(r"/api")?, Some(12345));
    ///     let leaf = node.divide(1)?;
    ///
    ///     assert_eq!(node.rule.origin(), "/");
    ///     assert_eq!(node.data, None);
    ///     assert_eq!(leaf.rule.origin(), "api");
    ///     assert_eq!(leaf.data, Some(12345));
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn divide(&mut self, len: usize) -> Result<RadixNode<'a, V>> {
        Ok(RadixNode {
            rule: self.rule.divide(len)?,
            data: std::mem::take(&mut self.data),
            next: std::mem::take(&mut self.next),
        })
    }

    pub fn clear(&mut self) {
        self.rule = RadixRule::default();
        self.data = None;
        self.next.clear();
    }
}

/// Debug trait
///
/// ```
/// use radixmap::{rule::RadixRule, node::RadixNode};
///
/// fn main() -> anyhow::Result<()> {
///     assert_eq!(format!("{:?}", RadixNode::new(RadixRule::new_plain(r"/api")?, None::<()>)), r"Plain(/api)".to_string());
///     assert_eq!(format!("{:?}", RadixNode::new(RadixRule::new_regex(r"{id:\d+}")?, None::<()>)), r"Regex({id:\d+})".to_string());
///     assert_eq!(format!("{:?}", RadixNode::new(RadixRule::new_param(r":id")?, None::<()>)), r"Param(:id)".to_string());
///     assert_eq!(format!("{:?}", RadixNode::new(RadixRule::new_glob(r"*")?, None::<()>)), r"Glob(*)".to_string());
///
///     Ok(())
/// }
/// ```
impl<'a, V> Debug for RadixNode<'a, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.rule.fmt(f)
    }
}

impl<'a, V: Clone> Clone for RadixNode<'a, V> {
    fn clone(&self) -> Self {
        Self {
            rule: self.rule.clone(),
            data: self.data.clone(),
            next: self.next.clone(),
        }
    }
}