use crate::{ComputedLayout, Direction, LayoutTree, NodeId, Size, Units};

#[cfg(feature = "no_std")]
use alloc::vec::Vec;

pub struct LayoutEngine {
    pub computed: Vec<ComputedLayout>,
}

impl LayoutEngine {
    pub fn new() -> Self {
        LayoutEngine {
            computed: Vec::new(),
        }
    }

    pub fn compute(&mut self, tree: &LayoutTree, root: NodeId, available: Size) {
        // Resize to allocate the necessary memory
        self.computed
            .resize(tree.styles.len(), ComputedLayout::default());

        self.layout_node(tree, root, available, 0.0, 0.0);
    }

    fn layout_node(&mut self, tree: &LayoutTree, node: NodeId, available: Size, x: f32, y: f32) {
        let style = tree.styles[node].clone();

        let (horizontal_spacing, vertical_spacing) = resolve_size(&available, &style.gap);

        let (width, height) = resolve_size(&available, &style.size);

        // Set the compiled info
        // I set the first node as postion of 0.0 because he's a vec that contains childs or just an single node
        self.computed[node] = ComputedLayout {
            x: x + style.margin.left - style.margin.right,
            y: y + style.margin.top - style.margin.bottom,
            width,
            height,
        };

        // Position of node
        let mut cursor = 0.0;

        for (i, &child) in tree.children[node].iter().enumerate() {
            let (mut x, mut y) = if i == 0 {
                match style.direction {
                    Direction::Column => (x, y + cursor),
                    Direction::Row => (x + cursor, y),
                }
            } else {
                match style.direction {
                    Direction::Column => (x, y + cursor + vertical_spacing),
                    Direction::Row => (x + cursor + horizontal_spacing, y),
                }
            };

            // Margin
            x += style.margin.left - style.margin.right;
            y += style.margin.top - style.margin.bottom;

            self.layout_node(tree, child, style.size, x, y);

            let child_layout = self.computed[child];

            cursor += match style.direction {
                Direction::Column => child_layout.height,
                Direction::Row => child_layout.width,
            };
        }
    }
}
fn resolve_size(_available: &Size, size: &Size) -> (f32, f32) {
    // TODO: Implement percent measure

    let width = match size.width {
        Units::Pixels(px) => px,
        Units::Percentage(_) => 0.0,
    };

    let height = match size.height {
        Units::Pixels(px) => px,
        Units::Percentage(_) => 0.0,
    };

    (width, height)
}
