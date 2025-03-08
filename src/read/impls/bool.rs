use crate::impl_read_one_from_for_from_str;

#[cfg(feature = "c-compatible")]
mod inner {
    use crate::ext::{Pattern, State};

    #[derive(Debug, Default, Clone, Copy)]
    pub(super) struct Alphabet;

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
}

// See <https://doc.rust-lang.org/std/primitive.bool.html#impl-FromStr-for-bool>
#[cfg(feature = "c-compatible")]
impl_read_one_from_for_from_str!(
    // `bool` is a special case, which only accepts `"true"` and `"false"`.
    bool => inner::Alphabet
);

#[cfg(not(feature = "c-compatible"))]
impl_read_one_from_for_from_str!(bool);
