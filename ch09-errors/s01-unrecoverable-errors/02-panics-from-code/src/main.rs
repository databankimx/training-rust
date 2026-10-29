fn main() {
    let v = vec![1, 2, 3];

    // The vector has three elements, at indexes 0, 1, and 2. Index 99 doesn't exist, so indexing panics.
    println!("{}", v[99]);

    // Try: change 99 to 2. What prints, and does the program still panic?
}
