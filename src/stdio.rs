use crate::stream::InputStream;
use std::{cell::RefCell, io::StdinLock};

thread_local! {
    pub static STDIN: RefCell<InputStream<StdinLock<'static>>> = RefCell::new(InputStream::new(std::io::stdin().lock()));
}
