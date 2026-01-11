# Vaulta Liquidity Aggregator - Build and Test Automation
# Cross-protocol liquidity routing with MEV protection

.PHONY: all build test bench run clean fmt clippy docs install help

# Default target
all: fmt clippy build

# ---------------------------------------------------------------------------
# Build Commands
# ---------------------------------------------------------------------------

build:
	@echo "  [Cargo] Building vaulta-liquidity-aggregator in release mode..."
	@cargo build --release
	@echo "  [✓] Build complete! Binary: target/release/vaulta-liquidity-aggregator"

build-dev:
	@echo "  [Cargo] Building vaulta-liquidity-aggregator in dev mode..."
	@cargo build
	@echo "  [✓] Dev build complete! Binary: target/debug/vaulta-liquidity-aggregator"

# ---------------------------------------------------------------------------
# Testing Commands
# ---------------------------------------------------------------------------

test: build-dev
	@echo "  [Test] Running test suite..."
	@cargo test --lib --tests -- --nocapture
	@echo "  [✓] All tests passed!"

test-verbose:
	@echo "  [Test] Running test suite with verbose output..."
	@cargo test --lib --tests -- --nocapture --test-threads=1

test-integration:
	@echo "  [Test] Running integration tests..."
	@cargo test --test '*' -- --nocapture

# ---------------------------------------------------------------------------
# Benchmarking
# ---------------------------------------------------------------------------

bench:
	@echo "  [Bench] Running benchmarks..."
	@cargo bench -- --output-format html
	@echo "  [✓] Benchmarks complete! Check target/criterion/ for reports"

bench-quick:
	@echo "  [Bench] Running quick benchmarks..."
	@cargo bench -- --quick

# ---------------------------------------------------------------------------
# Code Quality
# ---------------------------------------------------------------------------

fmt:
	@echo "  [Fmt] Formatting code..."
	@cargo fmt --all
	@echo "  [✓] Code formatted!"

clippy:
	@echo "  [Clippy] Running linter..."
	@cargo clippy --all-targets --all-features -- -D warnings
	@echo "  [✓] Linting passed!"

check: fmt clippy test
	@echo "  [✓] All checks passed!"

# ---------------------------------------------------------------------------
# Documentation
# ---------------------------------------------------------------------------

docs:
	@echo "  [Docs] Generating documentation..."
	@cargo doc --no-deps --open
	@echo "  [✓] Documentation generated!"

docs-build:
	@echo "  [Docs] Building documentation..."
	@cargo doc --no-deps
	@echo "  [✓] Documentation built in target/doc/"

# ---------------------------------------------------------------------------
# Run Commands
# ---------------------------------------------------------------------------

run: build
	@echo "  [Run] Executing vaulta-liquidity-aggregator..."
	@./target/release/vaulta-liquidity-aggregator --help

run-example:
	@echo "  [Run] Running example quote..."
	@cargo run --release -- quote \
		--input-mint So11111111111111111111111111111111111111112 \
		--output-mint EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v \
		--amount 1000000000

run-status:
	@echo "  [Run] Checking DEX status..."
	@cargo run --release -- status

# ---------------------------------------------------------------------------
# Installation
# ---------------------------------------------------------------------------

install: build
	@echo "  [Install] Installing vaulta-liquidity-aggregator..."
	@cargo install --path .
	@echo "  [✓] Installed! Run 'vaulta-liquidity-aggregator --help' to get started"

# ---------------------------------------------------------------------------
# Cleanup
# ---------------------------------------------------------------------------

clean:
	@echo "  [Clean] Removing build artifacts..."
	@cargo clean
	@rm -rf target/
	@echo "  [✓] Clean complete!"

clean-all: clean
	@echo "  [Clean] Removing all generated files..."
	@rm -rf results/
	@rm -rf logs/
	@echo "  [✓] Deep clean complete!"

# ---------------------------------------------------------------------------
# Development Workflow
# ---------------------------------------------------------------------------

dev: fmt clippy test build-dev
	@echo "  [✓] Development build ready!"

ci: fmt clippy test bench
	@echo "  [✓] CI checks complete!"

# ---------------------------------------------------------------------------
# Help
# ---------------------------------------------------------------------------

help:
	@echo "Vaulta Liquidity Aggregator - Build System"
	@echo ""
	@echo "Available targets:"
	@echo "  make build          - Build in release mode"
	@echo "  make build-dev      - Build in dev mode"
	@echo "  make test           - Run test suite"
	@echo "  make bench          - Run benchmarks"
	@echo "  make fmt            - Format code"
	@echo "  make clippy         - Run linter"
	@echo "  make check          - Run fmt, clippy, and test"
	@echo "  make docs           - Generate and open documentation"
	@echo "  make run-example    - Run example quote"
	@echo "  make run-status     - Check DEX status"
	@echo "  make install        - Install as cargo binary"
	@echo "  make clean          - Remove build artifacts"
	@echo "  make dev            - Full dev workflow (fmt, clippy, test, build)"
	@echo "  make ci             - CI workflow (fmt, clippy, test, bench)"
	@echo "  make help           - Show this help message"
