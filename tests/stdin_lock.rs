use iof::{stdin, ReadInto};
use ntest::timeout;

#[test]
#[cfg_attr(miri, ignored)]
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
