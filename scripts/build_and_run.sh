#!/usr/bin/env bash
set -euo pipefail

echo "Building workspace..."
cargo build --workspace

echo "Running tests..."
cargo test --workspace

echo "Running example 'toy_pake'..."
cargo run -p toy_pake
