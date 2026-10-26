fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3];

    // num is a reference to the third element. It is only used before the push.
    let num: &i32 = &v[2];
    println!("Third element is {}", *num);

    v.push(4);
    println!("Vector is now {:?}", v);

    // Try: move the println that uses num to after v.push(4), and run the program.
    // Rust rejects it, because the push could invalidate num. Read the error. Which two
    // operations on v does the error point at?
    //
    // Try: print v.capacity() before and after the push. What changes?
}
