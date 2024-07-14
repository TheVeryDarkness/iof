use std::cell::RefCell;
use std::io::StdinLock;

thread_local! {
    pub static STDIN: RefCell<StdinLock<'static>> = RefCell::new(std::io::stdin().lock());
}
