fn main() {
    // Box::new puts the number on the heap and gives back a pointer to it.
    let a = Box::new(15);

    // Moving `a` into `b` copies the pointer. The number on the heap stays where it is.
    let b = a;
    println!("b holds {b}");

    // Try: uncomment the next line and run the program. Rust will refuse to compile it,
    // because `a` was moved into `b`. Read the error. What is Rust protecting you from?
    // println!("a holds {a}");

    // Try: add `let c = Box::new(15);` and print `c` too. How many copies of 15 are on the heap now?

    // Try: change 15 to a String, such as String::from("hi"). Does the move still happen?
    // Does the program still compile when you print `b` afterwards?
}
