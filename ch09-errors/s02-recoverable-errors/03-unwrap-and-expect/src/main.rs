fn main() {
    // parse returns a Result. expect returns the number when it's Ok, and panics with our message when it's Err.
    let number: i32 = "42".parse().expect("the text should be a number");
    println!("The number is {number}");

    // Try: change "42" to "forty-two". What does the panic message say?
}
