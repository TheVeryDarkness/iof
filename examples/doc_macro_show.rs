fn main() {
    use iof::show;

    // This will write "42\n" to the standard output.
    show!(42);
    // This will write "Hello, World!\n" to the standard output.
    show!("Hello, World!");
    // This will write "1 2 3 4\n" to the standard output.
    show!([1, 2, 3, 4]);
    // This will write "1 2\n3 4\n" to the standard output.
    show!([[1, 2], [3, 4]]);
    // This will write "1, 2\n3, 4\n" to the standard output.
    show!([[1, 2], [3, 4]], sep=["\n", ", "]);
    // This will write "1, 2\n3, 4!" to the standard output.
    show!([[1, 2], [3, 4]], sep=["\n", ", "], end="!");
}
