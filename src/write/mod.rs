use crate::{stdout, SepBy, Separators};
use dimension::Dimension;
use separator::Separator;
use separators::DefaultSeparator;
use std::{
    collections::{BTreeSet, BinaryHeap, HashSet, LinkedList, VecDeque},
    io::{self, Write},
};

pub mod dimension;
mod impls;
mod macros;
pub(super) mod sep_by;
pub mod separator;
pub(super) mod separators;
pub(super) mod writer;

type Result<T = ()> = io::Result<T>;

/// Write into a stream.
///
/// - Most types that implement [std::fmt::Display] also implement this.
/// - [Vec] and `[T]` where `T` implements [std::fmt::Display] also implements this.
///   They write each item separated by a space.
/// - [Mat] where `T` implements [std::fmt::Display] also implements this.
///   They write each row separated by a newline, and each item in a row separated by a space.
///
/// [Mat]: crate::Mat
pub trait WriteInto: Dimension {
    /// Write into a stream with given separator.
    fn try_write_into_with_sep<S: Write + ?Sized>(&self, s: &mut S, sep: impl Separators)
        -> Result;
    /// Write into a stream using the default separator.
    #[inline]
    fn try_write_into<S: Write + ?Sized>(&self, s: &mut S) -> Result {
        self.try_write_into_with_sep(s, DefaultSeparator::new())
    }
    /// Write into a string with given separator.
    #[inline]
    fn try_write_into_string_with_sep(&self, sep: impl Separators) -> Result<String> {
        let mut s = Vec::new();
        self.try_write_into_with_sep(&mut s, sep)?;
        // What if the string is not valid UTF-8?
        String::from_utf8(s).map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))
    }
    /// Write into a string using the default separator.
    #[inline]
    fn try_write_into_string(&self) -> Result<String> {
        self.try_write_into_string_with_sep(DefaultSeparator::new())
    }
    /// Write into [std::io::Stdout] with given separator.
    #[inline]
    fn try_write_with_sep(&self, sep: impl Separators) -> Result {
        self.try_write_into_with_sep(&mut stdout(), sep)
    }
    /// Write into [std::io::Stdout] using the default separator.
    #[inline]
    fn try_write(&self) -> Result {
        self.try_write_with_sep(DefaultSeparator::new())
    }
}

impl<T: WriteInto> WriteInto for Vec<T> {
    #[inline]
    fn try_write_into_with_sep<S: Write + ?Sized>(
        &self,
        s: &mut S,
        sep: impl Separators,
    ) -> Result<()> {
        self.as_slice().try_write_into_with_sep(s, sep)
    }
}

impl<T: WriteInto, const N: usize> WriteInto for [T; N] {
    #[inline]
    fn try_write_into_with_sep<S: Write + ?Sized>(
        &self,
        s: &mut S,
        sep: impl Separators,
    ) -> Result<()> {
        self.as_slice().try_write_into_with_sep(s, sep)
    }
}

macro_rules! impl_write_into_for_into_iter {
    ($ty:ty) => {
        impl<T: WriteInto> WriteInto for $ty {
            #[inline]
            fn try_write_into_with_sep<S: Write + ?Sized>(
                &self,
                s: &mut S,
                sep: impl Separators,
            ) -> Result<()> {
                let (sep, residual) = sep.split();
                if let Some(sep) = &sep {
                    WriteInto::try_write_into_with_sep(&self.sep_by_write_into(sep), s, residual)
                } else {
                    WriteInto::try_write_into_with_sep(
                        &self.sep_by_write_into(Self::get_default_separator()),
                        s,
                        residual,
                    )
                }
            }
        }
    };
}

impl_write_into_for_into_iter!([T]);
impl_write_into_for_into_iter!(HashSet<T>);
impl_write_into_for_into_iter!(BTreeSet<T>);
impl_write_into_for_into_iter!(VecDeque<T>);
impl_write_into_for_into_iter!(BinaryHeap<T>);
impl_write_into_for_into_iter!(LinkedList<T>);

macro_rules! impl_write_into_for_deref {
    ($ty:ty) => {
        impl<T: WriteInto + ?Sized> WriteInto for $ty {
            #[inline]
            fn try_write_into_with_sep<S: Write + ?Sized>(
                &self,
                s: &mut S,
                sep: impl Separators,
            ) -> Result<()> {
                <T as WriteInto>::try_write_into_with_sep(self, s, sep)
            }
        }
    };
}

impl_write_into_for_deref!(&T);
impl_write_into_for_deref!(&mut T);
impl_write_into_for_deref!(Box<T>);
impl_write_into_for_deref!(std::rc::Rc<T>);
impl_write_into_for_deref!(std::sync::Arc<T>);
