## Generic Methods and Specific Methods

You can put methods on a generic struct in two ways. A method can work for every type `T`, or an `impl` block can apply only to one concrete type.

---

### Methods for any type

The `impl<T>` line declares `T` as a generic parameter, so the methods that follow apply to a `Point<T>` of any type:

```rust,ignore
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
```

The `T` after `impl` is what tells Rust that `Point<T>` is generic, not a concrete type. Using the same name as the struct's parameter is the convention,
but it doesn't have to match.

---

### Methods for one type

An `impl` block can name a concrete type, without declaring any parameters:

```rust,ignore
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

Only `Point<f32>` gets `distance_from_origin`. A `Point<i32>` doesn't have that method, because the formula uses floating-point operations that integers
don't provide. The sample creates a `Point<f32>`, and calls both methods:

```
x = 3
distance = 5
```

The distance is the square root of 3 squared plus 4 squared, which is 5.

---

### Methods with the same name

A method with the same name can't be defined for `Point<i32>` in one `impl`, and again for `Point<T>` in another, because Rust wouldn't know which one to call.
The quiz below shows this. Rust doesn't have specialization of methods like some object-oriented languages do. The exception is default methods in traits,
which Section 10.2 covers.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
x = 3
distance = 5
```

The comments in `src/main.rs` suggest calling `distance_from_origin` on a `Point<i32>`. The compiler reports that the method doesn't exist for that type.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Does this program compile?

```rust
struct Point<T> { x: T, y: T }

impl Point<i32> {
  fn f(&self) -> &i32 { &self.y }
}

impl<T> Point<T> {
  fn f(&self) -> &T { &self.x }
}

fn main() {
  let p: Point<i32> = Point { x: 1, y: 2 };
  println!("{}", p.f());
}
```

○ This program compiles
→ Not quite. Two methods named `f` apply to `Point<i32>`, so Rust can't tell which one `p.f()` means.
◉ This program does not compile
→ Yes. The two definitions of `f` conflict for `Point<i32>`, and Rust has no rule for choosing between them.
---
On which line does Rust report the problem?

[ 8 ]
→ Yes. Line 8 is the second `fn f`, the one from the generic `impl`, which conflicts with the first.
````

---

*For more detail, see section 10.1 "Generic Data Types" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch10-01-syntax.html).*
