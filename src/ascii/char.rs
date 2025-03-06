use std::{
    fmt::{self, Write as _},
    mem::transmute,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign},
    slice,
    str::FromStr,
};

/// An ASCII character. This is a subset of the Unicode character set.
///
/// See [ascii](std::ascii) for more information. And actually, most of this type is copied of the [`std::ascii::Char`] type.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(u8)]
pub enum Char {
    /// U+0000 (The default variant)
    Null = 0,
    /// U+0001
    StartOfHeading = 1,
    /// U+0002
    StartOfText = 2,
    /// U+0003
    EndOfText = 3,
    /// U+0004
    EndOfTransmission = 4,
    /// U+0005
    Enquiry = 5,
    /// U+0006
    Acknowledge = 6,
    /// U+0007
    Bell = 7,
    /// U+0008
    Backspace = 8,
    /// U+0009
    CharacterTabulation = 9,
    /// U+000A
    LineFeed = 10,
    /// U+000B
    LineTabulation = 11,
    /// U+000C
    FormFeed = 12,
    /// U+000D
    CarriageReturn = 13,
    /// U+000E
    ShiftOut = 14,
    /// U+000F
    ShiftIn = 15,
    /// U+0010
    DataLinkEscape = 16,
    /// U+0011
    DeviceControlOne = 17,
    /// U+0012
    DeviceControlTwo = 18,
    /// U+0013
    DeviceControlThree = 19,
    /// U+0014
    DeviceControlFour = 20,
    /// U+0015
    NegativeAcknowledge = 21,
    /// U+0016
    SynchronousIdle = 22,
    /// U+0017
    EndOfTransmissionBlock = 23,
    /// U+0018
    Cancel = 24,
    /// U+0019
    EndOfMedium = 25,
    /// U+001A
    Substitute = 26,
    /// U+001B
    Escape = 27,
    /// U+001C
    InformationSeparatorFour = 28,
    /// U+001D
    InformationSeparatorThree = 29,
    /// U+001E
    InformationSeparatorTwo = 30,
    /// U+001F
    InformationSeparatorOne = 31,
    /// U+0020
    Space = 32,
    /// U+0021
    ExclamationMark = 33,
    /// U+0022
    QuotationMark = 34,
    /// U+0023
    NumberSign = 35,
    /// U+0024
    DollarSign = 36,
    /// U+0025
    PercentSign = 37,
    /// U+0026
    Ampersand = 38,
    /// U+0027
    Apostrophe = 39,
    /// U+0028
    LeftParenthesis = 40,
    /// U+0029
    RightParenthesis = 41,
    /// U+002A
    Asterisk = 42,
    /// U+002B
    PlusSign = 43,
    /// U+002C
    Comma = 44,
    /// U+002D
    HyphenMinus = 45,
    /// U+002E
    FullStop = 46,
    /// U+002F
    Solidus = 47,
    /// U+0030
    Digit0 = 48,
    /// U+0031
    Digit1 = 49,
    /// U+0032
    Digit2 = 50,
    /// U+0033
    Digit3 = 51,
    /// U+0034
    Digit4 = 52,
    /// U+0035
    Digit5 = 53,
    /// U+0036
    Digit6 = 54,
    /// U+0037
    Digit7 = 55,
    /// U+0038
    Digit8 = 56,
    /// U+0039
    Digit9 = 57,
    /// U+003A
    Colon = 58,
    /// U+003B
    Semicolon = 59,
    /// U+003C
    LessThanSign = 60,
    /// U+003D
    EqualsSign = 61,
    /// U+003E
    GreaterThanSign = 62,
    /// U+003F
    QuestionMark = 63,
    /// U+0040
    CommercialAt = 64,
    /// U+0041
    CapitalA = 65,
    /// U+0042
    CapitalB = 66,
    /// U+0043
    CapitalC = 67,
    /// U+0044
    CapitalD = 68,
    /// U+0045
    CapitalE = 69,
    /// U+0046
    CapitalF = 70,
    /// U+0047
    CapitalG = 71,
    /// U+0048
    CapitalH = 72,
    /// U+0049
    CapitalI = 73,
    /// U+004A
    CapitalJ = 74,
    /// U+004B
    CapitalK = 75,
    /// U+004C
    CapitalL = 76,
    /// U+004D
    CapitalM = 77,
    /// U+004E
    CapitalN = 78,
    /// U+004F
    CapitalO = 79,
    /// U+0050
    CapitalP = 80,
    /// U+0051
    CapitalQ = 81,
    /// U+0052
    CapitalR = 82,
    /// U+0053
    CapitalS = 83,
    /// U+0054
    CapitalT = 84,
    /// U+0055
    CapitalU = 85,
    /// U+0056
    CapitalV = 86,
    /// U+0057
    CapitalW = 87,
    /// U+0058
    CapitalX = 88,
    /// U+0059
    CapitalY = 89,
    /// U+005A
    CapitalZ = 90,
    /// U+005B
    LeftSquareBracket = 91,
    /// U+005C
    ReverseSolidus = 92,
    /// U+005D
    RightSquareBracket = 93,
    /// U+005E
    CircumflexAccent = 94,
    /// U+005F
    LowLine = 95,
    /// U+0060
    GraveAccent = 96,
    /// U+0061
    SmallA = 97,
    /// U+0062
    SmallB = 98,
    /// U+0063
    SmallC = 99,
    /// U+0064
    SmallD = 100,
    /// U+0065
    SmallE = 101,
    /// U+0066
    SmallF = 102,
    /// U+0067
    SmallG = 103,
    /// U+0068
    SmallH = 104,
    /// U+0069
    SmallI = 105,
    /// U+006A
    SmallJ = 106,
    /// U+006B
    SmallK = 107,
    /// U+006C
    SmallL = 108,
    /// U+006D
    SmallM = 109,
    /// U+006E
    SmallN = 110,
    /// U+006F
    SmallO = 111,
    /// U+0070
    SmallP = 112,
    /// U+0071
    SmallQ = 113,
    /// U+0072
    SmallR = 114,
    /// U+0073
    SmallS = 115,
    /// U+0074
    SmallT = 116,
    /// U+0075
    SmallU = 117,
    /// U+0076
    SmallV = 118,
    /// U+0077
    SmallW = 119,
    /// U+0078
    SmallX = 120,
    /// U+0079
    SmallY = 121,
    /// U+007A
    SmallZ = 122,
    /// U+007B
    LeftCurlyBracket = 123,
    /// U+007C
    VerticalLine = 124,
    /// U+007D
    RightCurlyBracket = 125,
    /// U+007E
    Tilde = 126,
    /// U+007F
    Delete = 127,
}

