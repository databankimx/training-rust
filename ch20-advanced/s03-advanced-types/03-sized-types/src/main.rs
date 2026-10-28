use std::fmt::Display;

// ?Sized relaxes the default bound, so T may be a dynamically sized type such as str. The value is taken by reference,
// because a str can only be used behind a pointer.
fn show<T: Display + ?Sized>(value: &T) {
    println!("shown: {value}");
}

fn main() {
    // T is str here. The string slice is a reference to a str, so no size is needed at compile time.
    show("hello");

    // Box<str> also points to a str, and &*boxed gives the reference.
    let boxed: Box<str> = "boxed text".into();
    show(&*boxed);

    // T is i32, which has a known size, so the same function works for it too.
    show(&42);

    // Try: remove the ?Sized part of the bound. Which call stops compiling, and why?
}
