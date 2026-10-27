## Default Implementations

Sometimes a trait has behavior that most types share. Instead of forcing every implementation to repeat it, the trait can give the method a default body.
A type that implements the trait can then keep the default, or override it.

---

### A default method

The sample defines a `MakeNoise` trait, whose `make_noise` method has a body:

```rust,ignore
trait MakeNoise {
    fn make_noise(&self) {
        println!("(silence)");
    }
}
```

`Dog` overrides the method with its own body:

```rust,ignore
impl MakeNoise for Dog {
    fn make_noise(&self) {
        println!("bark");
    }
}
```

`Cat` has an empty `impl` block, so it uses the default:

```rust,ignore
impl MakeNoise for Cat {}
```

Both types can call `make_noise`, and the output shows which version ran:

```
bark
(silence)
```

Overriding a default uses the same syntax as implementing a method that has no default, so `Dog` needed no special form. A default can also call other methods of
the same trait, even methods that have no default. That lets a trait provide a lot of behavior while requiring each type to supply only a small part.

---

### A limit on defaults

An overriding method can't call the default version of itself. Once `Dog` replaces `make_noise`, the default body is no longer reachable from `Dog`'s version.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
bark
(silence)
```

The comments in `src/main.rs` suggest deleting `make_noise` from `Dog`'s impl block. Then `dog.make_noise()` uses the default, and the output becomes
`(silence)` for both animals.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Consider this program, which uses the trait from the sample, with `Dog` overriding `make_noise` and `Cat` using the default:

```rust
dog.make_noise();
cat.make_noise();
```

What does it print?

○ `bark` and then `bark`
→ Not quite. `Cat` doesn't override `make_noise`, so it uses the default, which prints `(silence)`.
◉ `bark`, then `(silence)`
→ Yes. `Dog` uses its own version, which prints `bark`, and `Cat` uses the default, which prints `(silence)`.
○ `(silence)` and then `bark`
→ Not quite. The calls happen in order, so `dog.make_noise()` prints first.
○ Nothing, because the program doesn't compile
→ Not quite. The program compiles, because `Cat` has an empty impl block that is allowed to use the defaults.
````

---

*For more detail, see section 10.2 "Defining Shared Behavior with Traits" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch10-02-traits.html).*
