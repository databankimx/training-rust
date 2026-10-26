## Method Calls Are Function Calls

Method syntax can look like something special, but it's a shorthand for a function call. Once you see the function behind the method,
the rules about borrowing and `&` make sense, and you can read code that uses either form.

---

### The same operation, two ways

The sample calls `area` both ways:

```rust,ignore
let area1 = r.area();
let area2 = Rectangle::area(&r);
```

The first is method syntax. The second calls the associated function `Rectangle::area` directly, and passes it `&r`. Both give the same result. The
sample prints them side by side, and the output shows they match:

```
2 2
```

The same is true for a method that changes the instance. Here `set_width` takes `&mut self`, so the function form passes `&mut r`:

```rust,ignore
r.set_width(2);
Rectangle::set_width(&mut r, 2);
```

After the call, `r.area()` gives `4`, because the width is now 2 and the height is 2.

---

### What the dot does for you

When you write `r.area()`, Rust turns it into `Rectangle::area(&r)`. The method's first parameter is `&self`, so Rust inserts the `&` for you. For a
method that takes `&mut self`, Rust inserts `&mut`. For a method that takes `self` by value, it passes the value.

This is why C and C++ programmers, who use both `.` and `->`, find Rust simpler. Rust has no arrow operator. The dot works for both values and
references, and Rust adds or removes references and dereferences as needed to make the types match.

The types can also change through several layers of references. The book's example calls `area` on a mutable reference to a boxed rectangle. Rust
dereferences the reference and the box, and then borrows the rectangle as `&Rectangle`, which the method expects. Chapter 4.2 covers the
dereferencing rules.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
2 2
4
```

The comments in `src/main.rs` suggest passing `r` instead of `&r` to `Rectangle::area`. Rust rejects that, because the function expects a reference and
`r` is a `Rectangle`. The method form doesn't have this problem, because Rust inserts the reference for you.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Say you have a variable `v` of type `&mut Vec<i32>`, and you want to call the `len` method with the following signature:

```
impl Vec<i32> {
  fn len(&self) -> usize {
    /* ... */
  }
}
```

If you try to compile the expression `v.len()`, which of the following statements best describes what happens?

○ It does not compile, because `&mut Vec<i32>` is not the same type as `&Vec<i32>`
→ Not quite. The `&mut` reference is converted to a shared reference, so the types do match.
○ It does not compile, `v` is not explicitly dereferenced
→ Not quite. The dot syntax dereferences `v` for you, so no explicit dereference is needed.
○ It compiles, because `&self` can take any kind of reference
→ Not quite. `&self` takes a shared reference specifically. The compiler converts the `&mut` for you, but it's the conversion, not the parameter, that makes this work.
◉ It compiles, because the `&mut` reference is implicitly reborrowed as an `&` reference
→ Yes. The expression `v.len()` becomes `Vec::len(&*v)`, which reborrows `v` as a shared reference, so the call compiles.
````

---

*For more detail, see section 5.3 "Method Syntax" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch05-03-method-syntax.html).*
