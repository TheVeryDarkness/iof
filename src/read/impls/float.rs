use crate::impl_read_one_from_for_from_str;

#[cfg(feature = "c-compatible")]
mod inner {
    use crate::ext::{Pattern, State};

    /// See <https://doc.rust-lang.org/std/primitive.f64.html#impl-FromStr-for-f64>
    #[derive(Debug, Clone, Copy, Default)]
    pub(super) enum Float {
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
}

// See <https://doc.rust-lang.org/std/primitive.f64.html#impl-FromStr-for-f64>
#[cfg(feature = "c-compatible")]
impl_read_one_from_for_from_str!(
    f32 f64 => inner::Float::Initial
);

#[cfg(not(feature = "c-compatible"))]
impl_read_one_from_for_from_str!(f32 f64);
