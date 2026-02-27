use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use taffy::{AvailableSpace, NodeId, Size, Style, TaffyTree};

/// Build a random leaf node
fn build_random_leaf(tree: &mut TaffyTree) -> NodeId {
    tree.new_with_children(Style::DEFAULT, &[]).unwrap()
}

/// A tree with many children that have shallow depth
fn build_flat_hierarchy(total_node_count: u32) -> (TaffyTree, NodeId) {
    let mut tree = TaffyTree::new();
    let mut rng = ChaCha8Rng::seed_from_u64(12345);
    let mut children = Vec::new();
    let mut node_count = 0;

    while node_count < total_node_count {
        let sub_children_count = rng.random_range(1..=4);
        let sub_children: Vec<NodeId> = (0..sub_children_count)
            .map(|_| build_random_leaf(&mut tree))
            .collect();
        let node = tree
            .new_with_children(Style::DEFAULT, &sub_children)
            .unwrap();

        children.push(node);
        node_count += 1 + sub_children_count;
    }

    let root = tree
        .new_with_children(Style::DEFAULT, children.as_slice())
        .unwrap();
    (tree, root)
}

fn benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("Tree creation");
    for node_count in [1_000u32, 10_000, 100_000].iter() {
        let benchmark_id = BenchmarkId::new("Tree::new", node_count);
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

fn layout_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("Layout computation");

    for node_count in [1_000u32, 10_000, 100_000].iter() {
        let benchmark_id = BenchmarkId::new("TaffyTree::compute_layout", node_count);

        let (mut tree, root) = build_flat_hierarchy(*node_count);

        group.bench_with_input(benchmark_id, node_count, |b, _| {
            b.iter(|| {
                tree.compute_layout(
                    root,
                    Size {
                        width: AvailableSpace::Definite(1000.0),
                        height: AvailableSpace::Definite(1000.0),
                    },
                )
                .unwrap();
                std::hint::black_box(&tree);
            })
        });
    }
    group.finish();
}

criterion_group!(benches, benchmarks, layout_benchmarks);
criterion_main!(benches);
