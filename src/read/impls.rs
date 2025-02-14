use super::fmt;
use crate::{
    impl_read_one_from_for_from_str, ASCIIChar, ASCIIString, BufReadExt, ReadError, ReadOneFrom,
    ReadOneFromError,
};
use std::{ffi::OsString, net::*, num::*, path::PathBuf};

// Implement `Parse` for all types that implement FromStr.
impl_read_one_from_for_from_str!(
    bool

    i8 u8
    i16 u16
    i32 u32
    i64 u64
    i128 u128
    isize usize

    NonZeroI8 NonZeroU8
    NonZeroI16 NonZeroU16
    NonZeroI32 NonZeroU32
    NonZeroI64 NonZeroU64
    NonZeroI128 NonZeroU128
    NonZeroIsize NonZeroUsize

    f32 f64

    /* char ASCIIChar */
    String
    ASCIIString
    PathBuf
    OsString

    IpAddr Ipv4Addr Ipv6Addr
    SocketAddr SocketAddrV4 SocketAddrV6
);

impl ReadOneFrom for char {
    type ParseError = <char as ::std::str::FromStr>::Err;

    #[inline]
    fn parse(s: &str) -> Result<Self, ReadOneFromError<Self>> {
        s.parse().map_err(|err| {
            ReadError::FromStrError(err, s.to_owned(), ::std::any::type_name::<Self>())
        })
    }

    #[inline]
    fn try_read_one_from<F: fmt::Format, S: BufReadExt>(
        stream: &mut S,
        format: &F,
    ) -> Result<Self, ReadOneFromError<Self>> {
        <Self as ReadOneFrom>::try_read_in_char_from(stream, format)
    }
}

impl ReadOneFrom for ASCIIChar {
    type ParseError = <Self as ::std::str::FromStr>::Err;

    #[inline]
    fn parse(s: &str) -> Result<Self, ReadOneFromError<Self>> {
        s.parse().map_err(|err| {
            ReadError::FromStrError(err, s.to_owned(), ::std::any::type_name::<char>())
        })
    }

    #[inline]
    fn try_read_one_from<F: fmt::Format, S: BufReadExt>(
        stream: &mut S,
        format: &F,
    ) -> Result<Self, ReadOneFromError<Self>> {
        <Self as ReadOneFrom>::try_read_in_char_from(stream, format)
    }
}
