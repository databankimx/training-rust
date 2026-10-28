use std::cell::RefCell;

fn main() {
    // The RefCell is not mut, but its value can still be changed through borrow_mut.
    // The borrowing rules are checked when the program runs, not when it compiles.
    let c = RefCell::new(5);

    *c.borrow_mut() += 1;
    println!("value = {}", c.borrow());

    // Try: uncomment these two lines. The underscores stop the unused-variable warnings, and the program panics on the second line.
    // let _first = c.borrow_mut();
    // let _second = c.borrow_mut();
}
