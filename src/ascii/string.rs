use super::char::{write_escaped, Char};
use std::{
    fmt,
    ops::{
        Deref, DerefMut, Index, IndexMut, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo,
        RangeToInclusive,
    },
    str::FromStr,
};

/// A string that only contains ASCII characters.
///
/// Similar to [`std::string::String`].
#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct String {
    bytes: Vec<Char>,
}

impl String {
    /// Create a new empty string.
    pub const fn new() -> Self {
        Self { bytes: Vec::new() }
    }
    /// Convert the string to a slice of ASCII characters.
    pub fn as_str(&self) -> &str {
        Char::slice_as_str(self.bytes.as_slice())
    }
    /// Convert the string to a mutable slice of ASCII characters.
    pub fn as_mut_str(&mut self) -> &mut str {
        Char::slice_as_mut_str(self.bytes.as_mut_slice())
    }
}

impl IntoIterator for String {
    type Item = Char;
    type IntoIter = std::vec::IntoIter<Char>;

    fn into_iter(self) -> Self::IntoIter {
        self.bytes.into_iter()
    }
}

impl Deref for String {
    type Target = [Char];

    fn deref(&self) -> &Self::Target {
        self.bytes.as_slice()
    }
}
impl DerefMut for String {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.bytes.as_mut_slice()
    }
}

impl PartialEq<str> for String {
    fn eq(&self, other: &str) -> bool {
        str::eq(self.as_str(), other)
    }
}
impl PartialEq<String> for str {
    fn eq(&self, other: &String) -> bool {
        str::eq(self, other.as_str())
    }
}

macro_rules! forward_index_impl {
    ($ty:ty) => {
        impl Index<$ty> for String {
            type Output = <Vec<Char> as Index<$ty>>::Output;

            fn index(&self, index: $ty) -> &Self::Output {
                &self.bytes[index]
            }
        }
        impl IndexMut<$ty> for String {
            fn index_mut(&mut self, index: $ty) -> &mut Self::Output {
                &mut self.bytes[index]
            }
        }
    };
}

forward_index_impl!(usize);
forward_index_impl!(Range<usize>);
forward_index_impl!(RangeInclusive<usize>);
forward_index_impl!(RangeTo<usize>);
forward_index_impl!(RangeToInclusive<usize>);
forward_index_impl!(RangeFrom<usize>);
forward_index_impl!(RangeFull);

impl fmt::Debug for String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("\"")?;
        for byte in &self.bytes {
            write_escaped(*byte, f)?;
        }
        f.write_str("\"")?;
        Ok(())
    }
}

impl fmt::Display for String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(Char::slice_as_str(self.bytes.as_slice()))
    }
}

/// An error which can be returned when parsing an ASCII character.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Byte(usize, u8),
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Byte(i, ch) => write!(f, "invalid byte {:x?} at {}", ch, i),
        }
    }
}
impl std::error::Error for Error {}

impl FromStr for String {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        let mut string = Vec::with_capacity(bytes.len());
        for (i, byte) in bytes.iter().enumerate() {
            let byte = *byte;
            let byte = byte.try_into().map_err(|byte| Error::Byte(i, byte))?;
            string.push(byte);
        }
        Ok(Self { bytes: string })
    }
}
