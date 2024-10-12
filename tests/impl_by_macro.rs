use iof::{impl_read_one_from_for_from_str, impl_write_into_for_display, show};
use std::{fmt::Display, str::FromStr};

struct Wrapper<T>(T);

impl<T: Display> Display for Wrapper<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T: FromStr> FromStr for Wrapper<T> {
    type Err = <T as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Wrapper(s.parse()?))
    }
}

impl_write_into_for_display!(Wrapper<usize>);
impl_write_into_for_display!(Wrapper<&str>);

impl_read_one_from_for_from_str!(Wrapper<usize>);
impl_read_one_from_for_from_str!(Wrapper<String>);

#[test]
fn show() {
    show!(Wrapper(42));
    show!(Wrapper("Hello, World!"));
}
