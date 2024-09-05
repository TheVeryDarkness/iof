use iof::{read, show};

fn main() {
    // Read an integer.
    let a: usize = read!();
    // Read a vector of `a` integers.
    let b: Vec<usize> = read!(a);
    // Read a matrix of `a` rows and `b.iter().product()` columns.
    let c: Vec<Vec<usize>> = read!(a, b.iter().product());

    let x: Vec<Vec<Vec<Vec<usize>>>> = read!(2, 2, 2, 2);
    assert_eq!(x.len(), 2);
    for (i, x) in x.into_iter().enumerate() {
        assert_eq!(x.len(), 2);
        for (j, x) in x.into_iter().enumerate() {
            assert_eq!(x.len(), 2);
            for (k, x) in x.into_iter().enumerate() {
                assert_eq!(x.len(), 2);
                for (l, x) in x.into_iter().enumerate() {
                    assert_eq!(x, i + j + k + l);
                }
            }
        }
    }

    show!(a);
    show!(b);
    show!(c);

    show!(a, b, c; sep = "\n");
    show!(a; end = "");
    show!(", ", a, " "; sep = "", end = "");
    show!(a, b, c; sep = " :: ");
}
