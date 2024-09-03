use iof::{get_line_some, read};

fn main() {
    // Read a single string from input.
    let s: String = read!();
    assert_eq!(s, "42");

    // Read a non-empty line of string from input.
    let s: String = get_line_some();
    assert_eq!(s, "abc");
}
