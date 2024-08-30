use std::{
    ffi::OsString,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
    num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
        NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
    },
    path::PathBuf,
    str::FromStr,
};

/// Parse a string into a type. Similar to [std::str::FromStr], but we need this to avoid conflicting.
///
/// # Example
///
/// ```rust
/// use iof::Parse;
/// let x: i32 = Parse::parse("42").unwrap();
/// assert_eq!(x, 42);
/// ```
pub trait Parse: Sized {
    /// Error that comes from [Parse].
    type Err: std::error::Error;

    /// Parse a string into a type.
    fn parse(s: &str) -> Result<Self, Self::Err>;
}

macro_rules! impl_parse {
    ($ty:ty) => {
        impl Parse for $ty {
            type Err = <$ty as FromStr>::Err;
            fn parse(s: &str) -> Result<Self, Self::Err> {
                <$ty as FromStr>::from_str(s)
            }
        }
    };
}
macro_rules! impl_parse_for {
    ($($tys:ty)*) => {
        $(impl_parse!($tys);)*
    };
}

// Implement `Parse` for all types that implement FromStr.
impl_parse_for!(
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
