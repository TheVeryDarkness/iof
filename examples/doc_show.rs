use iof::show;

fn main() {
    // Write a single integer to output.
    show!(42);

    // Write a single string to output.
    show!("Hello, World!");

    // Write a vector of integers to output.
    show!([1, 2, 3]);

    // Write a matrix of integers to output.
    show!([[1, 2, 3], [4, 5, 6]]);

    // Write a matrix of characters to output.
    show!([['.', '@', '/'], ['#', '$', '$']]);

    // Write a tuple to output.
    show!((1, 2, 3));

    // Write a tuple of tuples to output.
    show!(((1, 2), (3, 4)));

    // Write an empty tuple to output.
    show!(());
}
