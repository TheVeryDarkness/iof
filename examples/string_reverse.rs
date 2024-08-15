use iof::read_line;

/// Reads two lines of strings from input, and checks if they are in reverse order.
fn main() {
    let a: String = read_line();
    let b: String = read_line();
    assert_eq!(a, b.chars().rev().collect::<String>());
}
