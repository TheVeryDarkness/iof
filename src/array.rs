//! Helper for initializing an array of `MaybeUninit<T>` elements.
use std::{
    array::from_fn,
    mem::{forget, MaybeUninit},
};

/// Helper for initializing an array of `MaybeUninit<T>` elements.
///
/// Once dopped, initialized elements in the array are also dropped.
/// Call [std::mem::forget] to drop the array without dropping the elements.
///
/// Borrow from the underlying implementation of [std::array::from_fn].
///
/// # Safety
///
/// The caller must ensure that the array is not read from until it is fully initialized,
/// the array is not used after it is dropped without calling [std::mem::forget],
/// and the length should not exceed the capacity.
struct ArrayGuard<'a, T, const N: usize> {
    array: &'a mut [MaybeUninit<T>; N],
    len: usize,
}

impl<T, const N: usize> Drop for ArrayGuard<'_, T, N> {
    fn drop(&mut self) {
        for i in 0..self.len {
            unsafe {
                self.array[i].as_mut_ptr().drop_in_place();
            }
        }
    }
}

impl<'a, T, const N: usize> ArrayGuard<'a, T, N> {
    /// Create a new `InitializingArray` from a mutable reference to an array of `MaybeUninit<T>`.
    pub(crate) fn new(array: &'a mut [MaybeUninit<T>; N]) -> Self {
        Self { array, len: 0 }
    }

    /// Use [usize::unchecked_add] if it's stablized.
    pub(crate) unsafe fn push_unchecked(&mut self, value: T) {
        let _ = self.array.get_unchecked_mut(self.len).write(value);
        // Safety: We just wrote to the array.
        self.len = self.len.wrapping_add(1);
    }
}

pub(crate) fn array_from_fn<T, const N: usize>(mut f: impl FnMut() -> T) -> [T; N] {
    let mut array: [MaybeUninit<T>; N] = from_fn(|_| MaybeUninit::uninit());
    let mut guard = ArrayGuard::new(&mut array);
    for _ in 0..N {
        unsafe {
            guard.push_unchecked(f());
        }
    }
    forget(guard);
    // Hope this is optimized well.
    array.map(|x| unsafe { x.assume_init() })
}

pub(crate) fn array_try_from_fn<T, E, const N: usize>(
    mut f: impl FnMut() -> Result<T, E>,
) -> Result<[T; N], E> {
    let mut array: [MaybeUninit<T>; N] = from_fn(|_| MaybeUninit::uninit());
    let mut guard = ArrayGuard::new(&mut array);
    for _ in 0..N {
        unsafe {
            guard.push_unchecked(f()?);
        }
    }
    forget(guard);
    // Hope this is optimized well.
    Ok(array.map(|x| unsafe { x.assume_init() }))
}

#[cfg(test)]
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

#[cfg(test)]
mod tests {
    use rayon::iter::{IntoParallelIterator, ParallelIterator};

    use super::{
        array_from_fn,
        tracked::{check, Tracked},
    };
    use std::panic::catch_unwind;

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
        (0..16).into_par_iter().for_each(|_| {
            let res = catch_unwind(|| {
                let mut i = 0;
                let array: [Tracked; 3] = array_from_fn(|| {
                    if i >= 2 {
                        panic!("Sorry, something is wrong with the array.");
                    }
                    i += 1;
                    Tracked::new()
                });
                array
            });
            assert!(res.is_err());
        });
        check();
    }
}
