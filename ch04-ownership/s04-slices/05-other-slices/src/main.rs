fn main() {
    let a = [1, 2, 3, 4, 5];

    // A slice of an array works the same way as a string slice: a reference, plus a length.
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
    println!("{:?}", slice);

    // Try: change the range to &a[..2] and then to &a[3..]. What does each one print?
}
