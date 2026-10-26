fn main() {
    let greeting = return_a_string();
    println!("{greeting}");

    // Try: change return_a_string to return a &'static str, such as "Hello world", and print it.
    // Does the caller need to change? Why or why not?
    //
    // Try: change return_a_string to take a &mut String and fill it in. How does main change?
}

fn return_a_string() -> String {
    let s = String::from("Hello world");
    s
}
