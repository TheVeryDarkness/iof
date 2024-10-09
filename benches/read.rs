use criterion::{criterion_group, criterion_main, Criterion};
use iof::{unwrap, InputStream, ReadInto, ReadOneInto};
use std::{
    fs::{read_to_string, File},
    io::{BufRead, BufReader, Cursor},
};

const COUNT: usize = 0x10000 * 4;

fn template<B, R: BufRead>(
    case: &'static str,
    create_buffer: impl Fn() -> B,
    create_reader: impl Fn(&B) -> R,
) -> impl FnMut(&mut Criterion) {
    type Element = i32;
    move |c| {
        let buf = create_buffer();
        c.bench_function(&format!("{case} - read_all"), |b| {
            b.iter(|| {
                let mut reader = InputStream::new(create_reader(&buf));
                let results: Vec<Element> = reader.read_all();
                assert_eq!(results.len(), COUNT);
            })
        })
        .bench_function(&format!("{case} - read_n"), |b| {
            b.iter(|| {
                let mut reader = InputStream::new(create_reader(&buf));
                let results: Vec<Element> = reader.read_n(COUNT);
                assert_eq!(results.len(), COUNT);
            })
        })
        .bench_function(&format!("{case} - read while let"), |b| {
            b.iter(|| {
                let mut reader = InputStream::new(create_reader(&buf));
                let mut results: Vec<Element> = Vec::new();
                while let Ok(a) = reader.try_read_one() {
                    results.push(a);
                }
                assert_eq!(results.len(), COUNT);
            })
        })
        .bench_function(&format!("{case} - read for in"), |b| {
            b.iter(|| {
                let mut reader = InputStream::new(create_reader(&buf));
                let mut results: Vec<Element> = Vec::new();
                for _ in 0..COUNT {
                    results.push(reader.read());
                }
                assert_eq!(results.len(), COUNT);
            })
        })
        .bench_function(&format!("{case} - read_to_end split"), |b| {
            b.iter(|| {
                let mut reader = create_reader(&buf);
                let mut buf = Vec::new();
                unwrap!(reader.read_to_end(&mut buf));
                let results: Vec<Element> = buf
                    .split(|&b| matches!(b, b' ' | b'\n'))
                    .filter_map(|s| {
                        if s.is_empty() {
                            None
                        } else {
                            let s = unwrap!(std::str::from_utf8(s));
                            Some(unwrap!(s.parse::<Element>()))
                        }
                    })
                    .collect();
                assert_eq!(results.len(), COUNT);
            })
        })
        .bench_function(&format!("{case} - bytes split"), |b| {
            b.iter(|| {
                let reader = create_reader(&buf);
                let mut buf = Vec::new();
                let mut results: Vec<Element> = Vec::new();
                for byte in reader.bytes() {
                    let byte = unwrap!(byte);
                    if byte == b' ' || byte == b'\n' {
                        if !buf.is_empty() {
                            let s = unwrap!(std::str::from_utf8(&buf));
                            results.push(unwrap!(s.parse::<Element>()));
                            buf.clear();
                        }
                    } else {
                        buf.push(byte);
                    }
                }
                assert_eq!(results.len(), COUNT);
            })
        });
    }
}

fn cursor(c: &mut Criterion) {
    let s = unwrap!(read_to_string("benches/integers.txt"));
    (template("cursor", || Cursor::new(&s), |s| s.clone()))(c);
}

fn file(c: &mut Criterion) {
    (template(
        "cursor",
        || (),
        |()| {
            let f = unwrap!(File::open("benches/integers.txt"));
            BufReader::new(f)
        },
    ))(c);
}

criterion_group!(benches, cursor, file);
criterion_main!(benches);
