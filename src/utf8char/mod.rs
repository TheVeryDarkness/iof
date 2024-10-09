pub use extensible::Utf8Char;
pub use fixed::FixedUtf8Char;

mod extensible;
mod fixed;
pub(super) mod iter_fixed;

impl PartialEq<FixedUtf8Char> for Utf8Char {
    fn eq(&self, other: &FixedUtf8Char) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}
impl PartialEq<Utf8Char> for FixedUtf8Char {
    fn eq(&self, other: &Utf8Char) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}
