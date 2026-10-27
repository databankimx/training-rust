fn main() {
    let v1 = vec![1, 2, 3];

    // iter creates an iterator. Nothing happens until the iterator is used.
    let mut v1_iter = v1.iter();

    // next returns the next item wrapped in Some, and None once the items run out.
    // It changes the iterator's position, so the iterator must be mut.
    println!("{:?}", v1_iter.next());
    println!("{:?}", v1_iter.next());
    println!("{:?}", v1_iter.next());
    println!("{:?}", v1_iter.next());

    // A for loop takes ownership of an iterator, so the loop doesn't need mut.
    let v2_iter = v1.iter();
    for value in v2_iter {
        println!("loop: {value}");
    }

    // Try: change iter to into_iter. Does the loop still compile, and what type are the values now?
}