impl Char {
    /// Converts a `Char` to a `u8`.
    pub const fn to_u8(self) -> u8 {
        self as u8
    }
    /// Converts a `Char` to a string slice.
    pub const fn as_str(&self) -> &str {
        Self::slice_as_str(slice::from_ref(self))
    }
    /// Converts a slice of `Char` to a string slice.
    pub const fn slice_as_str(slice: &[Self]) -> &str {
        unsafe {
            std::str::from_utf8_unchecked(slice::from_raw_parts(slice.as_ptr().cast(), slice.len()))
        }
    }
    /// Converts a mutable slice of `Char` to a mutable string slice.
    pub fn slice_as_mut_str(slice: &mut [Self]) -> &mut str {
        unsafe {
            std::str::from_utf8_unchecked_mut(slice::from_raw_parts_mut(
                slice.as_mut_ptr().cast(),
                slice.len(),
            ))
        }
    }
}

macro_rules! into_int_impl {
    ($($ty:ty)*) => {
        $(
            impl From<Char> for $ty {
                #[inline]
                fn from(chr: Char) -> $ty {
                    chr as u8 as $ty
                }
            }
        )*
    }
}

into_int_impl!(u8 u16 u32 u64 u128 char);

const HEX_DIGITS: [Char; 16] = [
    Char::Digit0,
    Char::Digit1,
    Char::Digit2,
    Char::Digit3,
    Char::Digit4,
    Char::Digit5,
    Char::Digit6,
    Char::Digit7,
    Char::Digit8,
    Char::Digit9,
    Char::SmallA,
    Char::SmallB,
    Char::SmallC,
    Char::SmallD,
    Char::SmallE,
    Char::SmallF,
];

macro_rules! impl_arith {
    ($trait:ident $fn:ident $trait_assign:ident $fn_assign:ident) => {
        impl $trait<u8> for Char {
            type Output = Char;

            fn $fn(self, rhs: u8) -> Char {
                u8::$fn(self.to_u8(), rhs).try_into().unwrap()
            }
        }
        impl $trait_assign<u8> for Char {
            fn $fn_assign(&mut self, rhs: u8) {
                *self = Self::$fn(*self, rhs);
            }
        }
    };
}

