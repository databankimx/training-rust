fn main() {
    // Some and None are in the prelude, so they can be used without the Option:: prefix.
    let some_number = Some(5);
    let some_char = Some('e');

    // None has no value, so the type has to be written out, to say what Some would hold.
    let absent_number: Option<i32> = None;

    println!("{:?} {:?} {:?}", some_number, some_char, absent_number);

    // Try: change Some(5) to Some(5.0). What is the type of some_number now?
}
