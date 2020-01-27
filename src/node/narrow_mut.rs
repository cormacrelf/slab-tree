use crate::behaviors::RemoveBehavior;
use crate::node::NodeRef;
use crate::node::NodeMut;
use crate::NodeId;
use crate::iter_mut::NextSiblingsMut;

///
/// A mutable reference to a given `Node`'s data and its children.
/// Access is limited to its children so it cannot invalidate a mutating iterator.
///
#[derive(Debug, PartialEq)]
pub struct NarrowMut<'a, T> {
    inner: NodeMut<'a, T>,
}

impl<'a, T> From<NodeMut<'a, T>> for NarrowMut<'a, T> {
    fn from(other: NodeMut<'a, T>) -> Self {
        NarrowMut::new(other)
    }
}

impl<'a, T: 'a> NarrowMut<'a, T> {
    pub(crate) fn new(inner: NodeMut<'a, T>) -> NarrowMut<'a, T> {
        NarrowMut { inner }
    }

    pub fn children_mut(&mut self) -> NextSiblingsMut<'a, T> {
        self.inner.children_mut()
    }

    ///
    /// Returns the `NodeId` that identifies this `Node` in the tree.
    ///
    pub fn node_id(&self) -> NodeId {
        self.inner.node_id()
    }

    ///
    /// Returns a mutable reference to the data contained by the given `Node`.
    ///
    pub fn data(&self) -> &T {
        self.inner.data()
    }

    ///
    /// Returns a mutable reference to the data contained by the given `Node`.
    ///
    pub fn data_mut(&mut self) -> &mut T {
        self.inner.data_mut()
    }

    ///
    /// Returns a `NodeMut` pointing to this `Node`'s first child.  Returns a `Some`-value
    /// containing the `NodeMut` if this `Node` has a first child; otherwise returns a `None`.
    ///
    pub fn first_child(&mut self) -> Option<NarrowMut<T>> {
        self.inner.first_child().map(NarrowMut::new)
    }

    ///
    /// Returns a `NodeMut` pointing to this `Node`'s last child.  Returns a `Some`-value
    /// containing the `NodeMut` if this `Node` has a last child; otherwise returns a `None`.
    ///
    pub fn last_child(&mut self) -> Option<NarrowMut<T>> {
        self.inner.last_child().map(NarrowMut::new)
    }

    ///
    /// Appends a new `Node` as this `Node`'s last child (and first child if it has none).
    /// Returns a `NodeMut` pointing to the newly added `Node`.
    ///
    pub fn append(&mut self, data: T) -> NarrowMut<T> {
        NarrowMut::new(self.inner.append(data))
    }

    ///
    /// Prepends a new `Node` as this `Node`'s first child (and last child if it has none).
    /// Returns a `NodeMut` pointing to the newly added `Node`.
    ///
    pub fn prepend(&mut self, data: T) -> NarrowMut<T> {
        NarrowMut::new(self.inner.prepend(data))
    }

    ///
    /// Remove the first child of this `Node` and return the data that child contained.
    /// Returns a `Some`-value if this `Node` has a child to remove; returns a `None`-value
    /// otherwise.
    ///
    /// Children of the removed `Node` can either be dropped with `DropChildren` or orphaned with
    /// `OrphanChildren`.
    ///
    pub fn remove_first(&mut self, behavior: RemoveBehavior) -> Option<T> {
        self.inner.remove_first(behavior)
    }

    ///
    /// Remove the first child of this `Node` and return the data that child contained.
    /// Returns a `Some`-value if this `Node` has a child to remove; returns a `None`-value
    /// otherwise.
    ///
    /// Children of the removed `Node` can either be dropped with `DropChildren` or orphaned with
    /// `OrphanChildren`.
    ///
    pub fn remove_last(&mut self, behavior: RemoveBehavior) -> Option<T> {
        self.inner.remove_last(behavior)
    }

    ///
    /// Returns a `NodeRef` pointing to this `NodeMut`.
    ///
    /// ```
    /// use slab_tree::tree::TreeBuilder;
    ///
    /// let mut tree = TreeBuilder::new().with_root(1).build();
    /// let mut root = tree.root_mut().expect("root doesn't exist?");
    /// root.append(2);
    ///
    /// let root = root.as_ref();
    ///
    /// assert_eq!(root.data(), &1);
    /// ```
    ///
    pub fn as_ref(&self) -> NodeRef<T> {
        self.inner.as_ref()
    }

}
