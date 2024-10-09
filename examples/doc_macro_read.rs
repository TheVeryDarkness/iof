use iof::read;

fn main() {
    let mut i = 0usize;
    let mut f = || {
        i += 1;
        i
    };
    let m: Vec<Vec<u32>> = read!(3, f());
    assert_eq!(m, vec![vec![1], vec![2, 3], vec![4, 5, 6]]);
}
