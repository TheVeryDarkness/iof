use iof::{read, Mat};

fn main() {
    let c: usize = read!();
    for _ in 0..c {
        let (m, n): (usize, usize) = read!();
        let _: Mat<i32> = read!(m, n);
    }
}
