## Dynamically Sized Types and the Sized Trait

Rust needs to know the size of every value, so it can allocate it. Most types have a size known at compile time. Some don't, and they're called *dynamically sized types*, or DSTs. `str` is one: a string slice
can be any length, so `str` itself has no fixed size, and you can't create a variable of type `str`.

The golden rule is that a dynamically sized value must always sit behind a pointer. A `&str` is a pointer to a `str`, and it's two values, the address and the length, so its size is known. `Box<str>` and
`Rc<str>` also point to a `str`. Trait objects follow the same rule, which is why they're written `&dyn Trait` or `Box<dyn Trait>`.

The `Sized` trait marks the types whose size is known at compile time. Rust adds a `Sized` bound to every generic type parameter by default, so a generic function works only with types that have a known size.
To accept a dynamically sized type, the bound is relaxed with `?Sized`:

```rust,ignore
fn show<T: Display + ?Sized>(value: &T) {
    println!("shown: {value}");
}
```

Two things change in this signature. `?Sized` means `T` may or may not have a known size. The parameter is also taken by reference, `&T`, because a `T` that isn't `Sized` can't be passed by value.

The sample calls `show` with a string slice, a `Box<str>`, and an integer:

```rust,ignore
show("hello");
let boxed: Box<str> = "boxed text".into();
show(&*boxed);
show(&42);
```

The output is:

```
shown: hello
shown: boxed text
shown: 42
```

With `?Sized` removed from the bound, the first call would not compile, because `str` isn't `Sized`. The integer call would still work.

---

### Try it in the runner

Press **Run**. The output should match the three lines above.

The comments in `src/main.rs` suggest removing `?Sized` from the bound. The first call, `show("hello")`, stops compiling, because the compiler needs `T` to have a known size, and `str` doesn't.

---

*For more detail, see section 20.3 "Advanced Types" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch20-03-advanced-types.html).*
