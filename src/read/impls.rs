use super::fmt;
use crate::{
    ext::{Pattern, State},
    impl_read_one_from_for_from_str, ASCIIChar, ASCIIString, BufReadExt, ReadError, ReadOneFrom,
    ReadOneFromError,
};
use std::{ffi::OsString, net::*, num::*, path::PathBuf};

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

#[derive(Debug, Default, Clone, Copy)]
struct Alphabet;

impl Pattern for Alphabet {
    type Item = char;

    #[inline]
    fn step(&mut self, c: <Self as Pattern>::Item) -> bool {
        c.is_ascii_alphabetic()
    }

    #[inline]
    fn state(&self) -> State {
        State::Stoppable
    }
}

// See <https://doc.rust-lang.org/std/primitive.bool.html#impl-FromStr-for-bool>
impl_read_one_from_for_from_str!(
    // `bool` is a special case, which only accepts `"true"` and `"false"`.
    bool => Alphabet
);

// See <https://doc.rust-lang.org/std/primitive.f64.html#impl-FromStr-for-f64>
#[derive(Debug, Clone, Copy, Default)]
enum Float {
    #[default]
    Initial,
    /// `Sign`.
    Sign,

    /// `Sign? 'i'`.
    I,
    /// `Sign? 'in'`.
    In,
    /// `Sign? 'inf'`.
    Inf,
    /// `Sign? 'infi'`.
    Infi,
    /// `Sign? 'infin'`.
    Infin,
    /// `Sign? 'infini'`.
    Infini,
    /// `Sign? 'infinit'`.
    Infinit,
    /// `Sign? 'infinity'`.
    Infinity,

    /// `Sign? 'n'`.
    N,
    /// `Sign? 'na'`.
    Na,
    /// `Sign? 'nan'`.
    Nan,

    /// `Sign? Digit+`.
    Digits,
    /// `Sign? Dot`.
    Dot,
    /// `Sign? Digit+ Dot`.
    DotAfterDigits,
    /// `Sign? Digit* Dot Digit+`.
    DigitsAfterDot,
    /// `Sign? (Digit+ | Digit+ Dot Digit* | Digit* Dot Digit+) 'e'`.
    Exponent,
    /// `Sign? (Digit+ | Digit+ Dot Digit* | Digit* Dot Digit+) 'e' Sign`.
    SignAfterExponent,
    /// `Sign? (Digit+ | Digit+ Dot Digit* | Digit* Dot Digit+) 'e' Sign? Digit+`.
    DigitAfterExponent,
    // Overrun,
}

impl Pattern for Float {
    type Item = char;

    #[inline]
    fn step(&mut self, c: <Self as Pattern>::Item) -> bool {
        match self {
            Self::Initial => match c {
                '+' | '-' => *self = Self::Sign,
                '.' => *self = Self::Dot,
                _ if c.is_ascii_digit() => *self = Self::Digits,
                'I' | 'i' => *self = Self::I,
                'N' | 'n' => *self = Self::N,
                _ => return false,
            },
            Self::Sign => match c {
                '.' => *self = Self::Dot,
                _ if c.is_ascii_digit() => *self = Self::Digits,
                'I' | 'i' => *self = Self::I,
                'N' | 'n' => *self = Self::N,
                _ => return false,
            },
            Self::I => {
                match c {
                    'N' | 'n' => *self = Self::In,
                    _ => return false,
                }
                // if c.is_ascii_digit() {
                //     Ok(false)
                // } else {
                //     Err(PatternError::UnexpectedChar(c))
                // }
            }
            Self::In => match c {
                'F' | 'f' => *self = Self::Inf,
                _ => return false,
            },
            Self::Inf => match c {
                'I' | 'i' => *self = Self::Infi,
                _ => return false,
            },
            Self::Infi => match c {
                'N' | 'n' => *self = Self::Infin,
                _ => return false,
            },
            Self::Infin => match c {
                'I' | 'i' => *self = Self::Infini,
                _ => return false,
            },
            Self::Infini => match c {
                'T' | 't' => *self = Self::Infinit,
                _ => return false,
            },
            Self::Infinit => match c {
                'Y' | 'y' => *self = Self::Infinity,
                _ => return false,
            },
            Self::Infinity => return false,
            Self::N => match c {
                'A' | 'a' => *self = Self::Na,
                _ => return false,
            },
            Self::Na => match c {
                'N' | 'n' => *self = Self::Nan,
                _ => return false,
            },
            Self::Nan => return false,
            Self::Digits => match c {
                '.' => *self = Self::DotAfterDigits,
                'e' | 'E' => *self = Self::Exponent,
                _ if c.is_ascii_digit() => {}
                _ => return false,
            },
            Self::Dot | Self::DotAfterDigits => match c {
                'e' | 'E' => *self = Self::Exponent,
                _ if c.is_ascii_digit() => *self = Self::DigitsAfterDot,
                _ => return false,
            },
            Self::DigitsAfterDot => match c {
                'e' | 'E' => *self = Self::Exponent,
                _ if c.is_ascii_digit() => {}
                _ => return false,
            },
            Self::Exponent => {
                match c {
                    '+' | '-' => *self = Self::SignAfterExponent,
                    _ if c.is_ascii_digit() => *self = Self::DigitAfterExponent,
                    _ => return false,
                }
                // if c == '+' || c == '-' {
                //     *self = Self::Alphabet;
                //     Ok(true)
                // } else if c.is_ascii_digit() {
                //     Ok(false)
                // } else {
                //     Err(PatternError::UnexpectedChar(c))
                // }
            }
            Self::SignAfterExponent => match c {
                _ if c.is_ascii_digit() => *self = Self::DigitAfterExponent,
                _ => return false,
            },
            Self::DigitAfterExponent => match c {
                _ if c.is_ascii_digit() => {}
                _ => return false,
            },
            // Self::Overrun => {}
        }
        true
    }

