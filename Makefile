
# Makefile for git-twig

.PHONY: bootstrap run test fmt lint check clean serve-docs serve-landing

# Install git hooks
bootstrap:
	@echo "Installing git hooks..."
	@mkdir -p .git/hooks
	@ln -sf ../../scripts/hooks/pre-push .git/hooks/pre-push
	@ln -sf ../../scripts/hooks/pre-commit .git/hooks/pre-commit
	@chmod +x scripts/hooks/pre-push
	@chmod +x scripts/hooks/pre-commit
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

# Lint code
lint:
	cargo clippy -- -D warnings

# Check code definition
check:
	cargo check

# Clean build artifacts
clean:
	cargo clean

# Serve MkDocs documentation locally (at /docs/)
serve-docs:
	mkdocs serve

# Serve landing page locally
serve-landing:
	python3 -m http.server 8080 --directory docs/site
