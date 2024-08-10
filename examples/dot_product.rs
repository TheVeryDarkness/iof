use iof::{read, read_n};

/// Reads `n` integers `a`, `b` and `c` from input and checks if the dot product of `a` and `b` is equal to `c`.
fn main() {
    let n: usize = read();
    let a: Vec<i32> = read_n(n);
    let b: Vec<i32> = read_n(n);
    let c: Vec<i32> = read_n(n);
    for i in 0..n {
        assert_eq!(a[i] * b[i], c[i]);
    }
}
