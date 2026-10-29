fn main() {
    // An empty vector needs a type, because Rust can't infer it from values that aren't there yet.
    let mut v: Vec<i32> = Vec::new();

    // push adds to the end. The vector must be mut to change it.
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{v:?}");

    // vec! creates a vector with values already in it, and the type is inferred.
    let w = vec![1, 2, 3];
    println!("{w:?}");

    // Try: add `v.pop()` after the first println!, and print what it returns. What is left in v?
}
