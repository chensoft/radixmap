use super::def::*;
use super::iter::*;
use super::pack::*;
use super::rule::*;

/// The basic element inside a tree
pub struct RadixNode<'a, V> {
    /// The key of the radix map, valid only in a leaf node
    pub path: &'a str,

    /// The value of the radix map, valid only in a leaf node
    pub data: Option<V>,

    /// The pattern used for matching, supports named params, regex and glob
    pub rule: RadixRule<'a>,

    /// Node's children
    pub next: RadixPack<'a, V>,
}

impl<'a, V> RadixNode<'a, V> {
    /// Check if the node is a leaf node
    #[inline]
    pub fn is_leaf(&self) -> bool {
        self.next.is_empty()
    }

    /// Check if the node has no data
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_none()
    }

    /// An iterator for node
    ///
    /// ```
    /// use radixmap::{node::RadixNode, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut node = RadixNode::default();
    ///     node.insert("/api", "/api")?;
    ///     node.insert("/api/v1", "/api/v1")?;
    ///     node.insert("/api/v2", "/api/v2")?;
    ///
    ///     let mut iter = node.iter();
    ///
    ///     assert_eq!(iter.next().unwrap().path, "/api");
    ///     assert_eq!(iter.next().unwrap().path, "/api/v1");
    ///     assert_eq!(iter.next().unwrap().path, "/api/v1");
    ///     assert_eq!(iter.next(), None);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn iter(&self) -> Iter<'_, V> {
        Iter::from(self)
    }

    /// A mutable iterator for node
    ///
    /// ```
    /// use radixmap::{node::RadixNode, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut node = RadixNode::default();
    ///     node.insert("/api", 0)?;
    ///     node.insert("/api/v1", 1)?;
    ///     node.insert("/api/v2", 2)?;
    ///
    ///     let mut iter = node.iter_mut();
    ///
    ///     assert_eq!(iter.next().unwrap().path, "/api");
    ///     assert_eq!(iter.next().unwrap().path, "/api/v1");
    ///     assert_eq!(iter.next().unwrap().path, "/api/v1");
    ///     assert_eq!(iter.next(), None);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn iter_mut(&self) -> Iter<'_, V> {
        todo!()
    }

    /// Iterator adapter for path
    ///
    /// ```
    /// use radixmap::{node::RadixNode, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut node = RadixNode::default();
    ///     node.insert("/api", 0)?;
    ///     node.insert("/api/v1", 1)?;
    ///     node.insert("/api/v2", 2)?;
    ///
    ///     let mut iter = node.keys();
    ///
    ///     assert_eq!(iter.next().unwrap(), "/api");
    ///     assert_eq!(iter.next().unwrap(), "/api/v1");
    ///     assert_eq!(iter.next().unwrap(), "/api/v1");
    ///     assert_eq!(iter.next(), None);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn keys(&self) -> Keys<'_, V> {
        Keys::from(self)
    }

    /// Iterator adapter for data
    ///
    /// ```
    /// use radixmap::{node::RadixNode, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut node = RadixNode::default();
    ///     node.insert("/api", 0)?;
    ///     node.insert("/api/v1", 1)?;
    ///     node.insert("/api/v2", 2)?;
    ///
    ///     let mut iter = node.values();
    ///
    ///     assert_eq!(iter.next().unwrap(), 0);
    ///     assert_eq!(iter.next().unwrap(), 1);
    ///     assert_eq!(iter.next().unwrap(), 2);
    ///     assert_eq!(iter.next(), None);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn values(&self) -> Values<'_, V> {
        Values::from(self)
    }

    /// Mutable iterator adapter for data
    ///
    /// ```
    /// use radixmap::{node::RadixNode, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut node = RadixNode::default();
    ///     node.insert("/api", 0)?;
    ///     node.insert("/api/v1", 1)?;
    ///     node.insert("/api/v2", 2)?;
    ///
    ///     let mut iter = node.values_mut();
    ///
    ///     assert_eq!(iter.next().unwrap(), 0);
    ///     assert_eq!(iter.next().unwrap(), 1);
    ///     assert_eq!(iter.next().unwrap(), 2);
    ///     assert_eq!(iter.next(), None);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn values_mut(&mut self) -> Values<'_, V> {
        todo!()
    }

    /// Inserts a path and data into this node, which serves as the root node for the insertion.
    /// The method sequentially extracts path fragments and positions each node appropriately,
    /// ensuring that nodes with a common prefix share a single node in the tree
    pub fn insert(&mut self, path: &'a str, data: V) -> RadixResult<Option<V>> {
        let mut frag = path;

        loop {
            // extract the next path fragment and insert it via pack
            let next = RadixRule::try_from(frag)?;
            let used = next.origin();
            let slot = self.next.insert(next)?;

            // encountering a leaf node indicates completion of insertion
            if used.len() == frag.len() {
                let prev = slot.data.take();
                slot.path = path;
                slot.data = Some(data);
                return Ok(prev);
            }

            frag = &frag[used.len()..];
        }
    }

    /// Divide the node into two parts
    ///
    /// ```
    /// use radixmap::{node::RadixNode, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut node = RadixNode::try_from(("/api", 12345))?;
    ///
    ///     assert_eq!(node.rule, "/api");
    ///     assert_eq!(node.data, Some(12345));
    ///
    ///     let leaf = node.divide(1)?;
    ///
    ///     assert_eq!(node.rule, "/");
    ///     assert_eq!(node.data, None);
    ///     assert_eq!(leaf.rule, "api");
    ///     assert_eq!(leaf.data, Some(12345));
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

    /// Clear the nodes but preserve its capacity
    ///
    /// ```
    /// use radixmap::{node::RadixNode, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut node = RadixNode::try_from(("/api", ()))?;
    ///     node.insert("/api/v1", ())?;
    ///
    ///     assert!(!node.is_leaf());
    ///     assert!(!node.is_empty());
    ///
    ///     node.clear();
    ///
    ///     assert!(node.is_leaf());
    ///     assert!(node.is_empty());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn clear(&mut self) {
        self.path = "";
        self.data = None;
        self.rule = RadixRule::default();
        self.next.clear();
    }
}

