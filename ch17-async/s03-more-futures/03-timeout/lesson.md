## Building a Timeout from Two Futures

Futures can be combined to make new behavior. A timeout is a useful example: it runs a future, and gives up if the future takes too long. The `trpl` crate already provides the two pieces
needed, `select` and `sleep`, so the timeout is a short async function.

```rust,ignore
async fn timeout<F: Future>(
    future_to_try: F,
    max_time: Duration,
) -> Result<F::Output, Duration> {
    match trpl::select(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
}
```

The function races the caller's future against a timer. If the future finishes first, the result is `Left`, and the function returns `Ok` with the future's output. If the timer finishes first, the
result is `Right`, and the function returns `Err` with the time that was allowed.

The order of the arguments matters. `select` polls its arguments in the order they are passed, so `future_to_try` gets the first chance to finish, even when `max_time` is very short. The book
points out that this `select` isn't fair, unlike some other implementations, which choose the order at random.

The sample tries to run a future that takes five seconds, with a limit of two:

```rust,ignore
let slow = async {
    trpl::sleep(Duration::from_secs(5)).await;
    "Finally finished"
};

match timeout(slow, Duration::from_secs(2)).await {
    Ok(message) => println!("Succeeded with '{message}'"),
    Err(duration) => println!("Failed after {} seconds", duration.as_secs()),
}
```

The output is:

```
Failed after 2 seconds
```

When the timer wins, the slow future is dropped, so its remaining work never runs. The program doesn't wait for the full five seconds.

Because a future composes with other futures, a timeout can be combined with retries, and then used with network calls. The book's larger examples are built the same way.

---

### Try it in the runner

Press **Run**. The output should be `Failed after 2 seconds`, after about two seconds.

The comments in `src/main.rs` suggest raising the limit to 10 seconds. The slow future then finishes first, and the output is `Succeeded with 'Finally finished'`. Run it, and check that the
program takes about five seconds this time.

---

*For more detail, see section 17.3 "More Futures" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch17-03-more-futures.html).*
