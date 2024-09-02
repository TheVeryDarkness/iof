use iof::{read, read_m_n, Mat};

fn main() {
    let c: usize = read();
    for _ in 0..c {
        let (m, n): (usize, usize) = read();
        let _: Mat<i32> = read_m_n(m, n);
    }
}
