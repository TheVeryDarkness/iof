use iof::{dimension::Dimension, show, unwrap, Separators, WriteInto};
use std::{io, str::from_utf8};

/// A compact and sorted container.
struct CustomContainer<U> {
    data: Vec<U>,
}

impl<U: Ord> FromIterator<U> for CustomContainer<U> {
    fn from_iter<T: IntoIterator<Item = U>>(iter: T) -> Self {
        let mut data: Vec<U> = iter.into_iter().collect();
        data.sort();
        Self { data }
    }
}

impl<U> Dimension for CustomContainer<U> {
    const DIMENSION: usize = 1;
    const SPACE: bool = false;
}

impl<U: WriteInto> WriteInto for CustomContainer<U> {
    fn try_write_into_with_sep<S: io::Write + ?Sized>(
        &self,
        s: &mut S,
        _sep: impl Separators,
    ) -> io::Result<()> {
        self.data
            .try_write_into_with_sep(s, &[Self::get_default_separator()])
    }
}

#[test]
fn show() {
    let data = CustomContainer::from_iter([9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    let mut buf = Vec::new();
    show!(data => buf);
    assert_eq!(unwrap!(from_utf8(&buf)), "0123456789\n");
}