impl_arith!(Add add AddAssign add_assign);
impl_arith!(Sub sub SubAssign sub_assign);
impl_arith!(Mul mul MulAssign mul_assign);
impl_arith!(Div div DivAssign div_assign);
impl_arith!(Rem rem RemAssign rem_assign);

pub(crate) fn write_escaped(c: Char, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    use Char::{
        Apostrophe, CarriageReturn, CharacterTabulation, LineFeed, Null,
        ReverseSolidus as Backslash, ReverseSolidus, SmallX,
    };
    fn backslash(a: Char) -> ([Char; 4], usize) {
        ([Backslash, a, Null, Null], 2)
    }

    let (buf, len) = match c {
        Null => backslash(Char::Digit0),
        CharacterTabulation => backslash(Char::SmallT),
        CarriageReturn => backslash(Char::SmallR),
        LineFeed => backslash(Char::SmallN),
        ReverseSolidus => backslash(ReverseSolidus),
        Apostrophe => backslash(Apostrophe),
        _ if c.to_u8().is_ascii_control() => {
            let byte = c.to_u8();
            let hi = HEX_DIGITS[usize::from(byte >> 4)];
            let lo = HEX_DIGITS[usize::from(byte & 0xf)];
            ([Backslash, SmallX, hi, lo], 4)
        }
        _ => ([c, Null, Null, Null], 1),
    };

    f.write_str(Char::slice_as_str(&buf[..len]))
}

pub(crate) fn write_escaped_quoted(c: Char, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str("'")?;
    write_escaped(c, f)?;
    f.write_str("'")?;
    Ok(())
    // use Char::{
    //     Apostrophe, CarriageReturn, CharacterTabulation, LineFeed, Null,
    //     ReverseSolidus as Backslash, ReverseSolidus, SmallX,
    // };
    // fn backslash(a: Char) -> ([Char; 6], usize) {
    //     ([Apostrophe, Backslash, a, Apostrophe, Null, Null], 4)
    // }

    // let (buf, len) = match c {
    //     Null => backslash(Char::Digit0),
    //     CharacterTabulation => backslash(Char::SmallT),
    //     CarriageReturn => backslash(Char::SmallR),
    //     LineFeed => backslash(Char::SmallN),
    //     ReverseSolidus => backslash(ReverseSolidus),
    //     Apostrophe => backslash(Apostrophe),
    //     _ if c.to_u8().is_ascii_control() => {
    //         let byte = c.to_u8();
    //         let hi = HEX_DIGITS[usize::from(byte >> 4)];
    //         let lo = HEX_DIGITS[usize::from(byte & 0xf)];
    //         ([Apostrophe, Backslash, SmallX, hi, lo, Apostrophe], 6)
    //     }
    //     _ => ([Apostrophe, c, Apostrophe, Null, Null, Null], 3),
    // };

    // f.write_str(Char::slice_as_str(&buf[..len]))
}

impl fmt::Debug for Char {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_escaped_quoted(*self, f)
    }
}

impl fmt::Display for Char {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char((*self).into())
    }
}

impl PartialEq<u8> for Char {
    fn eq(&self, other: &u8) -> bool {
        u8::eq(&(*self).into(), other)
    }
}
impl PartialEq<Char> for u8 {
    fn eq(&self, other: &Char) -> bool {
        Char::eq(other, self)
    }
}
impl PartialEq<char> for Char {
    fn eq(&self, other: &char) -> bool {
        char::eq(&(*self).into(), other)
    }
}
impl PartialEq<Char> for char {
    fn eq(&self, other: &Char) -> bool {
        Char::eq(other, self)
    }
}

/// An error which can be returned when parsing an ASCII character.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Length(usize),
    /// Should never happen if given input is legal UTF-8.
    Byte(u8),
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Length(len) => write!(f, "invalid length: {}", len),
            Self::Byte(ch) => write!(f, "invalid byte: {:#x?}", ch),
        }
    }
}
impl std::error::Error for Error {}

impl TryFrom<u8> for Char {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, u8> {
        if value > 127 {
            return Err(value);
        }
        // Safety: `value` is guaranteed to be in the range of `Char`.
        Ok(unsafe { transmute::<u8, Char>(value) })
    }
}

impl FromStr for Char {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        if bytes.len() != 1 {
            return Err(Error::Length(bytes.len()));
        }
        let byte = bytes[0];
        byte.try_into().map_err(Error::Byte)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unwrap;

    #[test]
    #[should_panic = "invalid byte: 0x80"]
    fn try_from_u8() {
        let _ = unwrap!(Char::try_from(128).map_err(Error::Byte));
    }
}
