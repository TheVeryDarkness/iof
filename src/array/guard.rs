use std::mem::MaybeUninit;

/// Helper for initializing an array of `MaybeUninit<T>` elements.
///
/// Once dropped, initialized elements in the array are also dropped.
/// Call [std::mem::forget] to drop the array without dropping the elements.
///
/// Borrow from the underlying implementation of [std::array::from_fn].
///
/// # Safety
///
/// The caller must ensure that the array is not read from until it is fully initialized,
/// the array is not used after it is dropped without calling [std::mem::forget],
/// and the length should not exceed the capacity.
pub(super) struct ArrayGuard<'a, T, const N: usize> {
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

    /// Use [usize::unchecked_add] if it's stabilized.
    pub(crate) unsafe fn push_unchecked(&mut self, value: T) {
        let _ = self.array.get_unchecked_mut(self.len).write(value);
        // Safety: We just wrote to the array.
        self.len = self.len.wrapping_add(1);
    }
}
