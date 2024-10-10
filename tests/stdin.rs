use iof::{read, read_m_n, try_read, try_read_n, unwrap, Mat};

#[test]
fn read_n_0() {
    let s1: Vec<usize> = unwrap!(try_read_n(0));
    let s2: Vec<char> = unwrap!(try_read_n(0));
    assert!(s1.is_empty());
    assert!(s2.is_empty());
}

#[test]
fn read_m_n_0_0() {
    let s1: Mat<usize> = read_m_n(0, 1);
    let s2: Mat<char> = read_m_n(1, 0);
    assert!(s1.is_empty());
    assert_eq!(s2.len(), 1);
    assert!(s2[0].is_empty());
}

#[test]
fn read_array_0() {
    let s1: [usize; 0] = read();
    let s2: [char; 0] = try_read().unwrap();
    assert!(s1.is_empty());
    assert!(s2.is_empty());
}

#[test]
fn read_empty_tuple() {
    let () = read();
    let () = try_read().unwrap();
}
