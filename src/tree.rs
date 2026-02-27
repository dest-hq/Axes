#[cfg(feature = "no_std")]
use alloc::vec::Vec;
use smallvec::SmallVec;

use crate::{NodeId, Style};

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

    /// Set the parent to a node
    pub fn add_children(&mut self, parent: NodeId, children: Vec<NodeId>) {
        for child in children {
            self.children[parent].push(child);
            self.parents[child] = Some(parent)
        }
    }
}
