use crate::stream::InputStream;
use std::{cell::RefCell, io::StdinLock};

pub(crate) mod read_into;

thread_local! {
    pub(crate) static STDIN: RefCell<InputStream<StdinLock<'static>>> = RefCell::new(InputStream::new(std::io::stdin().lock()));
}
