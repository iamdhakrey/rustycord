# RustyCord Makefile

.PHONY: help build test check clean release release-patch release-minor release-major release-prerelease release-dry docs serve-docs

# Default target
help:
	@echo "🦀 RustyCord Development Commands"
	@echo ""
	@echo "Development:"
	@echo "  build         Build the project"
	@echo "  test          Run tests"
	@echo "  check         Run cargo check"
	@echo "  clean         Clean build artifacts"
	@echo ""
	@echo "Documentation:"
	@echo "  docs          Build documentation"
	@echo "  serve-docs    Serve documentation locally"
	@echo ""
	@echo "Release:"
	@echo "  release-dry   Dry run patch release"
	@echo "  release-patch Patch release (0.1.0 → 0.1.1)"
	@echo "  release-minor Minor release (0.1.0 → 0.2.0)"
	@echo "  release-major Major release (0.1.0 → 1.0.0)"
	@echo "  release-pre   Prerelease (0.1.0 → 0.1.1-alpha.1)"
	@echo ""

# Development commands
build:
	cargo build

test:
	cargo test

check:
	cargo check

clean:
	cargo clean

# Documentation commands
docs:
	mkdocs build

serve-docs:
	mkdocs serve

# Release commands
release-dry:
	@echo "🧪 Running dry release (patch)..."
	./scripts/release.sh patch --dry-run

release-patch:
	@echo "🚀 Starting patch release..."
	./scripts/release.sh patch

release-minor:
	@echo "🚀 Starting minor release..."
	./scripts/release.sh minor

release-major:
	@echo "🚀 Starting major release..."
	./scripts/release.sh major

release-pre:
	@echo "🚀 Starting prerelease..."
	./scripts/release.sh prerelease

# Aliases
release: release-patch
