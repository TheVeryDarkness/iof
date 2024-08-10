use crate::stream::InputStream;
use std::{cell::RefCell, io::StdinLock};

thread_local! {
    pub static STDIN: RefCell<InputStream<StdinLock<'static>>> = RefCell::new(InputStream::new(std::io::stdin().lock()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_into::ReadInto;

    #[test]
    fn read_remained() {
        let s: String = STDIN.with(|lock| lock.borrow_mut().read_remained_line());
        assert_eq!(s, "");
    }
}
