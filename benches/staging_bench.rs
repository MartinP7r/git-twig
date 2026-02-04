use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::process::Command;
use tempfile::TempDir;

/// Create a test repository with the specified number of modified files
fn create_test_repo(file_count: usize) -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let path = temp_dir.path();

    // Initialize git repo
    Command::new("git")
        .args(["init"])
        .current_dir(path)
        .output()
        .expect("Failed to init git repo");

    // Configure git user for commits
    Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(path)
        .output()
        .expect("Failed to set git user email");

    Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(path)
        .output()
        .expect("Failed to set git user name");

    // Create files
    for i in 0..file_count {
        let file_path = path.join(format!("file_{}.txt", i));
        std::fs::write(&file_path, format!("content {}", i)).expect("Failed to write file");
    }

    // Initial commit
    Command::new("git")
        .args(["add", "."])
        .current_dir(path)
        .output()
        .expect("Failed to stage files");

    Command::new("git")
        .args(["commit", "-m", "initial"])
        .current_dir(path)
        .output()
        .expect("Failed to commit");

    // Modify half the files
    for i in 0..(file_count / 2) {
        let file_path = path.join(format!("file_{}.txt", i));
        std::fs::write(&file_path, format!("modified content {}", i))
            .expect("Failed to modify file");
    }

    temp_dir
}

/// Benchmark running git status
fn bench_git_status(c: &mut Criterion) {
    let mut group = c.benchmark_group("git_status");

    for size in [10, 100, 500, 1000].iter() {
        let temp_dir = create_test_repo(*size);
        let path = temp_dir.path();

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                Command::new("git")
                    .args(["status", "--porcelain", "-b", "-u"])
                    .current_dir(path)
                    .output()
                    .expect("Failed to run git status")
            });
        });
    }

    group.finish();
}

/// Benchmark running git diff --numstat
fn bench_git_diff_numstat(c: &mut Criterion) {
    let mut group = c.benchmark_group("git_diff_numstat");

    for size in [10, 100, 500, 1000].iter() {
        let temp_dir = create_test_repo(*size);
        let path = temp_dir.path();

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                Command::new("git")
                    .args(["diff", "--numstat"])
                    .current_dir(path)
                    .output()
                    .expect("Failed to run git diff")
            });
        });
    }

    group.finish();
}

/// Benchmark single file stage operation
fn bench_stage_single_file(c: &mut Criterion) {
    let mut group = c.benchmark_group("stage_single_file");

    for size in [10, 100, 500].iter() {
        let temp_dir = create_test_repo(*size);
        let path = temp_dir.path();

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                // Stage file_0.txt
                Command::new("git")
                    .args(["add", "file_0.txt"])
                    .current_dir(path)
                    .output()
                    .expect("Failed to stage file");

                // Unstage it
                Command::new("git")
                    .args(["restore", "--staged", "file_0.txt"])
                    .current_dir(path)
                    .output()
                    .expect("Failed to unstage file");
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_git_status,
    bench_git_diff_numstat,
    bench_stage_single_file
);
criterion_main!(benches);
