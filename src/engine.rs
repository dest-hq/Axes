use crate::{
    ComputedLayout, Direction, HorizontalAlign, LayoutTree, NodeId, Size, Units, VerticalAlign,
};
#[cfg(feature = "no_std")]
use alloc::vec::Vec;
#[cfg(not(feature = "no_std"))]
use std::fmt::Debug;

pub struct LayoutEngine {
    computed: Vec<ComputedLayout>,
}

#[cfg(not(feature = "no_std"))]
impl Debug for LayoutEngine {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "Layout Engine")?;

        let len = self.computed.len();
        for (i, node) in self.computed.iter().enumerate() {
            let is_last = i == len - 1;
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

    pub fn get(&mut self, id: NodeId) -> Option<&ComputedLayout> {
        self.computed.get(id)
    }

    pub fn compute(&mut self, tree: &LayoutTree, root: NodeId, width: f32, height: f32) {
        // Resize to allocate the necessary memory
        self.computed
            .resize(tree.styles.len(), ComputedLayout::default());

        self.layout_node(tree, root, (width, height), 0.0, 0.0);
    }

    fn layout_node(
        &mut self,
        tree: &LayoutTree,
        node: NodeId,
        available: (f32, f32),
        x: f32,
        y: f32,
    ) {
        let style = &tree.styles[node];

        // Resolve spacing first, as it doesn't depend on size
        let (horizontal_spacing, vertical_spacing) = resolve_size(available, &style.gap);

        // Resolve size for the node only once
        let (mut width, mut height) = resolve_size(available, &style.size);

        // Min and Max size adjustments
        let (min_width, min_height) = resolve_size(
            available,
            style.min_size.as_ref().unwrap_or(&Size {
                width: 0.0.into(),
                height: 0.0.into(),
            }),
        );

        let (max_width, max_height) =
            resolve_size(available, style.max_size.as_ref().unwrap_or(&style.size));

        // Apply min/max constraints
        width = width.max(min_width).min(max_width) - (style.padding.left + style.padding.right);
        height =
            height.max(min_height).min(max_height) - (style.padding.top + style.padding.bottom);

        let (x_offset, y_offset) = get_offset(
            (width, height),
            available,
            &style.vertical_align,
            &style.horizontal_align,
        );

        // Save the computed layout for the node
        self.computed[node] = ComputedLayout {
            x: (x + style.margin.left - style.margin.right) + x_offset,
            y: (y + style.margin.top - style.margin.bottom) + y_offset,
            width,
            height,
        };

        // Initialize cursor for positioning child nodes
        let mut cursor = 0.0;

        // Layout the child nodes
        for (i, &child) in tree.children[node].iter().enumerate() {
            let (child_x, child_y) = if i == 0 {
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

            // Recursively layout the child node
            self.layout_node(
                tree,
                child,
                (width, height),
                child_x + style.padding.left,
                child_y + style.padding.top,
            );

            // Update cursor position based on the direction
            cursor += match style.direction {
                Direction::Column => self.computed[child].height,
                Direction::Row => self.computed[child].width,
            };
        }
    }
}

fn get_offset(
    node_size: (f32, f32),
    available: (f32, f32),
    vertical_align: &VerticalAlign,
    horizontal_align: &HorizontalAlign,
) -> (f32, f32) {
    let x = match horizontal_align {
        HorizontalAlign::Left => 0.0,
        HorizontalAlign::Center => (available.0 - node_size.0) / 2.0,
        HorizontalAlign::Right => available.0 - node_size.0,
    };

    let y = match vertical_align {
        VerticalAlign::Top => 0.0,
        VerticalAlign::Center => (available.1 - node_size.1) / 2.0,
        VerticalAlign::Bottom => available.1 - node_size.1,
    };

    (x, y)
}

fn resolve_size(available: (f32, f32), size: &Size) -> (f32, f32) {
    let width = match size.width {
        Units::Pixels(px) => px,
        Units::Percentage(p) => p * available.0,
    };

    let height = match size.height {
        Units::Pixels(px) => px,
        Units::Percentage(p) => p * available.1,
    };

    (width, height)
}
