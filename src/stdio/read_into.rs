use crate::{
    mat::Mat,
    read_into::{ReadInto, ReadIntoOne},
    stdio::stdin,
    stream::InputStream,
};
use std::io::{BufReader, Stdin};
pub use {
    read_m_n as read_mat, read_n as read_vec, try_read_m_n as try_read_mat,
    try_read_n as try_read_vec,
};

type StdinReader = InputStream<BufReader<Stdin>>;

macro_rules! expose_stdin {
    ($try_fn:ident $str_try_fn:literal $fn:ident $str_fn:literal [$ty_arg:ident] [$trait:ident] [$($trait_arg:tt)*] ($($arg:ident: $arg_ty:ty), *) -> $ret:ty | $err:ty) => {
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
            StdinReader: $trait<$($trait_arg)*>,
        {
            <StdinReader as $trait<$($trait_arg)*>>::$try_fn(&mut *stdin(), $($arg),* )
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
            StdinReader: $trait<$($trait_arg)*>,
        {
            <StdinReader as $trait<$($trait_arg)*>>::$fn(&mut *stdin(), $($arg),* )
        }
    };
}

expose_stdin!(
    try_read "ReadInto::try_read"
    read "ReadInto::read"
    [T] [ReadInto] [T] () -> T | <StdinReader as ReadInto<T>>::Error
);
expose_stdin!(
    try_read_n "ReadInto::try_read_n"
    read_n "ReadInto::read_n"
    [T] [ReadInto] [T] (n: usize) -> Vec<T> | <StdinReader as ReadInto<T>>::Error
);
expose_stdin!(
    try_read_m_n "ReadInto::try_read_m_n"
    read_m_n "ReadInto::read_m_n"
    [T] [ReadInto] [T] (m: usize, n: usize) -> Mat<T> | <StdinReader as ReadInto<T>>::Error
);

expose_stdin!(
    try_read_one "ReadIntoOne::try_read_one"
    read_one "ReadIntoOne::read_one"
    [T] [ReadIntoOne] [T] () -> T | <StdinReader as ReadIntoOne<T>>::Error
);
expose_stdin!(
    try_read_in_line_trimmed "ReadIntoOne::try_read_in_line_trimmed"
    read_in_line_trimmed "ReadIntoOne::read_in_line_trimmed"
    [T] [ReadIntoOne] [T] () -> T | <StdinReader as ReadIntoOne<T>>::Error
);
expose_stdin!(
    try_read_in_line_some_trimmed "ReadIntoOne::try_read_in_line_some_trimmed"
    read_in_line_some_trimmed "ReadIntoOne::read_in_line_some_trimmed"
    [T] [ReadIntoOne] [T] () -> T | <StdinReader as ReadIntoOne<T>>::Error
);
expose_stdin!(
    try_read_all "ReadIntoOne::try_read_all"
    read_all "ReadIntoOne::read_all"
    [T] [ReadIntoOne] [T] () -> Vec<T> | <StdinReader as ReadIntoOne<T>>::Error
);
expose_stdin!(
    try_read_all_in_line "ReadIntoOne::try_read_all_in_line"
    read_all_in_line "ReadIntoOne::read_all_in_line"
    [T] [ReadIntoOne] [T] () -> Vec<T> | <StdinReader as ReadIntoOne<T>>::Error
);
expose_stdin!(
    try_read_all_in_line_some "ReadIntoOne::try_read_all_in_line_some"
    read_all_in_line_some "ReadIntoOne::read_all_in_line_some"
    [T] [ReadIntoOne] [T] () -> Vec<T> | <StdinReader as ReadIntoOne<T>>::Error
);
expose_stdin!(
    try_read_in_char "ReadIntoOne::try_read_in_char"
    read_in_char "ReadIntoOne::read_in_char"
    [T] [ReadIntoOne] [T] () -> T | <StdinReader as ReadIntoOne<T>>::Error
);
