fn main() {
    println!("Before the panic");

    // panic! stops the program with this message. Nothing after it runs.
    panic!("crash and burn");

    // Try: move the println! below the panic! line. Does it print? Why, or why not?
}
