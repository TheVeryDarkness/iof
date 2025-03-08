/// Some examples of reading from standard input using the C-compatible feature.
///
/// We use a `v` to indicate the cursor position.
#[cfg(feature = "c-compatible")]
fn main() {
    use iof::read;

    // Read integers from input, that are not separated by spaces.
    //
    // v
    // 1+2
    let (a, op, b): (i32, char, u32) = read!();
    assert_eq!(a, 1);
    assert_eq!(op, '+');
    assert_eq!(b, 2);

    // Or float numbers.
    //
    // v
    // 1.2-3.4
    let (a, op, b): (f64, char, f64) = read!();
    assert_eq!(a, 1.2);
    assert_eq!(op, '-');
    assert_eq!(b, 3.4);
}

/// Without the C-compatible feature, this function will not be compiled.
#[cfg(not(feature = "c-compatible"))]
fn main() {
    use iof::try_read;

    // Read integers from input, that are not separated by spaces.
    // As `1+2` could be interpreted as a single integer, this will fail.
    //
    // v
    // 1+2
    let a: Result<i32, _> = try_read();
    assert!(a.is_err());

    // Or float numbers.
    // Similarly, `1.2-3.4` could be interpreted as a single float number.
    //
    // v
    // 1.2-3.4
    let a: Result<f64, _> = try_read();
    assert!(a.is_err());
}
