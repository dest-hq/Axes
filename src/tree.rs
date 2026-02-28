#[cfg(feature = "no_std")]
use alloc::vec::Vec;
use smallvec::SmallVec;

use crate::{NodeId, Style};

#[derive(Clone, Debug)]
pub struct LayoutTree {
    pub children: Vec<SmallVec<[NodeId; 6]>>,
    pub parents: Vec<Option<NodeId>>,
    pub styles: Vec<Style>,
}

impl LayoutTree {
    pub fn new() -> Self {
        Self {
            styles: Vec::new(),
            parents: Vec::new(),
            children: Vec::new(),
        }
    }

    /// Create a node without a parent
    pub fn new_child(&mut self, style: Style) -> NodeId {
        // Get current id
        let id = self.styles.len();

        // Push to tree
        {
            self.styles.push(style);
            self.parents.push(None);
            self.children.push(SmallVec::new());
        }

        id
    }

    /// Set the parent to nodes
    pub fn add_children(&mut self, parent: NodeId, children: &[NodeId]) {
        self.children[parent].extend_from_slice(children);
        for &child in children {
            self.parents[child] = Some(parent);
        }
    }

    /// Set the parent to a node
    pub fn add_child(&mut self, parent: NodeId, child: NodeId) {
        self.children[parent].push(child);
        self.parents[child] = Some(parent);
    }
}
