## Homework: A Timed Receiver

**The problem:** Read messages from an async channel, and stop when no message arrives within a time limit. The sender here stays open, so the channel never closes on its own. The
timeout is what ends the loop.

Create a project in VS Code with `cargo new timed_receiver`, then add the crate with `cargo add trpl@0.3.0`. In `src/main.rs`:

* Write `async fn within<F: Future>(future: F, limit: Duration) -> Option<F::Output>`. It races `future` against `trpl::sleep(limit)`, and returns `Some` with the output if the future
  finishes first, or `None` if the timer does. Use `trpl::select` and `trpl::Either`, as the timeout in Chapter 17.3 did.
* In `main`, inside `trpl::block_on`, create a channel with `trpl::channel()`. Send the words `first`, `second`, and `third`, then keep the sender alive.
* Loop, calling `within(rx.recv(), Duration::from_millis(500))`. Print each message as `got <word>`. When the timer wins, print `nothing arrived within 500 ms`, and stop.

Write tests in a `tests` module. One checks that `within` returns `Some` when the future is quick. Another checks that it returns `None` when the future is slow.

---

### Hints

* `rx.recv()` returns a future that resolves to `Option<T>`. `within` wraps it, so the loop sees `Option<Option<T>>`. The outer `None` means the timer won, and the inner `None` means the
  channel closed.
* `Either::Left(output)` is the first future's result, and `Either::Right(_)` is the timer's.
* Keep `tx` in scope until the end of the block. If it were dropped, `recv` would return `None` straight away, and the timeout would never be tested.

---

### Check your work

Your program should print:

```
got first
got second
got third
nothing arrived within 500 ms
```

---

### Extensions

* Change the loop so that it stops when the channel closes, and prints `channel closed` for that case. What happens if you drop `tx` before the loop?
* Make the timeout grow with each message, starting at 500 ms. What changes if the first message takes longer than the limit?

---

*This exercise is not from the book. It practices channels, `select`, and timeouts from Chapter 17.*
