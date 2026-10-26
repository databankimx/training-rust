## Associated Functions

Functions can also be defined inside an `impl` block, even when they don't take `self`. These are called *associated functions*,
because they belong to the type named after the `impl`. They don't need an instance to work with, so you call them with `::` on the type
rather than with a dot on an instance.

You've already used one: `String::from` is an associated function of `String`.

---

### Constructors

The most common associated function is a constructor, which creates and returns a new instance. The sample defines `square`, which makes a
`Rectangle` whose sides are equal:

```rust,ignore
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
```

`Self` is an alias for the type in the `impl` block, so `Self` here means `Rectangle`. Inside the function, `Self { ... }` builds a new instance.

Constructors are often named `new`, but `new` isn't a special word in Rust. It's a convention, and the language doesn't enforce it. The name `square`
is a clearer choice here, because it says what kind of rectangle is created.

The sample calls the constructor with `::`:

```rust,ignore
let sq = Rectangle::square(3);
```

The `::` syntax is used for associated functions, and for names inside modules, which you'll meet in Chapter 7.

---

### Several impl blocks

A type can have more than one `impl` block. The sample puts `square` in one block, and `area` in another. Both belong to `Rectangle`, and the program
behaves exactly as if they were in the same block. There's no need to split them here, but the option is useful later. Chapter 10 shows a case where
several `impl` blocks help.

The sample prints the area of the square, and then the square itself:

```
The area of the square is 9
Rectangle { width: 3, height: 3 }
```

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
The area of the square is 9
Rectangle { width: 3, height: 3 }
```

The comments in `src/main.rs` suggest calling `Rectangle::square(3).area()` directly, without a variable. That compiles, because the constructor
returns a value that the method can be called on.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
What is the keyword for constructor functions in Rust?

○ `constructor`
→ Not quite. Rust has no keyword for constructors.
○ `new`
→ Not quite. `new` is a common name for constructors, but it's a convention, not a keyword.
○ The name of the type being constructed
→ Not quite. Constructors can have any name, and the type's name is not used as a keyword.
◉ None of the above
→ Yes. Rust has no keyword for constructors. An associated function such as `new` is the usual convention, and the language doesn't enforce it.
````

---

*For more detail, see section 5.3 "Method Syntax" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch05-03-method-syntax.html).*
