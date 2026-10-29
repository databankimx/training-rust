## Summary: Advanced Types

The type system has a few features that go beyond everyday use.

* **Newtypes:** a wrapper type keeps values with different meanings apart, such as meters and millimeters. *(See the newtypes and aliases step.)*
* **Type aliases:** `type` gives a name to an existing type, which shortens long types. An alias is not a new type. *(See the newtypes and aliases step.)*
* **The never type:** `!` is the type of an expression that never produces a value, such as `continue`, `panic!`, or a function that never returns. It fits any match arm. *(See the never type step.)*
* **Dynamically sized types:** `str` and trait objects must sit behind a pointer. `?Sized` lets a generic function accept them. *(See the sized types step.)*

---

### Check your understanding

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Does this program compile? If not, on which line does Rust report the problem?

```rust
fn expect_none(x: Option<i32>) -> ! {
    match x {
        Some(n) => panic!("Expected none, found Some({n})"),
        None => (),
    }
}

fn main() {
    println!("{:?}", expect_none(None));
}
```

○ This program compiles, and panics at runtime
○ This program does not compile, on line 3
◉ This program does not compile, on line 4
○ This program compiles, and prints `()`
→ Yes. The return type `!` promises that the function never returns, but the `None` arm returns `()` instead of diverging. The mismatch is on line 4.
````

````quiz
Does this program compile? If not, on which line does Rust report the problem?

```rust
fn is_equal<T: Eq>(t1: &T, t2: &T) -> bool {
    t1 == t2
}

fn main() {
    println!("{}", is_equal("Hello", "world"));
}
```

○ This program compiles, and prints `false`
○ This program does not compile, on line 2
○ This program does not compile, on line 5
◉ This program does not compile, on line 6
→ Yes. The call passes `&str` values, so `T` is `str`. `str` is not `Sized`, and the bound `T: Eq` doesn't allow for that. Adding `?Sized` to the bound would fix it.
````

---

*The summary is written for this course, and draws on the quizzes in section 20.3 of [The Rust Programming Language](https://doc.rust-lang.org/book/ch20-03-advanced-types.html).*
