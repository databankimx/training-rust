## Yielding with yield_now

`trpl::yield_now` gives control back to the runtime, without waiting for anything. It's the cooperative version of a pause: a future that calls it says "other work can run now,
and I'll continue when you come back to me."

This sample is the starvation example from the last step, with `yield_now` placed after each slow step:

```rust,ignore
let a = async {
    println!("'a' started.");
    slow("a", 30);
    trpl::yield_now().await;
    slow("a", 10);
    trpl::yield_now().await;
    slow("a", 20);
    trpl::yield_now().await;
    println!("'a' finished.");
};
```

`select` polls `a` first, then `b`. Each time a future yields, the other one gets its turn, so the two take turns:

```
'a' started.
'a' ran for 30ms
'b' started.
'b' ran for 75ms
'a' ran for 10ms
'b' ran for 10ms
'a' ran for 20ms
'b' ran for 15ms
'a' finished.
```

`b` never reaches its last steps, because `select` returns as soon as `a` has finished. The output stops at `a finished.`.

`yield_now` is different from `trpl::sleep`. A sleep waits for a timer, which has a minimum resolution, so even a one-nanosecond sleep takes about a millisecond. A yield returns control at once, and
the runtime resumes the future as soon as it's its turn again.

This is cooperative multitasking. Each future decides when to hand over control, and every future is responsible for not blocking for too long. Yielding is cheap, but it isn't free, and
breaking up work that doesn't need to be split can make a program slower. Measure before you add yields.

---

### Try it in the runner

Press **Run**. The output should match the list above, with `'a' finished.` last.

The comments in `src/main.rs` suggest adding a short step to `a`. Work out where it appears before you run the program. Remember that `select` stops when `a` finishes, so anything `b` does
after that point doesn't print.

---

*For more detail, see section 17.3 "More Futures" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch17-03-more-futures.html).*
