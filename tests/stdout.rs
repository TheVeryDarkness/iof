use iof::WriteInto;

#[test]
fn try_write_vec() {
    let vec = vec![1, 2, 3];
    vec.try_write().unwrap();
}

#[test]
fn write_vec() {
    let vec = vec![1];
    vec.write();
}
