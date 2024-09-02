use iof::read_in_line_trimmed;

/// Reads two lines of strings from input, and checks if they are in reverse order.
fn main() {
    let a: String = read_in_line_trimmed();
    let b: String = read_in_line_trimmed();
    assert_eq!(a, b.chars().rev().collect::<String>());
}
