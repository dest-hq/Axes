#![cfg_attr(feature = "no_std", no_std)]
mod engine;
pub use engine::*;
mod computed;
pub use computed::*;
mod styles;
pub use styles::*;
mod style;
pub use style::*;
mod tree;
pub use tree::*;

pub type NodeId = usize;

#[cfg(feature = "no_std")]
extern crate alloc;
#[cfg(feature = "no_std")]
#[allow(unused_imports)]
use alloc::vec; // HACK: for some reason the vec! macro
// doesn't make rustc think a crate is used

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut tree = LayoutTree::new();

        let root = tree.new_child(Style {
            size: Size {
                height: Units::Percentage(0.5),
                width: Units::Percentage(0.5),
            },
            gap: Size {
                width: Units::Pixels(30.0),
                height: Units::Pixels(0.0),
            },
            direction: Direction::Row,
            ..Default::default()
        });

        let child1 = tree.new_child(Style {
            size: Size {
                width: Units::Percentage(1.0),
                height: Units::Pixels(300.0),
            },
            ..Default::default()
        });

        let child2 = tree.new_child(Style {
            size: Size {
                width: Units::Pixels(300.0),
                height: Units::Percentage(1.0),
            },
            margin: Margin {
                top: 20.0,
                left: 20.0,
                right: 0.0,
                bottom: 0.0,
            },
            max_size: Some(Size {
                width: Units::Pixels(200.0),
                height: Units::Pixels(300.0),
            }),
            min_size: Some(Size {
                width: Units::Pixels(300.0),
                height: Units::Pixels(40.0),
            }),
            ..Default::default()
        });

        // Set parent to child
        tree.add_children(root, vec![child1, child2].as_slice());

        let mut engine = LayoutEngine::new();

        engine.compute(&tree, root, 900.0, 900.0);

        assert_eq!(engine.computed[0].x, 0.0);
        assert_eq!(engine.computed[0].y, 0.0);

        assert_eq!(engine.computed[1].height, 300.0);
        assert_eq!(engine.computed[1].width, 450.0);
        assert_eq!(engine.computed[1].x, 0.0);
        assert_eq!(engine.computed[1].y, 0.0);

        assert_eq!(engine.computed[2].height, 300.0);
        assert_eq!(engine.computed[2].width, 200.0);
        assert_eq!(engine.computed[2].y, 20.0);
        assert_eq!(engine.computed[2].x, 500.0);
    }
}
