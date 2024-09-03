//! Helper for initializing an array of `MaybeUninit<T>` elements.
use guard::ArrayGuard;
use std::{
    array::from_fn,
    mem::{forget, MaybeUninit},
};

mod guard;
#[cfg(test)]
mod tests;

/// Create an array of `T` elements from a function that produces each element with a possible error.
///
/// It's safe to pass a function that may panic, but the array will be dropped.
///
/// # Examples
///
/// ```rust,ignore
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
