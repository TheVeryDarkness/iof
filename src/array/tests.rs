mod tracked {
    use std::sync::atomic::{AtomicUsize, Ordering};

    static COUNT: AtomicUsize = AtomicUsize::new(0);
    pub struct Tracked(Box<usize>);
    impl Tracked {
        pub(super) fn new() -> Self {
            loop {
                let n = COUNT.load(Ordering::Relaxed);
                if COUNT
                    .compare_exchange(n, n + 1, Ordering::Relaxed, Ordering::Relaxed)
                    .is_ok()
                {
                    // Well, we shouldn't use println! without a lock in real code.
                    println!("Creating Tracked({})", n);
                    return Tracked(Box::new(n));
                }
            }
        }
    }
    impl Drop for Tracked {
        fn drop(&mut self) {
            loop {
                let n = COUNT.load(Ordering::Relaxed);
                assert!(n > 0);
                if COUNT
                    .compare_exchange(n, n - 1, Ordering::Relaxed, Ordering::Relaxed)
                    .is_ok()
                {
                    break;
                }
            }
            // Well, we shouldn't use println! without a lock in real code.
            println!("Dropping Tracked({})", self.0);
        }
    }

    /// Ensure all elements are dropped.
    pub(super) fn check() {
        assert_eq!(COUNT.load(Ordering::Relaxed), 0);
    }
}

mod tests {
    use super::super::array_from_fn;
    use super::tracked::{check, Tracked};
    use std::{panic::catch_unwind, thread::spawn};

    #[test]
    fn array_string() {
        let mut i = 0;
        let array: [String; 3] = array_from_fn(|| {
            i += 1;
            i.to_string()
        });
        assert_eq!(array[0], "1");
        assert_eq!(array[1], "2");
        assert_eq!(array[2], "3");
    }

    #[test]
    fn array_tracked_caught_panic() {
        let threads: Vec<_> = (0..16)
            .map(|_| {
                spawn(|| {
                    let res = catch_unwind(|| {
                        let mut i = 0;
                        let array: [Tracked; 64] = array_from_fn(|| {
                            if i >= 63 {
                                panic!("Sorry, something is wrong with the array.");
                            }
                            i += 1;
                            Tracked::new()
                        });
                        array
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
}
