#!/bin/zsh

set -eux

clear

cargo +nightly fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo +nightly miri test --workspace
cargo llvm-cov --workspace --hide-instantiations --html --open
