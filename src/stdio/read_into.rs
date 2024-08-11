use crate::{
    mat::Mat,
    read_into::{MonoTuple, ReadInto, ReadIntoError},
    stdio::STDIN,
    stream::InputStream,
};
use std::str::FromStr;

macro_rules! expose_stdin {
    ($try_fn:ident $str_try_fn:literal $fn:ident $str_fn:literal [$($ty_arg:tt)*] ($($arg:ident: $arg_ty:ty), *) -> $ret:ty) => {
        /// Call [`
        #[doc = $str_try_fn]
        /// `] on [std::io::StdinLock].
        ///
        /// # Panics
        ///
        /// If [
        #[doc = $str_try_fn]
        /// ] panics.
        ///
        /// # Errors
        ///
        /// If this function is called in multiple threads, the behavior is undefined, possibly causing a deadlock.
        ///
        /// If [
        #[doc = $str_try_fn]
        /// ] returns an error.
        pub fn $try_fn<$($ty_arg)*>($($arg: $arg_ty),*) -> Result<$ret, ReadIntoError<T>>
        where
            T::Err: std::error::Error,
        {
            STDIN.with(|lock| lock.borrow_mut().$try_fn($($arg),*))
        }

        /// Call [`
        #[doc = $str_fn]
        /// `] on [std::io::StdinLock].
        ///
        /// # Panics
        ///
        /// If [
        #[doc = $str_try_fn]
        /// ] returns an error or panics.
        ///
        /// # Errors
        ///
        /// If this function is called in multiple threads, the behavior is undefined, possibly causing a deadlock.
        pub fn $fn<$($ty_arg)*>($($arg: $arg_ty),*) -> $ret
        where
            T::Err: std::error::Error,
        {
            STDIN.with(|lock| lock.borrow_mut().$fn($($arg),*))
        }
    };
}

expose_stdin!(
    try_read "ReadInto::try_read"
    read "ReadInto::read"
    [T: FromStr] () -> T
);
expose_stdin!(
    try_read_n "ReadInto::try_read_n"
    read_n "ReadInto::read_n"
    [T: FromStr] (n: usize) -> Vec<T>
);
expose_stdin!(
    try_read_m_n "ReadInto::try_read_m_n"
    read_m_n "ReadInto::read_m_n"
    [T: FromStr] (m: usize, n: usize) -> Mat<T>
);
expose_stdin!(
    try_read_array "ReadInto::try_read_array"
    read_array "ReadInto::read_array"
    [T: FromStr, const N: usize] () -> Box<[T; N]>
);
expose_stdin!(
    try_read_tuple "ReadInto::try_read_tuple"
    read_tuple "ReadInto::read_tuple"
    [T: FromStr, U: MonoTuple<T, InputStream<std::io::StdinLock<'static>>>] () -> U
);
expose_stdin!(
    try_read_remained_line "ReadInto::try_read_remained_line"
    read_remained_line "ReadInto::read_remained_line"
    [T: FromStr] () -> T
);
expose_stdin!(
    try_read_line "ReadInto::try_read_line"
    read_line "ReadInto::read_line"
    [T: FromStr] () -> T
);
expose_stdin!(
    try_read_char "ReadInto::try_read_char"
    read_char "ReadInto::read_char"
    [T: FromStr] () -> T
);
