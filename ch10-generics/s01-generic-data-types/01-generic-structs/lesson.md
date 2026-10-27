## Generic Data Types

Generics let one definition work with many concrete types. Instead of writing a struct or a function once for `i32` and again for `f64`, you write it
once with a type parameter, and the compiler fills in the real types where the code is used.

A type parameter is written in angle brackets after the name. By convention, the names are short, and `T` is the usual first choice. You can have as many
as you need, although a long list suggests the code should be split into smaller pieces.

---

### A struct with one type parameter

The `Point<T>` struct holds an `x` and a `y` value, both of type `T`:

```rust,ignore
struct Point<T> {
    x: T,
    y: T,
}
```

Both fields must have the same type, because both use `T`. A `Point` can be `Point<i32>` or `Point<f64>`, but a point with an integer `x` and a float `y`
doesn't compile. The error is a type mismatch, reported on the field that doesn't match.

---

### A struct with two type parameters

If `x` and `y` need different types, give the struct two parameters:

```rust,ignore
struct Point<T, U> {
    x: T,
    y: U,
}
```

The sample uses two parameters, and a method that combines values from two different points. The method `mixup` takes `self` and another point, and returns a
point with `x` from `self` and `y` from the other point:

```rust,ignore
fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
    Point {
        x: self.x,
        y: other.y,
    }
}
```

The method has its own parameters, `V` and `W`, because the other point can have any types. The return type uses `T` from `self` and `W` from the other
point. The sample's output is:

```
p3.x = 5, p3.y = c
```

`p1` has an integer `x` and a float `y`, and `p2` has a string slice `x` and a character `y`. The result takes `x` from `p1`, which is an integer, and `y`
from `p2`, which is a character.

---

### Enums

Enums can be generic too. The standard library's `Option<T>` and `Result<T, E>` are examples from Chapter 6 and Chapter 9. `Result<T, E>` has two parameters,
one for the success type and one for the error type.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
p3.x = 5, p3.y = c
```

The comments in `src/main.rs` suggest changing the struct to take one type parameter, and building `p1` and `p2` again. The compiler reports a type
mismatch, because `x` and `y` must share one type.

---

*For more detail, see section 10.1 "Generic Data Types" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch10-01-syntax.html).*
