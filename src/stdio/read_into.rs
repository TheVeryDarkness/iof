use crate::{
    mat::Mat, read::read_from::ReadFromError, stdin, ReadFrom, ReadInto, ReadOneFrom,
    ReadOneFromError, ReadOneInto,
};
pub use {
    read_m_n as read_mat, read_n as read_vec, try_read_m_n as try_read_mat,
    try_read_n as try_read_vec,
};

// type StdinReader = InputStream<BufReader<Stdin>>;

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
            $ty_arg: $trait,
        {
            stdin().$try_fn($($arg),*)
        }

        /// Unwrap the result of [`
        #[doc = $str_try_fn]
        /// `].
        #[track_caller]
        pub fn $fn<$ty_arg>($($arg: $arg_ty),*) -> $ret
        where
            $ty_arg: $trait,
        {
            $crate::unwrap!($try_fn::<$ty_arg>($($arg),*))
        }
    };
}

expose_stdin!(
    try_read "ReadInto::try_read"
    read "ReadInto::read"
    [T] [ReadFrom] [T] () -> T | ReadFromError<T>
);
expose_stdin!(
    try_read_n "ReadInto::try_read_n"
    read_n "ReadInto::read_n"
    [T] [ReadFrom] [T] (n: usize) -> Vec<T> | ReadFromError<T>
);
expose_stdin!(
    try_read_m_n "ReadInto::try_read_m_n"
    read_m_n "ReadInto::read_m_n"
    [T] [ReadFrom] [T] (m: usize, n: usize) -> Mat<T> | ReadFromError<T>
);

expose_stdin!(
    try_read_one "ReadOneInto::try_read_one"
    read_one "ReadOneInto::read_one"
    [T] [ReadOneFrom] [T] () -> T | ReadOneFromError<T>
);
expose_stdin!(
    try_read_in_line_trimmed "ReadOneInto::try_read_in_line_trimmed"
    read_in_line_trimmed "ReadOneInto::read_in_line_trimmed"
    [T] [ReadOneFrom] [T] () -> T | ReadOneFromError<T>
);
expose_stdin!(
    try_read_in_line_some_trimmed "ReadOneInto::try_read_in_line_some_trimmed"
    read_in_line_some_trimmed "ReadOneInto::read_in_line_some_trimmed"
    [T] [ReadOneFrom] [T] () -> T | ReadOneFromError<T>
);
expose_stdin!(
    try_read_all "ReadOneInto::try_read_all"
    read_all "ReadOneInto::read_all"
    [T] [ReadOneFrom] [T] () -> Vec<T> | ReadOneFromError<T>
);
expose_stdin!(
    try_read_any_in_line "ReadOneInto::try_read_any_in_line"
    read_any_in_line "ReadOneInto::read_any_in_line"
    [T] [ReadOneFrom] [T] () -> Vec<T> | ReadOneFromError<T>
);
expose_stdin!(
    try_read_some_in_line "ReadOneInto::try_read_some_in_line"
    read_some_in_line "ReadOneInto::read_all_in_line"
    [T] [ReadOneFrom] [T] () -> Vec<T> | ReadOneFromError<T>
);
expose_stdin!(
    try_read_in_char "ReadOneInto::try_read_in_char"
    read_in_char "ReadOneInto::read_in_char"
    [T] [ReadOneFrom] [T] () -> T | ReadOneFromError<T>
);
