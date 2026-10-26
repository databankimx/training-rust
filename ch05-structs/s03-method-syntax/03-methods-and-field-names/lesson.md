## Methods and Field Names

A method can have the same name as a field of its struct. Rust treats the two differently, based on whether you write
parentheses after the name. This step shows how that works, and why a method with a field's name is often a *getter*.

---

### A method named after a field

The sample defines a method called `width`. It doesn't return the field. It returns `true` when the field is greater than zero:

```rust,ignore
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}
```

Inside the method, `self.width` is the field, because it's a field access on `self`. The method's name doesn't change that. You can use a field
inside a method of the same name for any purpose.

In `main`, the two uses look different:

```rust,ignore
if rect1.width() {
    println!("The rectangle has a nonzero width; it is {}", rect1.width);
}

println!("The rectangle is {} pixels high", rect1.height);
```

`rect1.width()` has parentheses, so Rust calls the method, which returns `true`. `rect1.width` has none, so Rust reads the field, which is `30`.
Its output is:

```
The rectangle has a nonzero width; it is 30
The rectangle is 50 pixels high
```

---

### Getters

A method that returns a field's value, and does nothing else, is called a *getter*. Rust doesn't create getters automatically, unlike some other
languages, so you write them yourself.

Getters are useful because you can make a field private and its getter public. Other code can read the field through the getter, but can't change
it directly. You'll see how privacy works in Chapter 7.

The method in this sample isn't a plain getter, because it returns a `bool` rather than the field. It shows that a method with a field's name can do
any work at all, but most of the time a method with a field's name is a getter.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
The rectangle has a nonzero width; it is 30
The rectangle is 50 pixels high
```

The comments in `src/main.rs` suggest removing the parentheses after `rect1.width` in the `if`. The compiler then sees a `u32` where it expects a
`bool`, and reports a type mismatch.

---

*For more detail, see section 5.3 "Method Syntax" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch05-03-method-syntax.html).*
