use iof::show;
use std::collections::HashSet;

#[test]
fn show() {
    let set: HashSet<i32> = HashSet::from_iter([3, 2, 1]);

    show!(set);
}
