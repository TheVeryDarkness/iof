use iof::show;

fn main() {
    // Write a single integer to output.
    show!(42);

    // Write a single string to output.
    show!("Hello, World!");

    // Write a vector of integers to output. There will be a space between the integers.
    show!([1, 2, 3]);

    // Write a matrix of integers to output. There will be a newline between the rows, and a space between the integers.
    show!([[1, 2, 3], [4, 5, 6]]);

    // Write a matrix of characters to output. There will be a newline between the rows, and no space between the characters.
    show!([['.', '@', '/'], ['#', '$', '$']]);

    // Write a tuple to output.
    show!((1, 2, 3));

    // Write a tuple of tuples to output.
    show!(((1, 2), (3, 4)));

    // Write an empty tuple to output.
    show!(());

    // Write a tuple of vectors to output.
    show!(([1, 2], [3, 4]));

    // Write a 3-dimensional vector to output with custom separators.
    show!(
        [[[1, 2], [3, 4]], [[5, 6], [7, 8]]],
        sep = [" | ", " :: ", " "],
    );

    // Write several strings to output and append an exclamation mark.
    show!(("Hello", "World"), sep = ", ", end = "!\n");
}
