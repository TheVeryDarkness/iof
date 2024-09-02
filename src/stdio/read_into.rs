use crate::{
    mat::Mat,
    read_into::{ReadInto, ReadIntoSingle},
    stdio::STDIN,
    stream::InputStream,
};
use std::{
    borrow::{Borrow, BorrowMut},
    io::{BufReader, Stdin},
    sync::Mutex,
};
pub use {
    read_m_n as read_mat, read_n as read_vec, try_read_m_n as try_read_mat,
    try_read_n as try_read_vec,
};

type StdinReader = InputStream<BufReader<Stdin>>;

macro_rules! expose_stdin {
    ($try_fn:ident $str_try_fn:literal $fn:ident $str_fn:literal [$ty_arg:ident] [$trait:ident] ($($arg:ident: $arg_ty:ty), *) -> $ret:ty) => {
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
        pub fn $try_fn<$ty_arg>($($arg: $arg_ty),*) -> Result<$ret, <StdinReader as $trait<$ty_arg>>::Error>
        where
            StdinReader: $trait<$ty_arg>,
        {
            let lock: &Mutex<_> = Mutex::borrow(&STDIN);
            let mut lock = lock.lock().unwrap();
            <StdinReader as $trait<$ty_arg>>::$try_fn(&mut *lock.borrow_mut(), $($arg),* )
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
            StdinReader: $trait<$ty_arg>,
        {
            let lock: &Mutex<_> = Mutex::borrow(&STDIN);
            let mut lock = lock.lock().unwrap();
            <StdinReader as $trait<$ty_arg>>::$fn(&mut *lock.borrow_mut(), $($arg),* )
        }
    };
}

expose_stdin!(
    try_read "ReadInto::try_read"
    read "ReadInto::read"
    [T] [ReadInto] () -> T
);
expose_stdin!(
    try_read_n "ReadInto::try_read_n"
    read_n "ReadInto::read_n"
    [T] [ReadInto] (n: usize) -> Vec<T>
);
expose_stdin!(
    try_read_m_n "ReadInto::try_read_m_n"
    read_m_n "ReadInto::read_m_n"
    [T] [ReadInto] (m: usize, n: usize) -> Mat<T>
);

expose_stdin!(
    try_read_one "ReadIntoSingle::try_read_one"
    read_one "ReadIntoSingle::read_one"
    [T] [ReadIntoSingle] () -> T
);
expose_stdin!(
    try_read_in_line_trimmed "ReadIntoSingle::try_read_in_line_trimmed"
    read_in_line_trimmed "ReadIntoSingle::read_in_line_trimmed"
    [T] [ReadIntoSingle] () -> T
);
expose_stdin!(
    try_read_in_line_some_trimmed "ReadIntoSingle::try_read_in_line_some_trimmed"
    read_in_line_some_trimmed "ReadIntoSingle::read_in_line_some_trimmed"
    [T] [ReadIntoSingle] () -> T
);
expose_stdin!(
    try_read_all "ReadIntoSingle::try_read_all"
    read_all "ReadIntoSingle::read_all"
    [T] [ReadIntoSingle] () -> Vec<T>
);
expose_stdin!(
    try_read_all_in_line "ReadIntoSingle::try_read_all_in_line"
    read_all_in_line "ReadIntoSingle::read_all_in_line"
    [T] [ReadIntoSingle] () -> Vec<T>
);
expose_stdin!(
    try_read_all_in_line_some "ReadIntoSingle::try_read_all_in_line_some"
    read_all_in_line_some "ReadIntoSingle::read_all_in_line_some"
    [T] [ReadIntoSingle] () -> Vec<T>
);
expose_stdin!(
    try_read_in_char "ReadIntoSingle::try_read_in_char"
    read_in_char "ReadIntoSingle::read_in_char"
    [T] [ReadIntoSingle] () -> T
);
