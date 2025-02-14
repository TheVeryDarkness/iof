use crate::{
    mat::Mat, read::read_from::ReadFromError, stdin, ReadFrom, ReadInto, ReadOneFrom,
    ReadOneFromError, ReadOneInto,
};
pub use read_m_n as read_mat;
pub use read_n as read_vec;
pub use try_read_m_n as try_read_mat;
pub use try_read_n as try_read_vec;

// type StdinReader = InputStream<BufReader<Stdin>>;

macro_rules! expose_stdin {
    ($try_fn:ident $fn:ident $trait_into:ident [$ty_arg:ident] [$trait:ident] [$($trait_arg:tt)*] ($($arg:ident: $arg_ty:ty), *) -> $ret:ty | $err:ty) => {
        #[doc = concat!("Call [`", stringify!($trait_into), "::", stringify!($try_fn), "`] on [stdin].")]
        ///
        /// # Panics
        ///
        #[doc = concat!("If [`", stringify!($trait_into), "::", stringify!($try_fn), "`] panics.")]
        ///
        /// # Errors
        ///
        /// If this function is called in multiple threads, the behavior is undefined, possibly causing a deadlock.
        ///
        #[doc = concat!("If [`", stringify!($trait_into), "::", stringify!($try_fn), "`] returns an error.")]
        #[inline]
        pub fn $try_fn<$ty_arg>($($arg: $arg_ty),*) -> Result<$ret, $err>
        where
            $ty_arg: $trait,
        {
            stdin().$try_fn($($arg),*)
        }

        #[doc = concat!("Unwrap the result of [`", stringify!($try_fn), "`].")]
        #[track_caller]
        #[inline]
        pub fn $fn<$ty_arg>($($arg: $arg_ty),*) -> $ret
        where
            $ty_arg: $trait,
        {
            $crate::unwrap!($try_fn::<$ty_arg>($($arg),*))
        }
    };
}

expose_stdin!(
    try_read read ReadInto
    [T] [ReadFrom] [T] () -> T | ReadFromError<T>
);
expose_stdin!(
    try_read_n read_n ReadInto
    [T] [ReadFrom] [T] (n: usize) -> Vec<T> | ReadFromError<T>
);
expose_stdin!(
    try_read_m_n read_m_n ReadInto
    [T] [ReadFrom] [T] (m: usize, n: usize) -> Mat<T> | ReadFromError<T>
);

expose_stdin!(
    try_read_one read_one ReadOneInto
    [T] [ReadOneFrom] [T] () -> T | ReadOneFromError<T>
);
expose_stdin!(
    try_read_in_line_trimmed read_in_line_trimmed ReadOneInto
    [T] [ReadOneFrom] [T] () -> T | ReadOneFromError<T>
);
expose_stdin!(
    try_read_in_line_some_trimmed read_in_line_some_trimmed ReadOneInto
    [T] [ReadOneFrom] [T] () -> T | ReadOneFromError<T>
);
expose_stdin!(
    try_read_all read_all ReadOneInto
    [T] [ReadOneFrom] [T] () -> Vec<T> | ReadOneFromError<T>
);
expose_stdin!(
    try_read_any_in_line read_any_in_line ReadOneInto
    [T] [ReadOneFrom] [T] () -> Vec<T> | ReadOneFromError<T>
);
expose_stdin!(
    try_read_some_in_line read_some_in_line ReadOneInto
    [T] [ReadOneFrom] [T] () -> Vec<T> | ReadOneFromError<T>
);
expose_stdin!(
    try_read_in_char read_in_char ReadOneInto
    [T] [ReadOneFrom] [T] () -> T | ReadOneFromError<T>
);
