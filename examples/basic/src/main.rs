use axes::{Direction, LayoutEngine, LayoutTree, Size, Style, Units};

fn main() {
    let mut tree = LayoutTree::new();

    let root = tree.new_child(Style {
        size: Size::default(),
        gap: Size {
            width: Units::Pixels(30.0),
            height: Units::Pixels(0.0),
        },
        direction: Direction::Row,
        ..Default::default()
    });

    let child1 = tree.new_child(Style {
        size: Size {
            width: Units::Pixels(300.0),
            height: Units::Pixels(300.0),
        },
        ..Default::default()
    });

    let child2 = tree.new_child(Style {
        size: Size {
            width: Units::Pixels(300.0),
            height: Units::Pixels(300.0),
        },
        ..Default::default()
    });

    // Set parent to child
    tree.add_children(root, vec![child1, child2]);

    let mut engine = LayoutEngine::new();

    engine.compute(
        &tree,
        root,
        Size {
            width: Units::Pixels(900.0),
            height: Units::Pixels(900.0),
        },
    );

    println!("{:?}", engine);
}
