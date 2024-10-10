use criterion::{criterion_group, criterion_main, Criterion};
use iof::{sep_by, show, unwrap};
use std::ops::Range;

fn many_integers(c: &mut Criterion) {
    const INTEGERS: [Range<i64>; 3] = [
        -0x10000i64..0x10000i64,
        -0x80000000..-0x7fff0000,
        0x70000000..0x70010000,
    ];
    let s = unwrap!(std::fs::read_to_string("benches/long.txt"));
    c.bench_function("default_separator", |b| {
        b.iter(|| {
            let mut buf = Vec::new();
            show!(sep_by!(INTEGERS, "\n", " ") => buf);
            assert_eq!(buf, s.as_bytes());
        })
    })
    .bench_function("longer_separator", |b| {
        b.iter(|| {
            let mut buf = Vec::new();
            show!(sep_by!(INTEGERS, "\n\n", " :: ") => buf);
        })
    })
    .bench_function("char_separator", |b| {
        b.iter(|| {
            let mut buf = Vec::new();
            show!(sep_by!(INTEGERS, '\n', ' ') => buf);
        })
    });
}

criterion_group!(benches, many_integers);
criterion_main!(benches);
