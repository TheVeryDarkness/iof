use iof::read;

/// Some examples of reading from standard input using the `read` function.
fn main() {
    // Read a single integer from input.
    let n: u32 = read();
    assert_eq!(n, 42);

    // Read a string from input.
    let s: String = read();
    assert_eq!(s, "Hello!");

    // Read an array of integers from input.
    let arr: [u32; 4] = read();
    assert_eq!(arr, [1, 2, 3, 4]);

    // Read a nested array of integers from input.
    let arr: [[u32; 2]; 2] = read();
    assert_eq!(arr, [[1, 2], [3, 4]]);
}
