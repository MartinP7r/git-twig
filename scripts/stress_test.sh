#!/bin/bash
# Stress test script for git-twig performance
# Creates a test repository with many files and measures git-twig responsiveness

set -e

FILE_COUNT="${1:-1000}"
STRESS_DIR="/tmp/git-twig-stress-test"

echo "=== git-twig Stress Test ==="
echo "Creating test repo with $FILE_COUNT files..."

# Clean up any existing test directory
rm -rf "$STRESS_DIR"
mkdir -p "$STRESS_DIR"
cd "$STRESS_DIR"

# Initialize git repo
git init -q
git config user.email "test@example.com"
git config user.name "Test User"

# Create nested directory structure
echo "Creating directory structure..."
for dir in src lib tests docs examples; do
    mkdir -p "$dir"
    for subdir in a b c d e; do
        mkdir -p "$dir/$subdir"
    done
done

# Create files
echo "Creating $FILE_COUNT files..."
for i in $(seq 1 $FILE_COUNT); do
    dir_index=$((i % 5))
    subdir_index=$((i % 5))
    dirs=("src" "lib" "tests" "docs" "examples")
    subdirs=("a" "b" "c" "d" "e")
    dir="${dirs[$dir_index]}/${subdirs[$subdir_index]}"
    echo "content $i" > "$dir/file_$i.txt"
done

# Initial commit
echo "Creating initial commit..."
git add . && git commit -q -m "initial"

# Modify half the files (will be unstaged changes)
echo "Modifying half the files..."
for i in $(seq 1 $((FILE_COUNT / 2))); do
    dir_index=$((i % 5))
    subdir_index=$((i % 5))
    dirs=("src" "lib" "tests" "docs" "examples")
    subdirs=("a" "b" "c" "d" "e")
    dir="${dirs[$dir_index]}/${subdirs[$subdir_index]}"
    echo "modified content $i" >> "$dir/file_$i.txt"
done

# Stage some files
echo "Staging 10% of files..."
for i in $(seq 1 $((FILE_COUNT / 10))); do
    dir_index=$((i % 5))
    subdir_index=$((i % 5))
    dirs=("src" "lib" "tests" "docs" "examples")
    subdirs=("a" "b" "c" "d" "e")
    dir="${dirs[$dir_index]}/${subdirs[$subdir_index]}"
    git add "$dir/file_$i.txt" 2>/dev/null || true
done

echo ""
echo "=== Environment Ready ==="
echo "Directory: $STRESS_DIR"
echo "Modified files: $((FILE_COUNT / 2))"
echo "Staged files: $((FILE_COUNT / 10))"
echo ""

# Measure git status time
echo "=== Baseline: git status --porcelain ==="
time git status --porcelain -b -u > /dev/null

echo ""
echo "=== Baseline: git diff --numstat ==="
time git diff --numstat > /dev/null

echo ""
echo "=== Running git-twig (non-interactive) ==="
if command -v git-twig &> /dev/null; then
    time git-twig > /dev/null
else
    echo "git-twig not found in PATH. Run: cargo install --path /path/to/git-twig"
    echo "Or: cargo build --release && ./target/release/git-twig"
fi

echo ""
echo "=== Manual Interactive Test ==="
echo "To test interactive mode performance, run:"
echo "  cd $STRESS_DIR && git-twig -I"
echo ""
echo "Then try rapidly staging/unstaging files with 's' or space."
echo "Clean up with: rm -rf $STRESS_DIR"
