# Justfile for Crust8 workspace

default:
    @just --list

# Build
build-release:
    cargo build --workspace --release

build-debug:
    cargo build --workspace

check:
    cargo check --workspace

# Test & Lint
test:
    cargo test --workspace

lint:
    cargo clippy --workspace --all-targets -- -D warnings

# Formatting
fmt:
    cargo fmt --all

fmt-check:
    cargo fmt --all -- --check

# Clean
clean:
    cargo clean

# Run
run-desktop ROM_PATH:
    cargo run -p desktop --release -- "{{ROM_PATH}}"
