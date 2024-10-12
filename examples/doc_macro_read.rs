use iof::read;

fn main() {
    let mut i = 0usize;
    let mut f = || {
        i += 1;
        i
    };
    let m: Vec<Vec<u32>> = read!(3, f());
    assert_eq!(m, vec![vec![1], vec![2, 3], vec![4, 5, 6]]);

    let m: Vec<Vec<Vec<i32>>> = read!(3, 3, 3);
    assert_eq!(
        m,
        [
            [[1, 2, 3], [4, 5, 6], [7, 8, 9],],
            [[10, 11, 12], [13, 14, 15], [16, 17, 18],],
            [[19, 20, 21], [22, 23, 24], [25, 26, 27],],
        ],
    );
}
