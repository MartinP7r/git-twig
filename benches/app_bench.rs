//! Benchmarks for git-twig application operations
//!
//! These benchmarks measure the performance of core operations:
//! - Tree flattening (Node::flatten)
//! - Filter nodes (App::filter_nodes)
//! - ANSI code stripping (strip_ansi_codes)
//! - Full refresh cycle

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::collections::HashSet;

use git_twig::node::{Node, NodeType};
use git_twig::theme::{Theme, ThemeType};
use git_twig::tui::app::{strip_ansi_codes, App};

/// Create a test tree with the specified depth and breadth
fn create_test_tree(depth: usize, breadth: usize) -> Node {
    fn build_level(current_depth: usize, max_depth: usize, breadth: usize) -> Node {
        if current_depth >= max_depth {
            // Create a file node using the constructor
            Node::new_file(
                format!("file_{}.rs", current_depth),
                format!("path/to/file_{}.rs", current_depth),
                "M".to_string(),
                Some((10, 5)), // (added, deleted)
            )
        } else {
            // Create a directory with children
            let mut children = Vec::new();
            for _ in 0..breadth {
                let child = build_level(current_depth + 1, max_depth, breadth);
                children.push(child);
            }
            // Add some file nodes at each level
            for i in 0..3 {
                children.push(Node::new_file(
                    format!("file_{}.rs", i),
                    format!("level_{}/file_{}.rs", current_depth, i),
                    "M".to_string(),
                    Some((i + 1, i)),
                ));
            }
            Node::new_dir(
                format!("dir_{}", current_depth),
                format!("path/to/dir_{}", current_depth),
                children,
            )
        }
    }

    build_level(0, depth, breadth)
}

/// Benchmark tree flattening with various tree sizes
fn bench_tree_flatten(c: &mut Criterion) {
    let mut group = c.benchmark_group("tree_flatten");
    let theme = Theme::new(ThemeType::Unicode);
    let collapsed_paths: HashSet<String> = HashSet::new();

    // Test with different tree configurations (depth, breadth)
    for (depth, breadth) in [(3, 3), (4, 4), (5, 3), (3, 10)].iter() {
        let tree = create_test_tree(*depth, *breadth);
        let node_count = count_nodes(&tree);

        group.bench_with_input(BenchmarkId::new("nodes", node_count), &tree, |b, tree| {
            b.iter(|| {
                black_box(tree.flatten(2, false, &theme, &collapsed_paths));
            });
        });
    }

    group.finish();
}

/// Count total nodes in a tree
fn count_nodes(node: &Node) -> usize {
    match &node.node_type {
        NodeType::File { .. } => 1,
        NodeType::Directory { children } => 1 + children.iter().map(count_nodes).sum::<usize>(),
    }
}

/// Benchmark filter_nodes with various query lengths and node counts
fn bench_filter_nodes(c: &mut Criterion) {
    let mut group = c.benchmark_group("filter_nodes");
    let theme = Theme::new(ThemeType::Unicode);
    let collapsed_paths: HashSet<String> = HashSet::new();

    // Create trees of different sizes
    for (depth, breadth) in [(3, 5), (4, 4), (5, 3)].iter() {
        let tree = create_test_tree(*depth, *breadth);
        let flat_nodes = tree.flatten(2, false, &theme, &collapsed_paths);
        let node_count = flat_nodes.len();

        // Test with different query lengths
        for query in ["", "file", "dir_2", "nonexistent"].iter() {
            let id = format!("{}_nodes_query_{}", node_count, query.len());
            group.bench_with_input(BenchmarkId::new("filter", &id), &flat_nodes, |b, nodes| {
                b.iter(|| {
                    black_box(App::filter_nodes(nodes, query));
                });
            });
        }
    }

    group.finish();
}

/// Benchmark ANSI code stripping
fn bench_strip_ansi_codes(c: &mut Criterion) {
    let mut group = c.benchmark_group("strip_ansi_codes");

    // Sample strings with ANSI codes (simulating diff output)
    let test_strings: Vec<String> = vec![
        // No ANSI codes
        "plain text without any formatting".to_string(),
        // Single ANSI code
        "\x1B[32m+added line\x1B[0m".to_string(),
        // Multiple ANSI codes
        "\x1B[31m-\x1B[0m\x1B[31mdeleted line with \x1B[1mbold\x1B[0m text\x1B[0m".to_string(),
        // Long line with many codes
        (0..50)
            .map(|i| {
                if i % 2 == 0 {
                    format!("\x1B[32m+line {}\x1B[0m", i)
                } else {
                    format!("\x1B[31m-line {}\x1B[0m", i)
                }
            })
            .collect::<Vec<_>>()
            .join("\n"),
    ];

    for (idx, s) in test_strings.iter().enumerate() {
        let id = format!("len_{}_idx_{}", s.len(), idx);
        group.bench_with_input(BenchmarkId::new("strip", &id), s, |b, s| {
            b.iter(|| {
                black_box(strip_ansi_codes(s));
            });
        });
    }

    group.finish();
}

/// Benchmark multiple strip_ansi_codes calls (simulating diff search)
fn bench_strip_ansi_batch(c: &mut Criterion) {
    let mut group = c.benchmark_group("strip_ansi_batch");

    // Simulate a diff with many lines
    let diff_lines: Vec<String> = (0..500)
        .map(|i| {
            if i % 3 == 0 {
                format!("\x1B[32m+added line {} with some content\x1B[0m", i)
            } else if i % 3 == 1 {
                format!("\x1B[31m-deleted line {} with content\x1B[0m", i)
            } else {
                format!(" context line {} unchanged", i)
            }
        })
        .collect();

    group.bench_function("500_lines", |b| {
        b.iter(|| {
            for line in &diff_lines {
                black_box(strip_ansi_codes(line));
            }
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_tree_flatten,
    bench_filter_nodes,
    bench_strip_ansi_codes,
    bench_strip_ansi_batch,
);
criterion_main!(benches);
