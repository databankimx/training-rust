## Solution: A Timed Receiver

This walkthrough explains the example solution in `src/main.rs`. Read it after you've written your own version.

### Run the solution

Press **Run** for the output, and **Test** for the two tests. The output is:

```
got first
got second
got third
nothing arrived within 500 ms
```

### How it works

`within` is generic over any future, so it works with `rx.recv()` without naming the channel type. It races the future against a timer with `trpl::select`. If the future wins, its
output is wrapped in `Some`. If the timer wins, the future is dropped, and `None` is returned.

The channel's receive future resolves to `Option<T>`, so `within` returns a value of type `Option<Option<T>>`. The three arms of the `match` separate three situations: a message
arrived (`Some(Some(word))`), the channel closed (`Some(None)`), and the timer won (`None`). The loop stops in the last two cases.

`tx` is kept until the end of the block, which is what makes the timeout the only way out. If `tx` were dropped before the loop, every `recv` would return `None` straight away, and
the program would print `channel closed` instead of waiting.

The tests use one future that is ready at once, and one that takes five seconds, with a limit of 100 ms. The slow future is dropped when the timer wins, so the test takes about a tenth
of a second, not five seconds.

### The extensions

If `tx` is dropped before the loop, `recv` returns `None` once the three words are read, and the `Some(None)` arm prints `channel closed`. The timeout never runs, because the
channel has already ended.

A growing timeout means passing a different `limit` on each iteration. If the first message takes longer than the first limit, `within` returns `None` straight away, and the loop
stops before reading anything. The first limit needs to cover the slowest expected first message.

---

*This walkthrough accompanies the exercise in Chapter 17 of the course. It's not from the book.*
