use crate::node::*;
use crate::tree::Tree;
use crate::NodeId;

// todo: document this

pub struct AncestorsMut<'a, T> {
    node_id: Option<NodeId>,
    tree: *mut Tree<T>,
    _marker: PhantomData<&'a mut ()>,
}

impl<'a, T> AncestorsMut<'a, T> {
    pub(crate) fn new(node_id: Option<NodeId>, tree: &'a mut Tree<T>) -> AncestorsMut<T> {
        AncestorsMut { node_id, tree, _marker: Default::default() }
    }
}

impl<'a, T: 'a> Iterator for AncestorsMut<'a, T> {
    type Item = NodeMut<'a, T>;

    fn next(&mut self) -> Option<NodeMut<'a, T>> {
        let tree_ref = unsafe {
            &mut *self.tree as &'a mut Tree<T>
        };
        self.node_id
            .take()
            .and_then(|node_id| tree_ref.get_node_relatives(node_id).parent)
            .map(move |id| {
                self.node_id = Some(id);
                NodeMut::new(id, tree_ref)
            })
    }
}

use std::marker::PhantomData;

// possibly re-name this, not sure how I feel about it
pub struct NextSiblingsMut<'a, T> {
    node_id: Option<NodeId>,
    tree: *mut Tree<T>,
    _marker: PhantomData<&'a mut ()>,
}

impl<'a, T> NextSiblingsMut<'a, T> {
    pub(crate) fn new(node_id: Option<NodeId>, tree: *mut Tree<T>) -> NextSiblingsMut<'a, T> {
        NextSiblingsMut { node_id, tree, _marker: Default::default() }
    }
}

impl<'a, T: 'a> Iterator for NextSiblingsMut<'a, T> {
    type Item = NodeMut<'a, T>;

    fn next(&mut self) -> Option<NodeMut<'a, T>> {
        self.node_id.take().map(|node_id| {
            // Unsafety: YOLO
            let tree_ref = unsafe {
                &mut *self.tree as &'a mut Tree<T>
            };
            self.node_id = tree_ref.get_node_relatives(node_id).next_sibling;
            NodeMut::new(node_id, tree_ref)
        })
    }
}

/// Depth-first pre-order iterator
pub struct PreOrder<'a, T> {
    start: Option<NodeMut<'a, T>>,
    children: Vec<NextSiblingsMut<'a, T>>,
    tree: *mut Tree<T>,
    _marker: PhantomData<&'a mut ()>,
}

impl<'a, T> PreOrder<'a, T> {
    pub(crate) fn new(node: &NodeMut<'a, T>, tree: &'a mut Tree<T>) -> PreOrder<'a, T> {
        let children = vec![];
        let tree_ptr = tree as *mut Tree<T>;
        let start = tree.get_mut(node.node_id());
        PreOrder {
            start,
            children,
            tree: tree_ptr,
            _marker: Default::default(),
        }
    }
}

impl<'a, T: 'a> Iterator for PreOrder<'a, T> {
    type Item = NodeMut<'a, T>;

    fn next(&mut self) -> Option<NodeMut<'a, T>> {
        if let Some(mut node) = self.start.take() {
            let first_child_id = node.first_child().map(|child_ref| child_ref.node_id());
            self.children
                .push(NextSiblingsMut::new(first_child_id, self.tree));
            Some(node)
        } else {
            while self.children.len() > 0 {
                if let Some(mut node_ref) = self.children.last_mut().and_then(Iterator::next) {
                    if let Some(first_child) = node_ref.first_child() {
                        self.children
                            .push(NextSiblingsMut::new(Some(first_child.node_id()), self.tree));
                    }
                    return Some(node_ref);
                }
                self.children.pop();
            }
            None
        }
    }
}

/// Depth-first post-order iterator
pub struct PostOrderMut<'a, T> {
    nodes: Vec<(NodeMut<'a, T>, NextSiblingsMut<'a, T>)>,
    tree: *mut Tree<T>,
    _marker: PhantomData<&'a mut T>,
}

