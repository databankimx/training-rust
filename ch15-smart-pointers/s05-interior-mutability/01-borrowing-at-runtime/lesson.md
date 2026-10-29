## Borrowing Rules Checked at Runtime

Chapter 4 set out the borrowing rules: at any time you can have either one mutable reference or any number of immutable references, and never both. Normally the
compiler checks these rules, and a violation is a compile error. A `RefCell<T>` moves the same checks to runtime. It lets you borrow mutably through a value that isn't
declared `mut`, and if the rules are broken while the program runs, it panics.

This is called *interior mutability*: the value appears immutable from outside, but its contents can still change. `RefCell<T>` is useful when you're sure the rules are
followed, but the compiler can't prove it.

The sample wraps a number in a `RefCell`:

```rust,ignore
let c = RefCell::new(5);

*c.borrow_mut() += 1;
println!("value = {}", c.borrow());
```

`c` isn't declared `mut`, yet the value can be changed. The method `borrow_mut` returns a smart pointer, `RefMut<T>`, that gives a mutable reference to the value inside. The
`*` follows that reference, and `+= 1` changes the value. `borrow` works the same way for reading, and gives a `Ref<T>`. The output is:

```
value = 6
```

`RefCell` counts the active borrows. Each call to `borrow` or `borrow_mut` adds to the count, and a borrow ends when its smart pointer goes out of scope. While a mutable
borrow is active, another borrow of any kind breaks the rules, and the program panics.

---

### Try it in the runner

Press **Run**. The output should be `value = 6`.

The comments in `src/main.rs` suggest holding two mutable borrows at the same time. Uncomment the two lines, and press **Run** again. The program compiles without error,
which is the difference from a `&mut` reference, and then panics when the second `borrow_mut` runs. The panic message is `RefCell already borrowed`, and it points at the line of
the second borrow. Older versions of Rust print the message as `already borrowed: BorrowMutError`, so the wording may differ between versions. The underscore names keep the warnings
away, and the first borrow still lasts until the end of `main`, so the second one fails.

---

*For more detail, see section 15.5 "RefCell<T> and the Interior Mutability Pattern" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html).*
