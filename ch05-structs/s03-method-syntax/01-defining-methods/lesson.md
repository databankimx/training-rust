## Defining Methods

A *method* is like a function, but it's defined inside the context of a struct. It's called with dot syntax, on an
instance of the struct. Methods let you keep everything you can do with a type in one place, and they're the way most
Rust code is organized.

---

### The impl block

Methods are defined in an `impl` block, which is named after the type it belongs to:

```rust,ignore
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

Everything inside the `impl` block belongs to `Rectangle`. Each method's first parameter is `self`, which stands for the instance
the method is called on.

The sample moves the `area` function from the last sub-chapter into an `impl` block:

```rust,ignore
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

Its output is:

```
The area of the rectangle is 1500 square pixels.
```

The call `rect1.area()` uses method syntax: an instance, a dot, the method name, and any arguments in parentheses.

---

### What &self means

In the signature, `&self` is short for `self: &Self`. `Self` is an alias for the type named in the `impl` block, so here it
means `Rectangle`. Rust lets you write the short form, but the `&` is still needed. It says that the method borrows the instance.

A method can take `self` in three ways: by reference (`&self`), by mutable reference (`&mut self`), or by value (`self`). `&self`
is the usual choice when a method only reads the data, as `area` does. It means the caller keeps ownership, so `rect1` can still be
used after the call. The next steps cover the other two forms.

---

### Methods belong to types, not variables

A method has to be defined in an `impl` block for a type, such as `impl Rectangle`. You can't write one for a variable. The book's
quiz shows this mistake: it puts an `impl` block inside `main`, on a variable named `p`, and Rust rejects it.

Here's the program from the quiz:

```rust,ignore
struct Point(i32, i32);
fn main() {
    let p = Point(1, 2);

    impl p {
        fn x(&self) -> i32 { self.0 }
    }

    println!("{}", p.x());
}
```

Rust rejects line 5, where the `impl` block names the variable `p` instead of a type.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
The area of the rectangle is 1500 square pixels.
```

The comments in `src/main.rs` suggest changing `&self` to `self`. The method then takes ownership of the rectangle, and the `println!`
on the next line can't use `rect1` after the call. This is why the method borrows `self` in the first place.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Does this program compile?

```rust
struct Point(i32, i32);
fn main() {
  let p = Point(1, 2);

  impl p {
    fn x(&self) -> i32 { self.0 }
  }

  println!("{}", p.x());
}
```

○ This program does not compile
→ Yes. Methods are defined on types, such as `impl Point`, not on variables such as `p`.
○ This program does compile
→ Not quite. The `impl` block names the variable `p`, which isn't a type, so Rust rejects it.
---
On which line does Rust report the problem?

[ 5 ]
→ Yes. Line 5 is the `impl p` block, which names a variable where a type is needed.
````

---

*For more detail, see section 5.3 "Method Syntax" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch05-03-method-syntax.html).*
