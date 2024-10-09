use iof::{unwrap, WriteInto};

#[test]
fn try_write_vec() {
    let vec = vec![1, 2, 3];
    unwrap!(vec.try_write());
    let vec = vec![1];
    unwrap!(vec.try_write());
}
