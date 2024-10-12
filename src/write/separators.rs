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

impl<T: Separator + Copy> Separators for T {
    type Separator = Self;
    type Residual = Self;

    #[inline]
    fn split(self) -> (Option<Self::Separator>, Self::Residual) {
        (Some(self), self)
    }
}

impl<'r, T: Separator, const N: usize> Separators for &'r [T; N] {
    type Separator = &'r T;
    type Residual = &'r [T];

    #[inline]
    fn split(self) -> (Option<Self::Separator>, Self::Residual) {
        Separators::split(self.as_slice())
    }
}

impl<'r, T: Separator> Separators for &'r [T] {
    type Separator = &'r T;
    type Residual = &'r [T];

    #[inline]
    fn split(self) -> (Option<Self::Separator>, Self::Residual) {
        if let Some((first, residual)) = self.split_first() {
            (Some(first), residual)
        } else {
            (None, self)
        }
    }
}

/// Use default separator.
#[derive(Clone, Copy, Default)]
pub struct DefaultSeparator;

impl DefaultSeparator {
    /// Create a new default separator.
    #[inline]
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
