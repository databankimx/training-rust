use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn sum(list: &List) -> i32 {
    match list {
        Cons(value, rest) => value + sum(rest),
        Nil => 0,
    }
}

fn main() {
    // a is shared by b and c. Rc::clone adds one more owner, without copying the list.
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));

    println!("sum of a = {}", sum(&a));
    println!("sum of b = {}", sum(&b));
    println!("sum of c = {}", sum(&c));

    // Try: add a list d that starts with 1 and shares a, then print its sum. What is the answer?
}
