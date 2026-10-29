fn main() {
    // The 5 is stored on the heap. The box itself, which holds the pointer, is on the stack.
    let b = Box::new(5);
    println!("b = {}", b);

    // Try: change b to *b in the println! above. Does the output change?
}
