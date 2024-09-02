use crate::impl_read_into;
use std::{ffi::OsString, net::*, num::*, path::PathBuf};

// Implement `Parse` for all types that implement FromStr.
impl_read_into!(
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

    char
    String
    PathBuf
    OsString

    IpAddr Ipv4Addr Ipv6Addr
    SocketAddr SocketAddrV4 SocketAddrV6
);
