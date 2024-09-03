use iof::{read, Mat};

fn main() {
    // Read a single integer from input.
    let n: u32 = read!();
    assert_eq!(n, 42);

    // Read a single string from input.
    let n: String = read!();
    assert_eq!(n, "abc");

    // Read a vector of characters from input.
    let v: Vec<char> = read!();
    assert_eq!(v, ['d', 'e', 'f']);

    // Read a tuple from input.
    // Equivalent to:
    // let l: u32 = read!();
    // let m: f64 = read!();
    // let n: String = read!();
    let (l, m, n): (u32, f64, String) = read!();
    assert_eq!(l, 0);
    assert_eq!(m, 0.3);
    assert_eq!(n, "lmn");

    // Read a vector of integers from input.
    let v: Vec<u32> = read!(3);
    assert_eq!(v, [1, 2, 3]);

    // Read a matrix of integers from input.
    let m: Mat<u32> = read!(2, 3);
    assert_eq!(m, [[1, 2, 3], [4, 5, 6]]);

    // Read a matrix of characters from input.
    let m: Mat<char> = read!(3, 5);
    assert_eq!(
        m,
        [
            ['.', '@', '/', '#', '$'],
            ['!', '@', '#', '!', '@'],
            ['*', '&', '@', ':', ',']
        ]
    );
}
