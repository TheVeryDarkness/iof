#[macro_export]
/// Read a single data item, a [Vec] or a [Mat] from input.
///
/// - `read!()` reads a single data item from input.
/// - `read!(n)` reads `n` data items from input and stores them in a [Vec].
/// - `read!(m, n)` reads `m * n` data items from input and stores them in a [Mat].
///
/// [Mat]: crate::Mat
///
/// # Example
macro_rules! read {
    () => {
        $crate::read()
    };
    ($n:expr) => {
        $crate::read_n($n)
    };
    ($m:expr, $n:expr) => {
        $crate::read_m_n($m, $n)
    };
}
