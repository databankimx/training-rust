## Sharing Data with Rc<T>

Most of the time, a value has one owner. Sometimes several parts of a program need to own the same data, and no single one can be known to finish last. A graph is an
example: several edges can point to one node, and the node should exist until the last of them is gone.

`Rc<T>` allows shared ownership. The name stands for *reference counting*. Each `Rc` keeps a count of how many owners the value has, and the value is freed when the count
reaches zero. `Rc<T>` is only for single-threaded code. Chapter 16 covers sharing across threads.

The sample rebuilds the cons list with `Rc<List>` in place of `Box<List>`. Three lists share one tail:

```rust,ignore
let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
let b = Cons(3, Rc::clone(&a));
let c = Cons(4, Rc::clone(&a));
```

The list `a` holds 5 and 10. The lists `b` and `c` each hold one new value, and then point to `a`. With a `Box`, the first use of `a` would move it into `b`, and `c`
couldn't use it. With `Rc`, `Rc::clone` makes a second owner that points to the same data. Only the count changes, so the call is cheap.

The program prints:

```
sum of a = 15
sum of b = 18
sum of c = 19
```

The sums are 5 + 10, 3 + 15, and 4 + 15. All three lists read the same `a`.

Write `Rc::clone(&a)`, rather than `a.clone()`. The two do the same thing here, but the full form makes it clear the call only increases the count. A deep copy of the list would
cost far more time, so a reader of the code can tell at a glance that no copy happens.

---

### Try it in the runner

Press **Run**. The output should match the three lines above.

The comments in `src/main.rs` suggest adding a fourth list, which starts with 1 and shares `a`. Work out its sum before you run the code.

---

*For more detail, see section 15.4 "Rc<T>, the Reference-Counted Smart Pointer" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch15-04-rc.html).*
