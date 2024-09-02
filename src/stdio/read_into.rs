use crate::{
    mat::Mat,
    read_into::{ReadInto, ReadIntoSingle},
    stdio::STDIN,
    stream::InputStream,
};
use std::io::StdinLock;
pub use {
    read_m_n as read_mat, read_n as read_vec, try_read_m_n as try_read_mat,
    try_read_n as try_read_vec,
};

macro_rules! expose_stdin {
    ($try_fn:ident $str_try_fn:literal $fn:ident $str_fn:literal [$ty_arg:ident] [$trait:ident] ($($arg:ident: $arg_ty:ty), *) -> $ret:ty | $err:ty) => {
        /// Call [`
        #[doc = $str_try_fn]
        /// `] on [std::io::StdinLock].
        ///
        /// # Panics
        ///
        /// If [`
        #[doc = $str_try_fn]
        /// `] panics.
        ///
        /// # Errors
        ///
        /// If this function is called in multiple threads, the behavior is undefined, possibly causing a deadlock.
        ///
        /// If [`
        #[doc = $str_try_fn]
        /// `] returns an error.
        pub fn $try_fn<$ty_arg>($($arg: $arg_ty),*) -> Result<$ret, $err>
        where
            InputStream<StdinLock<'static>>: $trait<$ty_arg>,
        {
            STDIN.with(|lock| <InputStream<StdinLock<'static>> as $trait<$ty_arg>>::$try_fn(&mut *lock.borrow_mut(), $($arg),* ))
        }

        /// Call [`
        #[doc = $str_fn]
        /// `] on [std::io::StdinLock].
        ///
        /// # Panics
        ///
        /// If [`
        #[doc = $str_try_fn]
        /// `] returns an error or panics.
        ///
        /// # Errors
        ///
        /// If this function is called in multiple threads, the behavior is undefined, possibly causing a deadlock.
        pub fn $fn<$ty_arg>($($arg: $arg_ty),*) -> $ret
        where
            InputStream<StdinLock<'static>>: $trait<$ty_arg>,
        {
            STDIN.with(|lock| <InputStream<StdinLock<'static>> as $trait<$ty_arg>>::$fn(&mut *lock.borrow_mut(), $($arg),* ))
        }
    };
}

expose_stdin!(
    try_read "ReadInto::try_read"
    read "ReadInto::read"
    [T] [ReadInto] () -> T | <InputStream<StdinLock<'static>> as ReadInto<T>>::Error
);
expose_stdin!(
    try_read_n "ReadInto::try_read_n"
    read_n "ReadInto::read_n"
    [T] [ReadInto] (n: usize) -> Vec<T> | <InputStream<StdinLock<'static>> as ReadInto<T>>::Error
);
expose_stdin!(
    try_read_m_n "ReadInto::try_read_m_n"
    read_m_n "ReadInto::read_m_n"
    [T] [ReadInto] (m: usize, n: usize) -> Mat<T> | <InputStream<StdinLock<'static>> as ReadInto<T>>::Error
);
expose_stdin!(
    try_read_one "ReadIntoSingle::try_read_one"
    read_one "ReadIntoSingle::read_one"
    [T] [ReadIntoSingle] () -> T | <InputStream<StdinLock<'static>> as ReadIntoSingle<T>>::Error
);
expose_stdin!(
    try_read_in_line_trimmed "ReadIntoSingle::try_read_in_line_trimmed"
    read_in_line_trimmed "ReadIntoSingle::read_in_line_trimmed"
    [T] [ReadIntoSingle] () -> T | <InputStream<StdinLock<'static>> as ReadIntoSingle<T>>::Error
);
expose_stdin!(
    try_read_in_line_some_trimmed "ReadIntoSingle::try_read_in_line_some_trimmed"
    read_in_line_some_trimmed "ReadIntoSingle::read_in_line_some_trimmed"
    [T] [ReadIntoSingle] () -> T | <InputStream<StdinLock<'static>> as ReadIntoSingle<T>>::Error
);
expose_stdin!(
    try_read_in_char "ReadIntoSingle::try_read_in_char"
    read_in_char "ReadIntoSingle::read_in_char"
    [T] [ReadIntoSingle] () -> T | <InputStream<StdinLock<'static>> as ReadIntoSingle<T>>::Error
);
