use criterion::{criterion_group, criterion_main, Criterion};
use iof::{unwrap, InputStream, ReadInto, ReadOneInto};
use std::{fs::File, io::BufReader};

const COUNT: usize = 0x10000 * 4;

fn create_reader() -> InputStream<BufReader<File>> {
    let f = unwrap!(File::open("benches/integers.txt"));
    let buf = BufReader::new(f);
    InputStream::new(buf)
}

fn many_integers(c: &mut Criterion) {
    c.bench_function("read_all", |_| {
        let mut reader = create_reader();
        let _: Vec<i32> = reader.read_all();
    })
    .bench_function("read_n", |_| {
        let mut reader = create_reader();
        let _: Vec<i32> = reader.read_n(COUNT);
    })
    .bench_function("read", |_| {
        let mut reader = create_reader();
        let _: Vec<i32> = reader.read();
    })
    .bench_function("read while let", |_| {
        let mut reader = create_reader();
        let mut results: Vec<i64> = Vec::new();
        while let Ok(a) = reader.try_read_one() {
            results.push(a);
        }
    })
    .bench_function("read for in", |_| {
        let mut reader = create_reader();
        let mut results: Vec<i64> = Vec::new();
        for _ in 0..COUNT {
            results.push(reader.read());
        }
    });
}

criterion_group!(benches, many_integers);
criterion_main!(benches);
