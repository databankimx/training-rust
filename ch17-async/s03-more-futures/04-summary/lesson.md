## Summary: More Futures

Futures switch only at await points, so code between them must not block for long. Composing futures builds new behavior from simple parts.

* **Starvation:** a future that doesn't await keeps every other future waiting. *(See the starving the runtime step.)*
* **yield_now:** hands control to the runtime without waiting on a timer, so tasks take turns. *(See the yield now step.)*
* **Composition:** `select` and `sleep` make a timeout. `select` polls its arguments in order, so the first future has the first chance to finish. *(See the timeout step.)*

---

### Check your understanding

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Given a function `sleep(d)` that sleeps for the given duration, consider this function:

```rust
let futs: Vec<_> = [1, 2, 3].iter().map(|n| async move {
    sleep(Duration::from_secs(5)).await;
    n + 1
}).collect();

for fut in futs {
    let n = fut.await;
    println!("{n}");
}
```

When executed, about how many seconds should this take to execute?

○ 0
○ 5
◉ 15
→ Yes. The futures are created together, but they are awaited one after another. Each takes five seconds, so the total is 15 seconds. `join_all` would let them run at the same time.
````

````quiz
Say you are designing a utility function with the following specification:

> `map_stringify` takes two arguments: a vector of inputs, and an async function that converts the input to an output, where the outputs can be converted to strings.
> `map_stringify` returns a vector of stringified outputs.

Which function type signature best encodes this specification?

○ `async fn map_stringify(inputs: Vec<String>, f: impl Future<Output = String>) -> Vec<String>`
○ `fn map_stringify<I, O, F>(inputs: Vec<I>, f: impl Fn(I) -> O) -> Vec<impl Future<Output = String>> where O: ToString`
○ `async fn map_stringify<I, F>(inputs: Vec<I>, f: impl Fn(I) -> F) -> Vec<String> where F: Future + ToString`
◉ `async fn map_stringify<I, O, F>(inputs: Vec<I>, f: impl Fn(I) -> F) -> Vec<String> where O: ToString, F: Future<Output = O>`
→ Yes. The function is `async`, so it returns the strings directly. `f` takes an input and returns a future, and that future's output must be something that can be converted to a string.
````

````quiz
Say you are writing a program that needs to run some async code, but every 500ms it checks whether the computation should be halted.
Which helper function would be most appropriate for accomplishing this task?

○ `join`
○ `yield`
○ `spawn` / `spawn_task`
◉ `race` / `select`
→ Yes. Racing the long-running future against a 500 ms timer, in a loop, lets the program check for a halt on each round. `select` finishes as soon as either side does.
````

---

*The summary is written for this course, and draws on section 17.3 of [The Rust Programming Language](https://doc.rust-lang.org/book/ch17-03-more-futures.html).*
