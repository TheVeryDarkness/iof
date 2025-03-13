#!/bin/zsh

set -eux

clear

cargo +nightly fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo doc --no-deps
cargo test --workspace
cargo test --workspace --all-features
# cargo llvm-cov --workspace  --hide-instantiations --html --open --show-missing-lines
cargo llvm-cov --workspace  --hide-instantiations --html --open --show-missing-lines --all-features
# cargo +nightly miri test --workspace
# cargo +nightly miri test --workspace --all-features
