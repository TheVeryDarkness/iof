//! Helper for initializing an array of `MaybeUninit<T>` elements.
use std::{
    array::from_fn,
    mem::{forget, MaybeUninit},
};

mod guard;

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

/// Create an array of `T` elements from a function that produces each element.
///
/// It's safe to pass a function that may panic, but the array will be dropped.
///
/// # Examples
///
/// ```rust
/// use iof::array_from_fn;
/// let array: [String; 3] = array_from_fn(|| "hello".to_string());
/// assert_eq!(array[0], "hello");
/// assert_eq!(array[1], "hello");
/// assert_eq!(array[2], "hello");
/// ```
pub fn array_from_fn<T, const N: usize>(mut f: impl FnMut() -> T) -> [T; N] {
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

/// Create an array of `T` elements from a function that produces each element with a possible error.
///
/// It's safe to pass a function that may panic, but the array will be dropped.
///
/// # Examples
///
/// ```rust
/// use iof::array_try_from_fn;
/// let array: Result<[String; 3], ()> = array_try_from_fn(|| Ok("hello".to_string()));
/// assert_eq!(array, Ok(["hello", "hello", "hello"]));
/// ```
pub fn array_try_from_fn<T, E, const N: usize>(
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
