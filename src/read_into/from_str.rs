use std::{
    ffi::OsString,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
    num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
        NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
    },
    path::PathBuf,
    str,
};

/// Parse a value from a string. Similar to [str::FromStr], but we need this to avoid conflicting.
///
/// # Example
///
/// ```rust
/// use iof::FromStr;
/// let x: i32 = FromStr::from_str("42").unwrap();
/// assert_eq!(x, 42);
/// ```
pub trait FromStr: Sized {
    /// Error that comes from [FromStr::from_str].
    type Err: std::error::Error;

    /// Parse a string into a type.
    fn from_str(s: &str) -> Result<Self, Self::Err>;
}

/// Implement [FromStr] for given types that already implement [str::FromStr].
#[macro_export]
macro_rules! impl_from_str {
    ($($tys:ty)*) => {
        $(
            impl $crate::read_into::FromStr for $tys {
                type Err = <$tys as ::std::str::FromStr>::Err;
                fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
                    <$tys as ::std::str::FromStr>::from_str(s)
                }
            }
        )*
    };
}

// Implement `Parse` for all types that implement FromStr.
impl_from_str!(
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
