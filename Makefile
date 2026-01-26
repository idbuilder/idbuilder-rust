.PHONY: build check test lint fmt fmt-check clean

# Build release binary
build:
	cargo build --release

# Quick cargo check
check:
	cargo check

# Run all tests
test:
	cargo test

# Run clippy linter
lint:
	cargo clippy -- -D warnings

# Format code
fmt:
	cargo fmt

# Check formatting without modifying files
fmt-check:
	cargo fmt --check

# Run all checks (format + lint + test)
all: fmt-check lint test

# Clean build artifacts
clean:
	cargo clean
