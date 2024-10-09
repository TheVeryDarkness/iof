//! Run `cargo run --example benchmark_integers` to prepare data for the benchmark.

use iof::{sep_by, show, unwrap};
use std::{fs::File, path::PathBuf};

fn main() {
    let integers = [
        -0x10000i64..0x10000i64,
        -0x80000000..-0x7fff0000,
        0x70000000..0x70010000,
    ];

    let manifest_dir = env!("CARGO_MANIFEST_DIR");

    let mut results = PathBuf::from(manifest_dir);
    results.push("benches");
    results.push("integers.txt");

    let mut f = unwrap!(File::create(results));

    // Write integers to file
    show!(sep_by!(integers, "\n", " ") => f);
}
