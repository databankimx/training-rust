use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

// Every Pair has new.
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// cmp_display exists only when T can be compared with >= and printed with {}.
impl<T: PartialOrd + Display> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    Pair::new(3, 7).cmp_display();

    // Integers implement Display, so they get to_string from the standard library's blanket implementation.
    let s = 3.to_string();
    println!("{s}");

    // Try: call cmp_display on a Pair of a struct that doesn't implement PartialOrd. What does the compiler say?
}
