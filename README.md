# Axes

Axes is a layout engine written in Rust.

[![Crates.io](https://img.shields.io/crates/v/axes.svg?logo=rust)](https://crates.io/crates/axes)
[![Apache 2.0 or MIT license.](https://img.shields.io/badge/license-MIT-blue.svg)]()
[![Documentation](https://docs.rs/axes/badge.svg)](https://docs.rs/axes)
[![GitHub issues](https://img.shields.io/github/issues/dest-hq/axes.svg)](https://github.com/dest-hq/axes/issues/)


The goal of the project is simple, to provide a fast and predictable layout engine.

> [!WARNING]
> Axes is still a work in progress. The API may change.

## What is Axes?

Axes implements a custom layout model. It is **not** a CSS Flexbox/Grid engine.

If you need CSS compatibility, you should probably use Taffy instead.  
But if you need a lightweight and fast layout engine that you can embed into your own engine, Axes might be a better fit.

## Comparison

Axes was benchmarked against Taffy 0.9.2 from crates.io.

Benchmark results can be found in [`benches/results.md`](https://github.com/dest-hq/Axes/blob/main/benches/results.md).

In these benchmarks, Axes performs faster in all tested cases (Approximately 3x faster than Taffy).

Taffy supports significantly more layout features and full CSS-like behavior, which affects performance characteristics. Axes focuses on a smaller, custom layout model.

## Usage

```rust
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
    padding: Padding {
        top: 20.0,
        ..Default::default()
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
    vertical_align: VerticalAlign::Center,
    ..Default::default()
});

// Set parent to child
tree.add_children(root, vec![child1, child2].as_slice());

let mut engine = LayoutEngine::new();

engine.compute(&tree, root, 900.0, 900.0);

assert_eq!(engine.get(0).unwrap().x, 0.0);
assert_eq!(engine.get(0).unwrap().y, 0.0);

assert_eq!(engine.get(1).unwrap().height, 300.0);
assert_eq!(engine.get(1).unwrap().width, 450.0);
assert_eq!(engine.get(1).unwrap().x, 0.0);
assert_eq!(engine.get(1).unwrap().y, 20.0);

assert_eq!(engine.get(2).unwrap().height, 300.0);
assert_eq!(engine.get(2).unwrap().width, 200.0);
assert_eq!(engine.get(2).unwrap().y, 105.0);
assert_eq!(engine.get(2).unwrap().x, 500.0);
}}
```

## Contributing

Contributions are welcome.

If you want to help:

- Open an issue to discuss ideas or report bugs
- Submit a pull request
- Improve documentation or examples

Since the project is still evolving, breaking changes may happen between minor versions.

## License

Axes is licensed under the MIT License.
