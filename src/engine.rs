use crate::{
    ComputedLayout, Direction, HorizontalAlign, LayoutTree, NodeId, Size, Units, VerticalAlign,
};
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
        let (horizontal_spacing, vertical_spacing) = resolve_size(
            &tree,
            &Size {
                width: available.0.into(),
                height: available.1.into(),
            },
            &style.gap,
            node,
        );

        // Resolve size for the node only once
        let (width, height) = resolve_size(
            &tree,
            &Size {
                width: available.0.into(),
                height: available.1.into(),
            },
            &style.size,
            node,
        );

        // Min and Max size adjustments
        let (min_width, min_height) = resolve_size(
            &tree,
            &Size {
                width: available.0.into(),
                height: available.1.into(),
            },
            &style.min_size.unwrap_or_default(),
            node,
        );
        let (max_width, max_height) = resolve_size(
            &tree,
            &Size {
                width: available.0.into(),
                height: available.1.into(),
            },
            &style.max_size.unwrap_or(style.size),
            node,
        );

        // Apply min/max constraints
        let width = width.max(min_width).min(max_width);
        let height = height.max(min_height).min(max_height);

        let (x_offset, y_offset) = get_offset(
            &(width, height),
            &available,
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
                (
                    width - (style.padding.left + style.padding.right),
                    height - (style.padding.top + style.padding.bottom),
                ),
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
    node_size: &(f32, f32),
    available: &(f32, f32),
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

fn resolve_size(tree: &LayoutTree, available: &Size, size: &Size, node: NodeId) -> (f32, f32) {
    let width = match size.width {
        Units::Pixels(px) => px,
        Units::Percentage(p) => {
            match available.width {
                Units::Pixels(px) => p * px,
                Units::Percentage(_) => {
                    // Parent doesn't have fixed size, need to resolve recursively
                    if let Some(parent_id) = tree.parents[node] {
                        let parent_style = &tree.styles[parent_id];
                        let parent_width =
                            resolve_size(tree, available, &parent_style.size, parent_id).0;
                        p * parent_width
                    } else {
                        0.0
                    }
                }
            }
        }
    };

    let height = match size.height {
        Units::Pixels(px) => px,
        Units::Percentage(p) => match available.height {
            Units::Pixels(px) => p * px,
            Units::Percentage(_) => {
                if let Some(parent_id) = tree.parents[node] {
                    let parent_style = &tree.styles[parent_id];
                    let parent_height =
                        resolve_size(tree, available, &parent_style.size, parent_id).1;
                    p * parent_height
                } else {
                    0.0
                }
            }
        },
    };
    (width, height)
}
