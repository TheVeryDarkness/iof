use super::fmt;
use crate::{
    impl_read_one_from_for_from_str, ASCIIChar, ASCIIString, BufReadExt, ReadError, ReadOneFrom,
    ReadOneFromError,
};
use std::{ffi::OsString, net::*, path::PathBuf};

mod bool;
mod float;
mod signed;
mod unsigned;

// Implement `Parse` for some built-in types.
impl_read_one_from_for_from_str!(
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
        format: F,
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
        format: F,
    ) -> Result<Self, ReadOneFromError<Self>> {
        <Self as ReadOneFrom>::try_read_in_char_from(stream, format)
    }
}
