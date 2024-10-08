use criterion::{criterion_group, criterion_main, Criterion};
use iof::{sep_by, show, unwrap};

fn many_integers(c: &mut Criterion) {
    let s = unwrap!(std::fs::read_to_string("benches/integers.txt"));
    c.bench_function("default_separator", |_| {
        let mut buf = Vec::new();
        let integers = [
            -0x10000i64..0x10000i64,
            -0x80000000..-0x7fff0000,
            0x70000000..0x70010000,
        ];
        show!(sep_by!(integers, "\n", " ") => buf);
        assert_eq!(buf, s.as_bytes());
    })
    .bench_function("longer_separator", |_| {
        let mut buf = Vec::new();
        let integers = [
            -0x10000i64..0x10000i64,
            -0x80000000..-0x7fff0000,
            0x70000000..0x70010000,
        ];
        show!(sep_by!(integers, "\n\n", " :: ") => buf);
        assert_eq!(buf, s.as_bytes());
    })
    .bench_function("char_separator", |_| {
        let mut buf = Vec::new();
        let integers = [
            -0x10000i64..0x10000i64,
            -0x80000000..-0x7fff0000,
            0x70000000..0x70010000,
        ];
        show!(sep_by!(integers, '\n', ' ') => buf);
    });
}

criterion_group!(benches, many_integers);
criterion_main!(benches);
