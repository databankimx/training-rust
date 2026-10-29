use std::cell::RefCell;
use std::rc::Rc;

enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn sum(list: &List) -> i32 {
    match list {
        Cons(value, rest) => *value.borrow() + sum(rest),
        Nil => 0,
    }
}

fn main() {
    // value is shared by a, and a is shared by b and c. Changing value changes every list that reaches it.
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(
        Rc::clone(&value),
        Rc::new(Cons(Rc::new(RefCell::new(10)), Rc::new(Nil))),
    ));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    println!("before: a = {}, b = {}, c = {}", sum(&a), sum(&b), sum(&c));

    *value.borrow_mut() += 10;

    println!("after: a = {}, b = {}, c = {}", sum(&a), sum(&b), sum(&c));

    // Try: add 100 to value instead of 10. Work out the three new sums before you run the program.
}
