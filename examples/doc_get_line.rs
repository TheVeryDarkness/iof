use iof::{get_line, read};

fn main() {
    // Read a single string from input.
    let s: String = read!();
    assert_eq!(s, "42");

    // Read a line of string from input.
    let s: String = get_line();
    assert_eq!(s, "");

    // Read a line of string from input.
    let s: String = get_line();
    assert_eq!(s, "abc");
}
