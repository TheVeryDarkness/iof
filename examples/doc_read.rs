use iof::{read, Mat};

/// Some examples of reading from standard input.
///
/// We use a `v` to indicate the cursor position.
fn main() {
    // Read a single integer from input.
    //
    // v
    // 42 abc def
    let n: u32 = read!();
    assert_eq!(n, 42);

    // Read a single string from input.
    //
    //    v
    // 42 abc def
    let n: String = read!();
    assert_eq!(n, "abc");

    // Read a vector of characters from input.
    // Spaces are ignored, and those characters need not be separated by spaces.
    //
    //        v
    // 42 abc def
    let v: Vec<char> = read!();
    assert_eq!(v, ['d', 'e', 'f']);

    // Read a tuple from input. Equivalent to:
    //
    // ```
    // let l: u32 = read!();
    // let m: f64 = read!();
    // let n: String = read!();
    // ```
    //
    // v
    // 0 0.3 lmn
    let (l, m, n): (u32, f64, String) = read!();
    assert_eq!(l, 0);
    assert_eq!(m, 0.3);
    assert_eq!(n, "lmn");

    // Read a tuple from input, again.
    //
    // v
    // 1+2
    let (a, op, b): (i32, char, u32) = read!();
    assert_eq!(a, 1);
    assert_eq!(op, '+');
    assert_eq!(b, 2);

    // Read a vector of integers from input.
    // They are separated by spaces.
    //
    // v
    // 1 2 3
    let v: Vec<u32> = read!(3);
    assert_eq!(v, [1, 2, 3]);

    // Read a matrix of integers from input.
    // They are separated by spaces, and newlines are unnecessary but useful for readability.
    //
    // v
    // 1 2 3
    // 4 5 6
    let m: Mat<u32> = read!(2, 3);
    assert_eq!(m, [[1, 2, 3], [4, 5, 6]]);

    // Read a matrix of characters from input.
    // Spaces are ignored and unnecessary, too.
    //
    // v
    // .@/#$
    // !@#!@
    // *&@:,
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
