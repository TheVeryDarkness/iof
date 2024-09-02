use crate::impl_write_into;
use std::{io, num::*};

use super::WriteInto;

impl_write_into!(
    f32 f64
    bool
    char &str String

    i8 i16 i32 i64 i128 isize
    u8 u16 u32 u64 u128 usize

    NonZeroI8 NonZeroU8
    NonZeroI16 NonZeroU16
    NonZeroI32 NonZeroU32
    NonZeroI64 NonZeroU64
    NonZeroI128 NonZeroU128
    NonZeroIsize NonZeroUsize
);

impl<T: WriteInto + ?Sized> WriteInto for &T {
    fn try_write_into<S: io::Write>(&self, s: &mut S) -> io::Result<()> {
        WriteInto::try_write_into(*self, s)
    }
}
