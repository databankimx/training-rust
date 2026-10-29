## Tuple Structs and Unit Structs

Structs can also look like tuples, or have no fields at all. These two forms are useful in different situations, and this step
covers both.

---

### Tuple structs

A *tuple struct* has a name, but its fields don't have names. Only their types are given:

```rust,ignore
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
```

Tuple structs are useful when you want to give a tuple a name, and make it a different type from other tuples. Naming each
field would be verbose or repetitive, so the position is enough.

The two structs above are different types, even though both hold three `i32` values. So a function that takes a `Color` can't
be given a `Point`. The name is part of the type, and that's the point of defining them.

The sample creates both, and reads them two ways. You can use a dot and an index, such as `black.0`, or destructure the struct:

```rust,ignore
let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);

let Point(x, y, z) = origin;
println!("origin is ({x}, {y}, {z})");
println!("black is ({}, {}, {})", black.0, black.1, black.2);
```

Its output is:

```
origin is (0, 0, 0)
black is (0, 0, 0)
```

Unlike a plain tuple, a destructuring pattern for a tuple struct has to name the struct, as `Point(x, y, z)` does.

---

### Unit-like structs

A struct with no fields at all is a *unit-like struct*. It's declared with the keyword, a name, and a semicolon:

```rust,ignore
struct AlwaysEqual;
```

It holds no data, which seems pointless until you want to attach behavior to a type. Traits, which you'll meet in Chapter 10, let you
implement behavior on any type, including a unit-like struct with no data at all. The sample creates one, `_subject`. The underscore
tells Rust it's deliberately unused, so there's no warning.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
origin is (0, 0, 0)
black is (0, 0, 0)
```

The comments in `src/main.rs` suggest passing `black` to a function that takes a `Point`. Rust rejects the call, because the two
types are different, even though their fields match.

---

*For more detail, see section 5.1 "Defining and Instantiating Structs" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch05-01-defining-structs.html).*
