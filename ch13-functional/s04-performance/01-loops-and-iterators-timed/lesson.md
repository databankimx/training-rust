## Performance: Loops vs. Iterators

Before you choose between a loop and an iterator, it's fair to ask which is faster. The book measures the two versions of `search` on a large text, using a benchmark.
Its results were close: roughly 19.6 and 19.2 milliseconds, within the noise of the measurement.

The point isn't that the two are identical. Iterators are a *zero-cost abstraction*. They make the code read at a high level, and the compiler turns them into roughly the
same machine code you'd write by hand. Optimizations such as unrolling loops, and removing bounds checks on array access, still apply to iterator code.

This sample checks the same thing on your computer. It searches the same generated text with the loop and with the iterator, times each one with `Instant`, and
checks that the two give the same matches:

```rust,ignore
let start = Instant::now();
let loop_results = search_loop(query, &contents);
let loop_time = start.elapsed();
```

The output looks like this, though the times will differ on your machine:

```
loop: 100000 matches in 14.2ms
iterator: 100000 matches in 14.0ms
same results: true
```

The number of matches must be the same, and `same results` must be `true`. The times are the part that changes from one machine to another.

---

### Debug and release builds

The runner's **Run** button builds the program in debug mode. Debug builds skip most optimizations, so the times say more about the build than about the code. A fair
comparison needs an optimized build, which is `cargo run --release` in a terminal, from this step's folder. In a release build, the two times are usually very close.

The runner doesn't pass `--release`, so this comparison is one to make in a terminal.

---

### Try it in the runner

Press **Run**. The three lines above should appear, with the counts and `same results: true`.

The comments in `src/main.rs` suggest running `cargo run --release` in a terminal. Compare the two times with those from the runner. Remember that timings on any single run
can vary, so repeat it a few times before you draw a conclusion.

---

*For more detail, see section 13.4 "Comparing Performance: Loops vs. Iterators" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch13-04-performance.html).*
