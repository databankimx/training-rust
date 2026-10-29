## Creating and Updating Vectors

A *vector*, `Vec<T>`, stores several values of the same type in one data structure. The values sit next to each other in memory, so a vector
is good for lists such as the lines of a file, or the prices in a shopping cart.

---

### Creating a vector

A new, empty vector is made with `Vec::new`. Because the vector holds no values yet, Rust can't tell what type it will hold, so you write the
type in angle brackets:

```rust,ignore
let mut v: Vec<i32> = Vec::new();
```

When the vector starts with values, Rust infers the type, and the annotation isn't needed. The `vec!` macro makes a vector from a list:

```rust,ignore
let w = vec![1, 2, 3];
```

Here `w` is a `Vec<i32>`, because `i32` is the default integer type.

---

### Adding values

`push` adds a value to the end of the vector. Changing a vector needs `mut`, as with any other variable:

```rust,ignore
v.push(5);
v.push(6);
```

The sample pushes four values and then prints the vector. The output is:

```
[5, 6, 7, 8]
[1, 2, 3]
```

Vectors print with `{:?}`, which shows their contents in square brackets.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
[5, 6, 7, 8]
[1, 2, 3]
```

The comments in `src/main.rs` suggest calling `pop` after the first `println!`. `pop` removes the last value and returns it as an `Option`, so the
vector shrinks by one each time it is called.

---

*For more detail, see section 8.1 "Storing Lists of Values with Vectors" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch08-01-vectors.html).*
