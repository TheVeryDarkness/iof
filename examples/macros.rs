use iof::{read, show};

fn main() {
    // Read an integer.
    let a: usize = read!();
    // Read a vector of `a` integers.
    let b: Vec<usize> = read!(a);
    // Read a matrix of `a` rows and `b.iter().product()` columns.
    let c: Vec<Vec<usize>> = read!(a, b.iter().product());

    show!(a);
    show!(b);
    show!(c);

    show!(a, b, c; sep = "\n");
    show!(a; end = "");
    show!(", ", a, " "; sep = "", end = "");
    show!(a, b, c; sep = " :: ");
}
