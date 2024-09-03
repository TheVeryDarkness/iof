use std::sync::atomic::{AtomicUsize, Ordering};

static COUNT: AtomicUsize = AtomicUsize::new(0);
pub struct Tracked(Box<(usize, usize, usize)>);
impl Tracked {
    pub(super) fn new(i: usize, j: usize) -> Self {
        loop {
            let n = COUNT.load(Ordering::Relaxed);
            if COUNT
                .compare_exchange(n, n + 1, Ordering::Relaxed, Ordering::Relaxed)
                .is_ok()
            {
                // Well, we shouldn't use println! without a lock in real code.
                println!("Creating Tracked({n}) for ({i}, {j})");
                return Tracked(Box::new((n, i, j)));
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
        let (n, i, j) = *self.0;
        // Well, we shouldn't use println! without a lock in real code.
        println!("Dropping Tracked({n}) for ({i}, {j})");
    }
}

/// Ensure all elements are dropped.
pub(super) fn check() {
    assert_eq!(COUNT.load(Ordering::Relaxed), 0);
}
