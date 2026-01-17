
# Makefile for git-twig

.PHONY: bootstrap run test fmt check clean

# Install git hooks
bootstrap:
	@echo "Installing git hooks..."
	@mkdir -p .git/hooks
	@ln -sf ../../scripts/hooks/pre-push .git/hooks/pre-push
	@chmod +x scripts/hooks/pre-push
	@echo "✅ Hooks installed!"

# Run the application (default args)
run:
	cargo run -- --theme ascii

# Run tests
test:
	cargo test

# Format code
fmt:
	cargo fmt

# Check code definition
check:
	cargo check

# Clean build artifacts
clean:
	cargo clean
