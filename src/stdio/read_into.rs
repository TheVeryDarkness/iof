use crate::{
    mat::Mat,
    read_into::{parse::Parse, ReadInto, ReadIntoSingle},
    stdio::STDIN,
    stream::InputStream,
};
use std::{io::StdinLock, ops::DerefMut};

macro_rules! expose_stdin {
    ($try_fn:ident $str_try_fn:literal $fn:ident $str_fn:literal [$($ty_arg:tt)*] ($($arg:ident: $arg_ty:ty), *) -> $ret:ty | $err:ty) => {
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
        pub fn $try_fn<$($ty_arg)*>($($arg: $arg_ty),*) -> Result<$ret, $err>
        where
            InputStream<StdinLock<'static>>: ReadInto<T>,
        {
            STDIN.with(|lock| lock.borrow_mut().deref_mut().$try_fn($($arg),*))
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
        pub fn $fn<$($ty_arg)*>($($arg: $arg_ty),*) -> $ret
        where
            InputStream<StdinLock<'static>>: ReadInto<T>,
        {
            STDIN.with(|lock| lock.borrow_mut().deref_mut().$fn($($arg),*))
        }
    };
}

expose_stdin!(
    try_read "ReadInto::try_read"
    read "ReadInto::read"
    [T] () -> T | <InputStream<StdinLock<'static>> as ReadInto<T>>::Error
);
expose_stdin!(
    try_read_n "ReadInto::try_read_n"
    read_n "ReadInto::read_n"
    [T] (n: usize) -> Vec<T> | <InputStream<StdinLock<'static>> as ReadInto<T>>::Error
);
expose_stdin!(
    try_read_m_n "ReadInto::try_read_m_n"
    read_m_n "ReadInto::read_m_n"
    [T] (m: usize, n: usize) -> Mat<T> | <InputStream<StdinLock<'static>> as ReadInto<T>>::Error
);
expose_stdin!(
    try_read_remained_line "ReadIntoSingle::try_read_remained_line"
    read_remained_line "ReadIntoSingle::read_remained_line"
    [T: Parse] () -> T | <InputStream<StdinLock<'static>> as ReadIntoSingle<T>>::Error
);
expose_stdin!(
    try_read_line "ReadIntoSingle::try_read_line"
    read_line "ReadIntoSingle::read_line"
    [T: Parse] () -> T | <InputStream<StdinLock<'static>> as ReadIntoSingle<T>>::Error
);
expose_stdin!(
    try_read_char "ReadIntoSingle::try_read_char"
    read_char "ReadIntoSingle::read_char"
    [T: Parse] () -> T | <InputStream<StdinLock<'static>> as ReadIntoSingle<T>>::Error
);
