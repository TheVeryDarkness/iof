mod tracked;

use crate::array::array_try_from_fn;
use std::convert::Infallible;
use std::{panic::catch_unwind, thread::spawn};
use tracked::{check, Tracked};

#[test]
fn array_string() {
    let mut i = 0;
    let array: [String; 3] = array_try_from_fn(|| -> Result<String, Infallible> {
        i += 1;
        Ok(i.to_string())
    })
    .unwrap();
    assert_eq!(array[0], "1");
    assert_eq!(array[1], "2");
    assert_eq!(array[2], "3");
}

#[test]
fn consecutive() {
    array_tracked_caught_panic();
    array_tracked()
}

fn array_tracked_caught_panic() {
    let threads: Vec<_> = (0..16)
        .map(|i| {
            spawn(move || {
                let res = catch_unwind(|| {
                    let mut j = 0;
                    let array: Result<[Tracked; 64], Infallible> = array_try_from_fn(|| {
                        if j >= 63 {
                            panic!("Sorry, something is wrong with the array.");
                        }
                        j += 1;
                        Ok(Tracked::new(i, j))
                    });
                    array.unwrap()
                });
                assert!(res.is_err());
            })
        })
        .collect();

    for t in threads {
        t.join().unwrap();
    }

    check();
}

fn array_tracked() {
    let threads: Vec<_> = (0..16)
        .map(|i| {
            spawn(move || {
                let res = catch_unwind(|| {
                    let mut j = 0;
                    let array: Result<[Tracked; 64], &'static str> = array_try_from_fn(|| {
                        if j >= 63 {
                            Err("Sorry, something is wrong with the array.")?
                        }
                        j += 1;
                        Ok(Tracked::new(i, j))
                    });
                    array
                })
                .unwrap();
                assert!(res.is_err());
            })
        })
        .collect();

    for t in threads {
        t.join().unwrap();
    }

    check();
}
