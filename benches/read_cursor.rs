use criterion::{criterion_group, criterion_main, Criterion};
use iof::{unwrap, InputStream, ReadInto, ReadOneInto};
use std::{fs::read_to_string, io::Cursor};

const COUNT: usize = 0x10000 * 4;

fn many_integers(c: &mut Criterion) {
    let s = unwrap!(read_to_string("benches/integers.txt"));
    c.bench_function("read_all", |b| {
        b.iter(|| {
            let mut reader = InputStream::new(Cursor::new(s.as_str()));
            let results: Vec<i32> = reader.read_all();
            assert_eq!(results.len(), COUNT);
        })
    })
    .bench_function("read_n", |b| {
        b.iter(|| {
            let mut reader = InputStream::new(Cursor::new(s.as_str()));
            let results: Vec<i32> = reader.read_n(COUNT);
            assert_eq!(results.len(), COUNT);
        })
    })
    .bench_function("read", |b| {
        b.iter(|| {
            let mut reader = InputStream::new(Cursor::new(s.as_str()));
            // Three lines in total.
            let mut results: Vec<i32> = reader.read();
            results.append(&mut reader.read());
            results.append(&mut reader.read());
            assert_eq!(results.len(), COUNT);
        })
    })
    .bench_function("read while let", |b| {
        b.iter(|| {
            let mut reader = InputStream::new(Cursor::new(s.as_str()));
            let mut results: Vec<i64> = Vec::new();
            while let Ok(a) = reader.try_read_one() {
                results.push(a);
            }
            assert_eq!(results.len(), COUNT);
        })
    })
    .bench_function("read for in", |b| {
        b.iter(|| {
            let mut reader = InputStream::new(Cursor::new(s.as_str()));
            let mut results: Vec<i64> = Vec::new();
            for _ in 0..COUNT {
                results.push(reader.read());
            }
            assert_eq!(results.len(), COUNT);
        })
    });
}

criterion_group!(benches, many_integers);
criterion_main!(benches);
