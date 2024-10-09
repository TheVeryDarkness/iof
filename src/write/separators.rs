use super::separator::Separator;

/// Separator by.
pub trait Separators: Copy {
    /// Separator type.
    type Separator: Separator;

    /// Residual type.
    type Residual: Separators;

    /// Split the separator on current dimension and other dimensions.
    ///
    /// Please ensure that the separator is not empty.
    fn split(self) -> (Option<Self::Separator>, Self::Residual);
}

impl<T: Separators> Separators for &T {
    type Separator = T::Separator;
    type Residual = T::Residual;

    #[inline]
    fn split(self) -> (Option<Self::Separator>, Self::Residual) {
        (*self).split()
    }
}

impl<'r, 'a, const N: usize> Separators for &'r [&'a str; N] {
    type Separator = &'a str;
    type Residual = &'r [&'a str];

    #[inline]
    fn split(self) -> (Option<Self::Separator>, Self::Residual) {
        Separators::split(self.as_slice())
    }
}

impl<'a> Separators for &[&'a str] {
    type Separator = &'a str;
    type Residual = Self;

    #[inline]
    fn split(self) -> (Option<Self::Separator>, Self) {
        if let Some((first, residual)) = self.split_first() {
            (Some(*first), residual)
        } else {
            (None, self)
        }
    }
}

impl<'a> Separators for &'a str {
    type Separator = &'a str;
    type Residual = Self;

    #[inline]
    fn split(self) -> (Option<Self::Separator>, Self) {
        (Some(self), self)
    }
}

impl<'r, const N: usize> Separators for &'r [char; N] {
    type Separator = char;
    type Residual = &'r [char];

    #[inline]
    fn split(self) -> (Option<Self::Separator>, Self::Residual) {
        Separators::split(self.as_slice())
    }
}

impl<'a> Separators for &[char] {
    type Separator = char;
    type Residual = Self;

    #[inline]
    fn split(self) -> (Option<Self::Separator>, Self) {
        if let Some((first, residual)) = self.split_first() {
            (Some(*first), residual)
        } else {
            (None, self)
        }
    }
}

impl<'a> Separators for char {
    type Separator = char;
    type Residual = Self;

    #[inline]
    fn split(self) -> (Option<Self::Separator>, Self) {
        (Some(self), self)
    }
}

/// Use default separator.
#[derive(Clone, Copy)]
pub struct DefaultSeparator;
impl DefaultSeparator {
    /// Create a new default separator.
    pub const fn new() -> Self {
        Self
    }
}

impl Separators for DefaultSeparator {
    type Separator = &'static str;
    type Residual = Self;

    #[inline]
    fn split(self) -> (Option<Self::Separator>, Self) {
        (None, self)
    }
}