    fn state(&self) -> State {
        match self {
            Self::Inf
            | Self::Infinity
            | Self::Nan
            | Self::Digits
            | Self::DotAfterDigits
            | Self::DigitsAfterDot
            | Self::DigitAfterExponent => State::Stoppable,
            // Self::Overrun => State::Overrun,
            _ => State::Unfulfilled,
        }
    }
}

// See <https://doc.rust-lang.org/std/primitive.f64.html#impl-FromStr-for-f64>
impl_read_one_from_for_from_str!(
    f32 f64 => Float::Initial
);

#[derive(Debug, Clone, Copy)]
enum Unsigned {
    Initial,
    Sign,
    Digits,
    // Overrun,
}

impl Pattern for Unsigned {
    type Item = char;

    #[inline]
    fn step(&mut self, c: <Self as Pattern>::Item) -> bool {
        match self {
            Self::Initial => {
                match c {
                    '+' => *self = Self::Sign,
                    _ if c.is_ascii_digit() => *self = Self::Digits,
                    _ => return false,
                }
                // if c == '+' {
                //     *self = Self::Sign;
                //     Ok(true)
                // } else if c.is_ascii_digit() {
                //     *self = Self::Digits;
                //     Ok(false)
                // } else {
                //     Err(PatternError::UnexpectedChar(c))
                // }
            }
            Self::Sign => {
                match c {
                    _ if c.is_ascii_digit() => *self = Self::Digits,
                    _ => return false,
                }
                // if c.is_ascii_digit() {
                //     *self = Self::Digits;
                //     Ok(false)
                // } else {
                //     Err(PatternError::UnexpectedChar(c))
                // }
            }
            Self::Digits => return c.is_ascii_digit(),
            // Self::Overrun => {}
        }
        true
    }

    #[inline]
    fn state(&self) -> State {
        match self {
            Self::Digits => State::Stoppable,
            // Self::Overrun => State::Overrun,
            Self::Initial | Self::Sign => State::Unfulfilled,
        }
    }
}

impl_read_one_from_for_from_str!(
    u8
    u16
    u32
    u64
    u128
    usize

    NonZeroU8
    NonZeroU16
    NonZeroU32
    NonZeroU64
    NonZeroU128
    NonZeroUsize

    => Unsigned::Initial
);

#[derive(Debug, Clone, Copy)]
enum Signed {
    Initial,
    Sign,
    Digits,
    // Overrun,
}

impl Pattern for Signed {
    type Item = char;

    #[inline]
    fn step(&mut self, c: <Self as Pattern>::Item) -> bool {
        match self {
            Self::Initial => {
                match c {
                    '+' | '-' => *self = Self::Sign,
                    _ if c.is_ascii_digit() => *self = Self::Digits,
                    _ => return false,
                }
                // if matches!(c, '+' | '-') {
                //     *self = Self::Sign;
                //     Ok(true)
                // } else if c.is_ascii_digit() {
                //     *self = Self::Digits;
                //     Ok(false)
                // } else {
                //     Err(PatternError::UnexpectedChar(c))
                // }
            }
            Self::Sign => {
                match c {
                    _ if c.is_ascii_digit() => *self = Self::Digits,
                    _ => return false,
                }
                // if c.is_ascii_digit() {
                //     *self = Self::Digits;
                //     Ok(false)
                // } else {
                //     Err(PatternError::UnexpectedChar(c))
                // }
            }
            Self::Digits => return c.is_ascii_digit(),
            // Self::Overrun => {}
        }
        true
    }

    #[inline]
    fn state(&self) -> State {
        match self {
            Self::Digits => State::Stoppable,
            // Self::Overrun => State::Overrun,
            Self::Initial | Self::Sign => State::Unfulfilled,
        }
    }
}

impl_read_one_from_for_from_str!(
    i8
    i16
    i32
    i64
    i128
    isize

    NonZeroI8
    NonZeroI16
    NonZeroI32
    NonZeroI64
    NonZeroI128
    NonZeroIsize

    => Signed::Initial
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
