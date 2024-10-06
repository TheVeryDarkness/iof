use super::{dimension::*, Separator, WriteInto};
use crate::impl_for_single;
use std::{io, num::*};

impl_for_single!(
    f32 f64
    bool
    str String

    i8 i16 i32 i64 i128 isize
    u8 u16 u32 u64 u128 usize

    NonZeroI8 NonZeroU8
    NonZeroI16 NonZeroU16
    NonZeroI32 NonZeroU32
    NonZeroI64 NonZeroU64
    NonZeroI128 NonZeroU128
    NonZeroIsize NonZeroUsize
);

impl WriteInto for char {
    fn try_write_into_with_sep<S: io::Write + ?Sized>(
        &self,
        s: &mut S,
        sep: &[impl Separator],
    ) -> super::Result {
        debug_assert_eq!(sep.len(), Self::DIMENSION);
        s.write_all(&[*self as u8])
    }
}

impl Dimension for char {
    const DIMENSION: usize = 0;
    const SPACE: bool = false;
}

macro_rules! check_separators_count {
    ($sep:expr, $t0:ty $(, $t:ty)*) => {
        debug_assert_eq!($sep.len(), Self::DIMENSION, "Separator count mismatch.");
        $(
            debug_assert_eq!(
                <$t0 as Dimension>::DIMENSION,
                <$t as Dimension>::DIMENSION,
                "Dimension mismatch: {} != {}",
                <$t0 as Dimension>::DIMENSION,
                <$t as Dimension>::DIMENSION,
            );
        )*
    };
    ($sep:expr $(,)?) => {
        debug_assert_eq!($sep.len(), Self::DIMENSION, "Separator count mismatch.");
    };
}

macro_rules! impl_for_tuple {
    ($n0:ident $t0:ident $(, $n:ident $t:ident)+ $(,)?) => {
        impl<$t0: WriteInto, $($t: WriteInto),*> WriteInto for ($t0, $($t,)*) {
            fn try_write_into_with_sep<S: io::Write + ?Sized>(&self, s: &mut S, sep: &[impl Separator]) -> io::Result<()> {
                check_separators_count!(sep, $t0 $(, $t)*);

                let ($n0, $($n, )*) = self;
                let (sep, residual) = sep.split_first().expect("Separator count mismatch.");

                $n0.try_write_into_with_sep(s, residual)?;
                let mut i = 0usize;
                $(
                    sep.format(i, |args| s.write_fmt(args))?;
                    $n.try_write_into_with_sep(s, residual)?;
                    #[allow(unused_assignments)]
                    {
                        i += 1
                    }
                )*

                Ok(())
            }
        }
        impl<$t0: Dimension, $($t: Dimension,)*> Dimension for ($t0, $($t,)*) {
            const DIMENSION: usize = 1 + $t0::DIMENSION;
            const SPACE: bool = true;
        }
    };
    ($n0:ident $t0:ident $(,)?) => {
        impl<$t0: WriteInto> WriteInto for ($t0, ) {
            fn try_write_into_with_sep<S: io::Write + ?Sized>(&self, s: &mut S, sep: &[impl Separator]) -> io::Result<()> {
                check_separators_count!(sep, $t0);
                let ($n0, ) = self;
                let (_sep, residual) = sep.split_first().expect("Separator count mismatch.");
                $n0.try_write_into_with_sep(s, residual)?;
                Ok(())
            }
        }
        impl<$t0: Dimension> Dimension for ($t0, ) {
            const DIMENSION: usize = 1 + $t0::DIMENSION;
            const SPACE: bool = true;
        }
    };
    () => {
        impl WriteInto for () {
            fn try_write_into_with_sep<S: io::Write + ?Sized>(&self, _s: &mut S, _sep: &[impl Separator]) -> io::Result<()> {
                check_separators_count!(_sep, );
                Ok(())
            }
        }
        impl Dimension for () {
            const DIMENSION: usize = 0;
            const SPACE: bool = true;
        }
    };
}

impl_for_tuple!();
impl_for_tuple!(t1 T1);
impl_for_tuple!(t1 T1, t2 T2);
impl_for_tuple!(t1 T1, t2 T2, t3 T3);
impl_for_tuple!(t1 T1, t2 T2, t3 T3, t4 T4);
impl_for_tuple!(t1 T1, t2 T2, t3 T3, t4 T4, t5 T5);
impl_for_tuple!(t1 T1, t2 T2, t3 T3, t4 T4, t5 T5, t6 T6);
impl_for_tuple!(t1 T1, t2 T2, t3 T3, t4 T4, t5 T5, t6 T6, t7 T7);
impl_for_tuple!(t1 T1, t2 T2, t3 T3, t4 T4, t5 T5, t6 T6, t7 T7, t8 T8);
impl_for_tuple!(t1 T1, t2 T2, t3 T3, t4 T4, t5 T5, t6 T6, t7 T7, t8 T8, t9 T9);
impl_for_tuple!(t1 T1, t2 T2, t3 T3, t4 T4, t5 T5, t6 T6, t7 T7, t8 T8, t9 T9, t10 T10);
impl_for_tuple!(t1 T1, t2 T2, t3 T3, t4 T4, t5 T5, t6 T6, t7 T7, t8 T8, t9 T9, t10 T10, t11 T11);
impl_for_tuple!(t1 T1, t2 T2, t3 T3, t4 T4, t5 T5, t6 T6, t7 T7, t8 T8, t9 T9, t10 T10, t11 T11, t12 T12);
