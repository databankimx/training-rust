fn main() {
    // Option<T> is used with two types. The compiler makes a separate version for each, at compile time.
    let integer = Some(5);
    let float = Some(5.0);

    println!("{:?} {:?}", integer, float);

    // Try: add a third use, such as Some('c'). How many versions of Option does the program now need?
}
