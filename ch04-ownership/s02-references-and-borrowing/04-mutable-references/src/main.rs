fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3];

    // A mutable reference gives temporary, exclusive access to v[2].
    let num: &mut i32 = &mut v[2];
    *num += 1;
    println!("Third element is {}", *num);

    println!("Vector is now {:?}", v);

    // Try: change `&mut v[2]` to `&v[2]` and leave the rest alone. Which line now fails,
    // and why does a shared reference not allow that change?
    //
    // Try: put `let again: &mut i32 = &mut v[0];` before the first println!, and print `again`
    // after it. Why is that rejected, even though `again` points at a different element?
}
