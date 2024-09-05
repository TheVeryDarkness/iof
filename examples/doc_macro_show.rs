fn main() {
    use iof::show;

    // This will write "42\n" to the standard output.
    show!(42);
    // This will write "42 Hello, World!\n" to the standard output.
    show!(42, "Hello, World!");
    // This will write "42 Hello, World! 1 2 3 4\n" to the standard output.
    show!(42, "Hello, World!", [1, 2, 3, 4]);
    // This will write "42 Hello, World! 1 2 3 4 1 2 3 4\n" to the standard output.
    show!(42, "Hello, World!", [1, 2, 3, 4], [[1, 2], [3, 4]]);
    // This will write "42, Hello, World!, 1 2 3 4, 1 2 3 4\n" to the standard output.
    show!(42, "Hello, World!", [1, 2, 3, 4], [[1, 2], [3, 4]]; sep=", ");
    // This will write "42, Hello, World!, 1 2 3 4, 1 2 3 4!" to the standard output.
    show!(42, "Hello, World!", [1, 2, 3, 4], [[1, 2], [3, 4]]; sep=", ", end="!");
}
