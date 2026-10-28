fn main() {
    let x = 5;

    // y is a reference to x. The * follows the reference to the value it points to.
    let y = &x;
    println!("reference: {}", *y);

    // A box holding a copy of x. The * follows the box's pointer in the same way.
    let b = Box::new(x);
    println!("box: {}", *b);

    // Try: add assert_eq!(5, y); without the *. What does the compiler say, and why?
}
