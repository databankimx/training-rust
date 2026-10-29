// impl Fn lets a function return one closure, without naming its type. The closure captures n, so move is needed.
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

// Each closure has its own unique type, so a Vec can only hold them through a trait object, Box<dyn Fn>.
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn returns_initialized_closure(init: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + init)
}

fn main() {
    let add_ten = make_adder(10);
    println!("make_adder: {}", add_ten(5));

    let handlers = vec![returns_closure(), returns_initialized_closure(123)];
    for handler in &handlers {
        println!("handler: {}", handler(5));
    }

    // Try: change the Box in returns_closure to impl Fn(i32) -> i32, and try to put it in the vector. What does the compiler say?
}
