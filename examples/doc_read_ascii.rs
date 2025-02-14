use iof::{read_m_n, read_n, read_one, ASCIIChar, Mat};

fn main() {
    let c: ASCIIChar = read_one();
    assert_eq!(c, ASCIIChar::Digit1);

    let v: Vec<ASCIIChar> = read_n(2);
    assert_eq!(v, [ASCIIChar::Digit2, ASCIIChar::Digit3]);

    let m: Mat<ASCIIChar> = read_m_n(2, 3);
    assert_eq!(
        m,
        [
            [ASCIIChar::Digit4, ASCIIChar::Digit5, ASCIIChar::Digit6],
            [ASCIIChar::Digit7, ASCIIChar::Digit8, ASCIIChar::Digit9],
        ]
    );
}
