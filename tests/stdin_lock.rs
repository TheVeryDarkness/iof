use iof::{get_line, get_line_some, stdin, ReadInto};
use ntest::timeout;

#[test]
#[cfg_attr(miri, ignore)]
#[timeout(100)]
#[should_panic]
fn reentry_stdin() {
    let mut a = stdin();
    let mut b = stdin();
    let a = &mut *a;
    let b = &mut *b;
    let _: usize = a.read();
    let _: usize = b.read();
}

#[test]
#[cfg_attr(miri, ignore)]
#[timeout(100)]
#[should_panic]
fn stdin_get_line() {
    let _ = get_line();
}

#[test]
#[cfg_attr(miri, ignore)]
#[timeout(100)]
#[should_panic]
fn stdin_get_line_some() {
    let _ = get_line_some();
}
