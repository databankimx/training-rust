## Summary: Traits for Async

Async runs on a few traits and types that the compiler uses for you.

* **Future and poll:** `await` polls a future until it's ready. A pending future hands control back to the runtime. *(See the poll method step.)*
* **Pin:** a pinned value can't move, which is needed for futures that hold references to themselves. `Box::pin` and `pin!` create pinned values. *(See the pinning a collection step.)*
* **Unpin:** most types can move freely, so they're `Unpin`, and pinning is optional for them. Futures from async blocks usually aren't. *(See the pinning a collection step.)*
* **Stream:** `poll_next` is the low-level form of `StreamExt::next`. Most code uses `await` with `StreamExt`, and never calls `poll_next` directly. *(See the poll method step.)*

You rarely need these details in day-to-day code. They matter when an error message mentions `Pin` or `Unpin`, or when you write a runtime or a custom future.

---

### Check your understanding

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Given a future `f`, which of the following best describes how `f.await` works?

○ It calls `f.poll()` once, which blocks in-place until `f` is ready
○ It calls `f.poll()` in a loop, exiting if `f` is ready, or immediately polling again if `f` is not ready
○ It calls `f.push(...)` once, which executes a callback containing the remaining code after `f.await` once `f` is ready
◉ It calls `f.poll()` in a loop, exiting if `f` is ready, or returning control to the async runtime if `f` is not ready
→ Yes. Each poll either gives the value, or gives control to the runtime, which polls again later. The loop doesn't block the thread, which is why await is cheap.
````

````quiz
Imagine Rust did not require that futures were pinned in order to be polled. Which of the following async functions could potentially cause undefined behavior if not pinned?

○ `async fn example(x: &i32) -> i32 { sleep(Duration::from_secs(1)).await; *y }`
○ `async fn example(x: Vec<i32>) -> i32 { sleep(Duration::from_secs(1)).await; x[0] }`
○ `async fn example() -> i32 { let x = 0; sleep(Duration::from_secs(1)).await; x }`
◉ `async fn example(x: i32) -> i32 { let y = &x; sleep(Duration::from_secs(1)).await; *y }`
→ Yes. Here `y` is a reference to `x`, and both live inside the future. If the future moved, `y` would point at a location that is no longer valid. That's a self-reference, and pinning prevents it.
````

---

*The summary is written for this course, and draws on section 17.5 of [The Rust Programming Language](https://doc.rust-lang.org/book/ch17-05-traits-for-async.html).*
