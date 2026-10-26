fn main() {
    let a_num = 4;
    make_and_drop();
    println!("a_num is still {a_num} after the function returns");
}

fn make_and_drop() {
    let a_box = Box::new(5);
    println!("a_box holds {a_box} while make_and_drop runs");

    // Try: return a_box instead of letting it go out of scope, and keep the value in main.
    // When is the heap memory freed now?
    //
    // Try: change the box to Box::new(String::from("hello")). Does the program still behave the same way?
}
