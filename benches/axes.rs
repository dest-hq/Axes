use axes::{LayoutEngine, LayoutTree, NodeId, Size, Style, Units};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

/// Build a random leaf node
fn build_random_leaf(tree: &mut LayoutTree) -> NodeId {
    tree.new_child(Style::default())
}

/// A tree with many children that have shallow depth
fn build_flat_hierarchy(total_node_count: u32) -> (LayoutTree, NodeId) {
    let mut tree = LayoutTree::new();
    let mut rng = ChaCha8Rng::seed_from_u64(12345);
    let mut children = Vec::new();
    let mut node_count = 0;

    while node_count < total_node_count {
        let sub_children_count = rng.random_range(1..=4);
        let sub_children: Vec<NodeId> = (0..sub_children_count)
            .map(|_| build_random_leaf(&mut tree))
            .collect();
        let node = tree.new_child(Style::default());
        tree.add_children(node, sub_children);

        children.push(node);
        node_count += 1 + sub_children_count;
    }

    let root = tree.new_child(Style::default());
    tree.add_children(root, children);
    (tree, root)
}

fn layout_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("Layout computation");

    for node_count in [1_000u32, 10_000, 100_000].iter() {
        let benchmark_id = BenchmarkId::new("LayoutEngine::compute", node_count);

        // Pre-build the tree once
        let (tree, root) = build_flat_hierarchy(*node_count);

        group.bench_with_input(benchmark_id, node_count, |b, _| {
            b.iter(|| {
                let mut engine = LayoutEngine::new();
                engine.compute(
                    &tree,
                    root,
                    Size {
                        width: Units::Pixels(1000.0),
                        height: Units::Pixels(1000.0),
                    },
                );
                std::hint::black_box(engine);
            })
        });
    }
    group.finish();
}

fn benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("Tree creation");
    for node_count in [1_000u32, 10_000, 100_000].iter() {
        let benchmark_id = BenchmarkId::new("Tree::new".to_string(), node_count);
        group.bench_with_input(benchmark_id, node_count, |b, &node_count| {
            b.iter(|| {
                let (tree, root) = build_flat_hierarchy(node_count);
                std::hint::black_box(tree);
                std::hint::black_box(root);
            })
        });
    }
    group.finish();
}

criterion_group!(benches, benchmarks, layout_benchmarks);
criterion_main!(benches);
