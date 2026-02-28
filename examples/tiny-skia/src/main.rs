use axes::{Direction, LayoutEngine, LayoutTree, Size as axesSize, Style, Units};
use tiny_skia::{Color, FillRule, Paint, PathBuilder, Pixmap, Rect, Transform};

fn main() {
    let mut tree = LayoutTree::new();

    let root = tree.new_child(Style {
        gap: axesSize {
            width: Units::Pixels(0.0),
            height: Units::Pixels(20.0),
        },
        direction: Direction::Column,
        ..Default::default()
    });

    let rect1 = tree.new_child(Style {
        size: axesSize {
            width: Units::Pixels(100.0),
            height: Units::Pixels(100.0),
        },
        ..Default::default()
    });

    let rect2 = tree.new_child(Style {
        size: axesSize {
            width: Units::Pixels(100.0),
            height: Units::Pixels(100.0),
        },
        ..Default::default()
    });

    // Set parent to child
    tree.add_children(root, vec![rect1, rect2].as_slice());

    let mut engine = LayoutEngine::new();

    engine.compute(&tree, root, 500.0, 500.0);

    let mut pixmap = Pixmap::new(500, 500).unwrap();

    pixmap.fill(Color::from_rgba8(255, 255, 255, 255));

    let mut paint = Paint::default();
    paint.set_color(Color::from_rgba8(255, 0, 0, 255));

    let rect = Rect::from_xywh(
        engine.get(1).unwrap().x,
        engine.get(1).unwrap().y,
        engine.get(1).unwrap().width,
        engine.get(1).unwrap().height,
    )
    .unwrap();

    let mut path = PathBuilder::new();
    path.push_rect(rect);

    let rect_path = path.finish().unwrap();

    pixmap.fill_path(
        &rect_path,
        &paint,
        FillRule::EvenOdd,
        Transform::identity(),
        None,
    );

    let rect2 = Rect::from_xywh(
        engine.get(2).unwrap().x,
        engine.get(2).unwrap().y,
        engine.get(2).unwrap().width,
        engine.get(2).unwrap().height,
    )
    .unwrap();

    let mut path = PathBuilder::new();
    path.push_rect(rect2);
    let rect_path = path.finish().unwrap();

    pixmap.fill_path(
        &rect_path,
        &paint,
        FillRule::EvenOdd,
        Transform::identity(),
        None,
    );

    pixmap.save_png("rectangles.png").unwrap();
}
