## Homework: Shapes and use

**The problem:** The program below has a module tree of shapes, and it uses three kinds of `use`. Your task is to make it compile and print
the areas, using the `use` forms from this chapter, and to explain each choice.

The starter below works in its current form. Read it, predict the output, and then run it. After that, do the extension tasks.

```rust,ignore
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

pub use shapes::round::Circle;
use shapes::flat::{self, Square};

fn main() {
    let c = Circle { radius: 2.0 };
    let s = Square { side: 3.0 };
    let small = flat::Square { side: 1.0 };

    println!("circle area: {}", c.area());
    println!("square area: {}", s.area());
    println!("small square area: {}", small.area());
}
```

Predict the output before you run it. Then check your prediction.

---

### Hints

* `pub use` re-exports a name, so code that can reach the crate root can use `Circle` directly.
* `use shapes::flat::{self, Square};` brings in two names in one line. `self` stands for `flat` itself.
* Both `Square` and `flat::Square` refer to the same struct. Which form you use is a matter of readability.
* Check the area of each shape by hand. The circle uses 3.14 as pi.

---

### Test your code

Your program should print these three lines:

```
circle area: 12.56
square area: 9
small square area: 1
```

---

### Extensions

* Change `pub use shapes::round::Circle;` to a plain `use`. Does the program still compile? Why?
* Add a third module, `shapes::solid`, with a `Cube` struct. Bring it in with a nested `use` that also brings in `solid` itself.
* Replace `flat::Square` in `main` with `Square`, and remove the `flat` name from the `use` line. What does the compiler say?

---

*This exercise is not from the book. It practices `use`, `pub use`, and nested paths from Chapter 7.*
