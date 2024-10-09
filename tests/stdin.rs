use iof::{stdin, Mat, ReadInto};

#[test]
fn read_n_0() {
    let s1: Vec<usize> = stdin().read();
    let s2: Vec<char> = stdin().try_read_n(0).unwrap();
    assert!(s1.is_empty());
    assert!(s2.is_empty());
}

#[test]
fn read_m_n_0_0() {
    let s1: Mat<usize> = stdin().read_m_n(0, 1);
    let s2: Mat<char> = stdin().try_read_m_n(1, 0).unwrap();
    assert!(s1.is_empty());
    assert_eq!(s2.len(), 1);
    assert!(s2[0].is_empty());
}

#[test]
fn read_array_0() {
    let s1: [usize; 0] = stdin().read();
    let s2: [char; 0] = stdin().try_read().unwrap();
    assert!(s1.is_empty());
    assert!(s2.is_empty());
}

#[test]
fn read_empty_tuple() {
    let () = stdin().read();
    let () = stdin().try_read().unwrap();
}
