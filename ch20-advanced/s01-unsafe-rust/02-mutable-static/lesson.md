## Mutable Static Variables

Global variables in Rust are called *static* variables. An immutable static is safe to read, and it has a single fixed address in memory. A mutable static is not safe, because if two threads changed it at the same time, the
result would be a data race. Rust therefore requires `unsafe` for every access to a mutable static, both reading and writing.

This step uses the book's counter, `COUNTER`:

```rust,ignore
static mut COUNTER: u32 = 0;

// SAFETY: Calling this from more than one thread at a time is a data race, which is undefined behavior.
unsafe fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
```

`add_to_count` is marked `unsafe fn`, because it has a requirement that the compiler can't check: only one thread may use the counter. The comment above it, starting with `SAFETY`, states that
requirement. The body still needs its own `unsafe` block, because Rust asks for one even inside an unsafe function. That keeps the unsafe part as small as possible.

The read in `main` uses `&raw const COUNTER`. Recent versions of Rust deny by default any attempt to create a reference to a mutable static, and that includes a reference created by `println!`. Reading
through a raw pointer avoids the lint, and makes the access visible. The output is:

```
COUNTER: 3
```

This program is single-threaded, so the access is safe, and the `SAFETY` comment in `main` records why.

Where possible, prefer the thread-safe types from Chapter 16, such as `Mutex<T>` and `AtomicU32`. The compiler then checks the access, and no `unsafe` is needed.

---

### Try it in the runner

Press **Run**. The output should be `COUNTER: 3`.

The comments in `src/main.rs` suggest calling `add_to_count(3)` a second time before printing. The counter then holds 6, and the output is `COUNTER: 6`.

---

*For more detail, see section 20.1 "Unsafe Rust" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html).*
