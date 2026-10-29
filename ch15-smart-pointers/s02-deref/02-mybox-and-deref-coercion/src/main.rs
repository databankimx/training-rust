use std::ops::Deref;

// A tuple struct with one element, like Box<T>. Unlike Box<T>, it doesn't store its data on the heap.
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Deref lets * work on a MyBox, and lets a &MyBox be used where a reference to the inner type is expected.
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, *y);
    println!("*y = {}", *y);

    // hello takes a &str. Deref coercion turns &MyBox<String> into &String, and then into &str.
    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // Try: remove the impl Deref block. Which line stops compiling, and what does the error say?
}
