//! Run `cargo run --example benchmark_integers` to prepare data for the benchmark.

use iof::{sep_by, show, unwrap};
use std::{fs::File, ops::Range, path::PathBuf};

fn main() {
    const INTEGERS: [Range<i32>; 3] = [
        -0x10000..0x10000,
        -0x80000000..-0x7fff0000,
        0x70000000..0x70010000,
    ];

    let manifest_dir = env!("CARGO_MANIFEST_DIR");

    let benches = PathBuf::from(manifest_dir).join("benches");

    show!(sep_by!(INTEGERS, "\n", " ") => unwrap!(File::create(benches.join("long.txt"))));

    show!(sep_by!(INTEGERS, "\n", "\n") => unwrap!(File::create(benches.join("short.txt"))));
}