impl<'a, T> PostOrderMut<'a, T> {
    pub(crate) fn new(node: &NodeMut<'a, T>, tree: &'a mut Tree<T>) -> PostOrderMut<'a, T> {
        let tree_ptr = tree as *mut Tree<T>;
        let mut node = tree
            .get_mut(node.node_id())
            .expect("getting node of node ref id");
        let first_child_id = node.first_child().map(|first_child| first_child.node_id());
        let nodes = vec![(node, NextSiblingsMut::new(first_child_id, tree_ptr))];
        PostOrderMut { nodes, tree: tree_ptr, _marker: Default::default() }
    }
}

impl<'a, T> Iterator for PostOrderMut<'a, T> {
    type Item = NodeMut<'a, T>;

    fn next(&mut self) -> Option<NodeMut<'a, T>> {
        if let Some((node, mut children)) = self.nodes.pop() {
            if let Some(next) = children.next() {
                self.nodes.push((node, children));
                let mut node_id = next.node_id();
                loop {
                    let tree_ref = unsafe {
                        &mut *self.tree as &'a mut Tree<T>
                    };
                    let node = tree_ref.get_mut(node_id).expect("getting node of node ref id");
                    if let Some(first_child) = node.as_ref().first_child() {
                        node_id = first_child.node_id();
                        let mut children = NextSiblingsMut::new(Some(node_id), self.tree);
                        assert!(children.next().is_some(), "skipping first child");
                        self.nodes.push((node, children));
                    } else {
                        break Some(node);
                    }
                }
            } else {
                Some(node)
            }
        } else {
            None
        }
    }
}

/// Depth-first level-order iterator
pub struct LevelOrder<'a, T> {
    start: NodeMut<'a, T>,
    levels: Vec<(NodeId, NextSiblingsMut<'a, T>)>,
    tree: *mut Tree<T>,
}

impl<'a, T> LevelOrder<'a, T> {
    pub(crate) fn new(node: NodeMut<'a, T>, tree: &'a mut Tree<T>) -> LevelOrder<'a, T> {
        let tree_ptr = tree as *mut Tree<T>;
        let start = tree
            .get_mut(node.node_id())
            .expect("getting node of node ref id");
        let levels = Vec::new();
        LevelOrder {
            start,
            levels,
            tree: tree_ptr,
        }
    }
}

impl<'a, T> Iterator for LevelOrder<'a, T> {
    type Item = NodeMut<'a, T>;

    fn next(&mut self) -> Option<NodeMut<'a, T>> {
        let tree_ref = unsafe {
            &mut *self.tree as &'a mut Tree<T>
        };
        if self.levels.is_empty() {
            let first_child_id = self.start.first_child().map(|child| child.node_id());
            self.levels.push((
                self.start.node_id(),
                NextSiblingsMut::new(first_child_id, self.tree),
            ));
            let node = tree_ref
                .get_mut(self.start.node_id())
                .expect("getting node of existing node ref id");
            Some(node)
        } else {
            let mut on_level = self.levels.len();
            let next_level = on_level + 1;
            let mut level = on_level;
            while level > 0 {
                if let Some(mut node) = self.levels.last_mut().expect("non-empty levels").1.next() {
                    if level >= on_level {
                        return Some(node);
                    } else {
                        let first_child_id = node.first_child().map(|child| child.node_id());
                        self.levels
                            .push((node.node_id(), NextSiblingsMut::new(first_child_id, self.tree)));
                        level += 1;
                    }
                } else {
                    let (node_id, _) = self.levels.pop().expect("on level > 0");
                    if let Some(mut next) = self.levels.last_mut().and_then(|level| level.1.next()) {
                        let first_child_id = next.first_child().map(|child| child.node_id());
                        self.levels
                            .push((next.node_id(), NextSiblingsMut::new(first_child_id, self.tree)));
                    } else {
                        if level == 1 {
                            if on_level < next_level {
                                on_level += 1;
                                let node = tree_ref
                                    .get(node_id)
                                    .expect("getting node of existing node ref id");
                                let first_child_id =
                                    node.first_child().map(|child| child.node_id());
                                self.levels.push((
                                    node.node_id(),
                                    NextSiblingsMut::new(first_child_id, self.tree),
                                ));
                            } else {
                                break;
                            }
                        } else {
                            level -= 1;
                        }
                    }
                }
            }
            None
        }
    }
}
