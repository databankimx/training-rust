// The Box in Cons gives the enum a known size. Without it, a List would contain a List directly, and the size would be infinite.
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

// Recursion follows the list one Cons at a time, until it reaches Nil.
fn sum(list: &List) -> i32 {
    match list {
        Cons(value, rest) => value + sum(rest),
        Nil => 0,
    }
}

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("sum = {}", sum(&list));

    // Try: replace the innermost Box::new(Nil) with just Nil. What does the compiler say?
}
