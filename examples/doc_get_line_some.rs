use iof::{get_line_some, read};

fn main() {
    // Read a single string from input.
    //
    // v
    // 42
    // abc
    let s: String = read!();
    assert_eq!(s, "42");

    // Read a non-empty line of string from input.
    // Before reading, the cursor is at the end of the previous line.
    // However, as the function name implies, it will repeatedly read lines until a non-empty line is found.
    //
    //   v
    // 42
    // abc
    let s: String = get_line_some();
    assert_eq!(s, "abc");
}
