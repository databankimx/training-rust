mod shapes {
    pub mod round {
        pub struct Circle {
            pub radius: f64,
        }

        impl Circle {
            pub fn area(&self) -> f64 {
                3.14 * self.radius * self.radius
            }
        }
    }

    pub mod flat {
        pub struct Square {
            pub side: f64,
        }

        impl Square {
            pub fn area(&self) -> f64 {
                self.side * self.side
            }
        }
    }
}

// Re-exported, so Circle is available at the crate root.
pub use shapes::round::Circle;

// A nested use brings flat itself, and Square, into scope in one line.
use shapes::flat::{self, Square};

fn main() {
    let c = Circle { radius: 2.0 };
    let s = Square { side: 3.0 };
    let small = flat::Square { side: 1.0 };

    println!("circle area: {}", c.area());
    println!("square area: {}", s.area());
    println!("small square area: {}", small.area());
}
