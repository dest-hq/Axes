use crate::{ComputedLayout, Direction, LayoutTree, NodeId, Size, Units};

#[cfg(feature = "no_std")]
use alloc::vec::Vec;
#[cfg(not(feature = "no_std"))]
use std::fmt::Debug;

pub struct LayoutEngine {
    pub computed: Vec<ComputedLayout>,
}

#[cfg(not(feature = "no_std"))]
impl Debug for LayoutEngine {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "Layout Engine")?;

        let len = self.computed.len();
        for (i, node) in self.computed.iter().enumerate() {
            let is_last = if i == len - 1 { true } else { false };
            let connector = if is_last { "└─" } else { "├─" };

            writeln!(
                f,
                "{} Node {}: x: {}, y: {}, w: {}, h: {}",
                connector,
                i + 1,
                node.x,
                node.y,
                node.width,
                node.height
            )?
        }
        Ok(())
    }
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
        let style = &tree.styles[node];

        let (horizontal_spacing, vertical_spacing) = resolve_size(&available, &style.gap);

        let (width, height) = resolve_size(&available, &style.size);

        let (min_size, max_size) =
            if let (Some(min_size), Some(max_size)) = (style.min_size, style.max_size) {
                (min_size, max_size)
            } else {
                (Size::default(), style.size)
            };

        let (min_width, min_height) = resolve_size(&available, &min_size);
        let (max_width, max_height) = resolve_size(&available, &max_size);

        // Set the compiled info
        self.computed[node] = ComputedLayout {
            x: x + style.margin.left - style.margin.right,
            y: y + style.margin.top - style.margin.bottom,
            width: width.max(min_width).min(max_width),
            height: height.max(min_height).min(max_height),
        };

        // Position of node
        let mut cursor = 0.0;

        for (i, &child) in tree.children[node].iter().enumerate() {
            let (x, y) = if i == 0 {
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
