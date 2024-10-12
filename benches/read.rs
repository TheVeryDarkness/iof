use criterion::{criterion_group, criterion_main, Criterion};
use iof::{unwrap, InputStream, ReadInto, ReadOneInto};
use std::{
    fs::{read_to_string, File},
    io::{self, BufRead, BufReader, Cursor, Read, Write},
};

#[derive(Clone)]
struct LazyWriter(std::ops::Range<i32>);

impl Read for LazyWriter {
    fn read(&mut self, mut buf: &mut [u8]) -> io::Result<usize> {
        if let Some(num) = self.0.next() {
            let s = format!("{} ", num);
            let len = s.len();
            let _ = buf.write(s.as_bytes())?;
            Ok(len)
        } else {
            Ok(0)
        }
    }
}

fn template<R: BufRead>(
    case: &'static str,
    count: usize,
    create_reader: impl Fn() -> R,
) -> impl FnMut(&mut Criterion) {
    type Element = i32;
    move |c| {
        c.bench_function(&format!("{case} - read_all"), |b| {
            b.iter(|| {
                let mut reader = InputStream::new(create_reader());
                let results: Vec<Element> = reader.read_all();
                assert_eq!(results.len(), count);
            })
        })
        .bench_function(&format!("{case} - read_n"), |b| {
            b.iter(|| {
                let mut reader = InputStream::new(create_reader());
                let results: Vec<Element> = reader.read_n(count);
                assert_eq!(results.len(), count);
            })
        })
        .bench_function(&format!("{case} - read while let"), |b| {
            b.iter(|| {
                let mut reader = InputStream::new(create_reader());
                let mut results: Vec<Element> = Vec::new();
                while let Ok(a) = reader.try_read_one() {
                    results.push(a);
                }
                assert_eq!(results.len(), count);
            })
        })
        .bench_function(&format!("{case} - read for in"), |b| {
            b.iter(|| {
                let mut reader = InputStream::new(create_reader());
                let mut results: Vec<Element> = Vec::new();
                for _ in 0..count {
                    results.push(reader.read());
                }
                assert_eq!(results.len(), count);
            })
        })
        .bench_function(&format!("{case} - read_to_end split"), |b| {
            b.iter(|| {
                let mut reader = create_reader();
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
                assert_eq!(results.len(), count);
            })
        })
        .bench_function(&format!("{case} - bytes split"), |b| {
            b.iter(|| {
                let reader = create_reader();
                let mut buf = Vec::new();
                let mut results: Vec<Element> = Vec::with_capacity(count);
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
                assert_eq!(results.len(), count);
            })
        });
    }
}

const COUNT: usize = 0x10000 * 4;

fn cursor(c: &mut Criterion) {
    {
        let s = unwrap!(read_to_string("benches/long.txt"));
        (template("cursor - long", COUNT, || Cursor::new(&s)))(c);
    }
    {
        let s = unwrap!(read_to_string("benches/short.txt"));
        (template("cursor - short", COUNT, || Cursor::new(&s)))(c);
    }
}

fn file(c: &mut Criterion) {
    (template("file - long", COUNT, || {
        let f = unwrap!(File::open("benches/long.txt"));
        BufReader::new(f)
    }))(c);
    (template("file - short", COUNT, || {
        let f = unwrap!(File::open("benches/short.txt"));
        BufReader::new(f)
    }))(c);
}

fn lazy(c: &mut Criterion) {
    let writer = LazyWriter(0..COUNT as i32);
    {
        (template("lazy - long", COUNT, || BufReader::new(writer.clone())))(c);
    }
    {
        (template("lazy - short", COUNT, || BufReader::new(writer.clone())))(c);
    }
}

criterion_group!(benches, cursor, file, lazy);
criterion_main!(benches);
