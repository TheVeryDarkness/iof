#!/bin/zsh

set -eux

clear

cargo +nightly fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo doc --no-deps
cargo test --workspace
cargo llvm-cov --workspace  --hide-instantiations --html --open --show-missing-lines
# cargo llvm-cov --workspace --html --open
# cargo llvm-cov --workspace --hide-instantiations --html --open
cargo +nightly miri test --workspace
