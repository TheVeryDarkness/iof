use super::WriteOneInto;
use crate::impl_write_into;
use std::{io, num::*};

impl_write_into!(
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

impl WriteOneInto for char {
    const SEP_ITEM: &'static str = "";

    fn try_write_one_into<S: io::Write>(&self, s: &mut S) -> io::Result<()> {
        s.write_all(&[*self as u8])
    }
}

macro_rules! impl_write_one_into_for_tuple {
    ($n0:ident $t0:ident $(, $n:ident $t:ident)* $(,)?) => {
        impl<$t0: WriteOneInto, $($t: WriteOneInto),*> WriteOneInto for ($t0, $($t,)*) {
            const SEP_ITEM: &'static str = " ";

            fn try_write_one_into<S: io::Write>(&self, s: &mut S) -> io::Result<()> {
                let ($n0, $($n, )*) = self;
                $n0.try_write_one_into(s)?;
                $(
                    s.write_all(Self::SEP_ITEM.as_bytes())?;
                    $n.try_write_one_into(s)?;
                )*
                Ok(())
            }
        }
    };
    () => {
        impl WriteOneInto for () {
            const SEP_ITEM: &'static str = " ";

            fn try_write_one_into<S: io::Write>(&self, _s: &mut S) -> io::Result<()> {
                Ok(())
            }
        }
    };
}

impl_write_one_into_for_tuple!();
impl_write_one_into_for_tuple!(t1 T1);
impl_write_one_into_for_tuple!(t1 T1, t2 T2);
impl_write_one_into_for_tuple!(t1 T1, t2 T2, t3 T3);
impl_write_one_into_for_tuple!(t1 T1, t2 T2, t3 T3, t4 T4);
impl_write_one_into_for_tuple!(t1 T1, t2 T2, t3 T3, t4 T4, t5 T5);
impl_write_one_into_for_tuple!(t1 T1, t2 T2, t3 T3, t4 T4, t5 T5, t6 T6);
impl_write_one_into_for_tuple!(t1 T1, t2 T2, t3 T3, t4 T4, t5 T5, t6 T6, t7 T7);
impl_write_one_into_for_tuple!(t1 T1, t2 T2, t3 T3, t4 T4, t5 T5, t6 T6, t7 T7, t8 T8);
impl_write_one_into_for_tuple!(t1 T1, t2 T2, t3 T3, t4 T4, t5 T5, t6 T6, t7 T7, t8 T8, t9 T9);
impl_write_one_into_for_tuple!(t1 T1, t2 T2, t3 T3, t4 T4, t5 T5, t6 T6, t7 T7, t8 T8, t9 T9, t10 T10);
impl_write_one_into_for_tuple!(t1 T1, t2 T2, t3 T3, t4 T4, t5 T5, t6 T6, t7 T7, t8 T8, t9 T9, t10 T10, t11 T11);
impl_write_one_into_for_tuple!(t1 T1, t2 T2, t3 T3, t4 T4, t5 T5, t6 T6, t7 T7, t8 T8, t9 T9, t10 T10, t11 T11, t12 T12);