/// Create a node from a rule
///
/// ```
/// use radixmap::{node::RadixNode, rule::RadixRule, RadixResult};
///
/// fn main() -> RadixResult<()> {
///     assert_eq!(RadixNode::<'_, ()>::from(RadixRule::try_from("/api")?).rule, "/api");
///     assert_eq!(RadixNode::<'_, ()>::from(RadixRule::try_from(":id")?).rule, ":id");
///
///     Ok(())
/// }
/// ```
impl<'a, V> From<RadixRule<'a>> for RadixNode<'a, V> {
    fn from(rule: RadixRule<'a>) -> Self {
        Self { path: "", data: None, rule, next: Default::default() }
    }
}

/// Create a node from (path, data)
///
/// ```
/// use radixmap::{node::RadixNode, RadixResult};
///
/// fn main() -> RadixResult<()> {
///     assert_eq!(RadixNode::try_from(("/api", ()))?.rule, "/api");
///     assert_eq!(RadixNode::try_from((":id", ()))?.rule, ":id");
///
///     Ok(())
/// }
/// ```
impl<'a, V> TryFrom<(&'a str, V)> for RadixNode<'a, V> {
    type Error = RadixError;

    fn try_from((path, data): (&'a str, V)) -> RadixResult<Self> {
        Ok(Self { path, data: Some(data), rule: RadixRule::try_from(path)?, next: Default::default() })
    }
}

/// Default trait
/// ```
/// use radixmap::{node::RadixNode, RadixResult};
///
/// fn main() -> RadixResult<()> {
///     let mut node = RadixNode::default();
///     assert!(node.insert("/api", ()).is_ok());
///
///     Ok(())
/// }
/// ```
impl<'a, V> Default for RadixNode<'a, V> {
    fn default() -> Self {
        Self { path: "", data: None, rule: RadixRule::default(), next: RadixPack::default() }
    }
}

/// Debug trait
///
/// ```
/// use radixmap::{node::RadixNode, RadixResult};
///
/// fn main() -> RadixResult<()> {
///     assert_eq!(format!("{:?}", RadixNode::try_from((r"/api", ()))?).as_str(), r"Plain(/api)");
///     assert_eq!(format!("{:?}", RadixNode::try_from((r":id", ()))?).as_str(), r"Param(:id)");
///     assert_eq!(format!("{:?}", RadixNode::try_from((r"{id:\d+}", ()))?).as_str(), r"Regex({id:\d+})");
///     assert_eq!(format!("{:?}", RadixNode::try_from((r"*", ()))?).as_str(), r"Glob(*)");
///
///     Ok(())
/// }
/// ```
impl<'a, V> Debug for RadixNode<'a, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.rule.fmt(f)
    }
}

/// Clone trait
/// ```
/// use radixmap::{node::RadixNode, RadixResult};
///
/// fn main() -> RadixResult<()> {
///     let mut node_a = RadixNode::try_from(("/api", 123))?;
///     let mut node_b = node_a.clone();
///
///     assert_eq!(node_a.path, node_b.path);
///     assert_eq!(node_a.data, node_b.data);
///     assert_eq!(node_a.rule, node_b.rule);
///
///     Ok(())
/// }
/// ```
impl<'a, V: Clone> Clone for RadixNode<'a, V> {
    fn clone(&self) -> Self {
        Self {
            path: self.path,
            data: self.data.clone(),
            rule: self.rule.clone(),
            next: self.next.clone(),
        }
    }
}