use criterion::{criterion_group, criterion_main, Criterion};
use iof::{unwrap, InputStream, ReadInto, ReadOneInto};
use std::{
    fs::{read_to_string, File},
    io::{self, BufRead, BufReader, Read, Write},
};

struct LazyWriter<const LONG: bool>(std::ops::Range<i32>, Vec<u8>);

impl<const LONG: bool> Read for LazyWriter<LONG> {
    fn read(&mut self, mut buf: &mut [u8]) -> io::Result<usize> {
        let mut count = 0usize;
        if !self.1.is_empty() {
            let len = buf.write(&self.1)?;
            self.1.drain(..len);
            count += len;
        }
        for num in &mut self.0 {
            let mut s = if LONG {
                format!("{} ", num)
            } else {
                format!("{}\n", num)
            };
            let len = buf.write(s.as_bytes())?;
            count += len;
            s.drain(..len);
            self.1.extend(s.bytes());
            if len == 0 {
                break;
            }
        }
        Ok(count)
    }
}

fn template<R: BufRead>(
    case: &'static str,
    count: usize,
    create_reader: impl Fn() -> R,
) -> impl FnMut(&mut Criterion) {
    type Element = i32;
    move |c| {
        c.bench_function(&format!("{case}-read_all"), |b| {
            b.iter(|| {
                let mut reader = InputStream::new(create_reader());
                let results: Vec<Element> = reader.read_all();
                assert_eq!(results.len(), count);
            })
        })
        .bench_function(&format!("{case}-read_n"), |b| {
            b.iter(|| {
                let mut reader = InputStream::new(create_reader());
                let results: Vec<Element> = reader.read_n(count);
                assert_eq!(results.len(), count);
            })
        })
        .bench_function(&format!("{case}-read while let"), |b| {
            b.iter(|| {
                let mut reader = InputStream::new(create_reader());
                let mut results: Vec<Element> = Vec::new();
                while let Ok(a) = reader.try_read_one() {
                    results.push(a);
                }
                assert_eq!(results.len(), count);
            })
        })
        .bench_function(&format!("{case}-read for in"), |b| {
            b.iter(|| {
                let mut reader = InputStream::new(create_reader());
                let mut results: Vec<Element> = Vec::new();
                for _ in 0..count {
                    results.push(reader.read());
                }
                assert_eq!(results.len(), count);
            })
        })
        .bench_function(&format!("{case}-read_to_end split"), |b| {
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
        .bench_function(&format!("{case}-bytes split"), |b| {
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
                if !buf.is_empty() {
                    let s = unwrap!(std::str::from_utf8(&buf));
                    results.push(unwrap!(s.parse::<Element>()));
                    // buf.clear();
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
        (template("cursor-long", COUNT, || s.as_bytes()))(c);
    }
    {
        let s = unwrap!(read_to_string("benches/short.txt"));
        (template("cursor-short", COUNT, || s.as_bytes()))(c);
    }
}

fn file(c: &mut Criterion) {
    (template("file-long", COUNT, || {
        let f = unwrap!(File::open("benches/long.txt"));
        BufReader::new(f)
    }))(c);
    (template("file-short", COUNT, || {
        let f = unwrap!(File::open("benches/short.txt"));
        BufReader::new(f)
    }))(c);
}

#[allow(dead_code)]
fn lazy(c: &mut Criterion) {
    (template("lazy-short", COUNT, || {
        BufReader::new(LazyWriter::<false>(0..COUNT as i32, Vec::new()))
    }))(c);
    (template("lazy-long", COUNT, || {
        BufReader::new(LazyWriter::<true>(0..COUNT as i32, Vec::new()))
    }))(c);
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn criterion_config() -> Criterion {
    Criterion::default().with_profiler(pprof::criterion::PProfProfiler::new(
        100,
        pprof::criterion::Output::Flamegraph(None),
    ))
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
fn criterion_config() -> Criterion {
    Criterion::default()
}

criterion_group!(
    name = benches;
    config = criterion_config();
    targets = cursor, file
);
criterion_main!(benches);
