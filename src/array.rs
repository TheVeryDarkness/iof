//! Helper for initializing an array of `MaybeUninit<T>` elements.
use std::mem::MaybeUninit;

pub(crate) struct InitializingArray<'a, T, const N: usize> {
    array: &'a mut [MaybeUninit<T>; N],
    len: usize,
}

impl<T, const N: usize> Drop for InitializingArray<'_, T, N> {
    fn drop(&mut self) {
        for i in 0..self.len {
            unsafe {
                self.array[i].as_mut_ptr().drop_in_place();
            }
        }
    }
}

impl<'a, T, const N: usize> InitializingArray<'a, T, N> {
    pub(crate) fn new(array: &'a mut [MaybeUninit<T>; N]) -> Self {
        Self { array, len: 0 }
    }

    /// Use [usize::unchecked_add] if it's stablized.
    pub(crate) unsafe fn push_unchecked(&mut self, value: T) {
        self.array.get_unchecked_mut(self.len).write(value);
        // Safety: We just wrote to the array.
        self.len = self.len.wrapping_add(1);
    }
}
