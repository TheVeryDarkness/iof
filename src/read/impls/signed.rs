use crate::impl_read_one_from_for_from_str;
use std::num::*;

#[cfg(feature = "c-compatible")]
mod inner {
    use crate::ext::{Pattern, State};

    #[derive(Debug, Clone, Copy)]
    pub(super) enum Signed {
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
}

#[cfg(feature = "c-compatible")]
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

    => inner::Signed::Initial
);

#[cfg(not(feature = "c-compatible"))]
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
);
