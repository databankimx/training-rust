fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // Indexing counts from zero, so index 2 is the third element. It panics if the index is out of range.
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    // get returns an Option, so an index past the end gives None instead of a panic.
    match v.get(100) {
        Some(value) => println!("Element 100 is {value}"),
        None => println!("There is no element at index 100"),
    }

    // Try: change &v[2] to &v[100]. What does the panic message say about the length?
}
