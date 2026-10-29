## Summary: Concurrency with Async

Async gives the same kinds of concurrency as threads, with the runtime choosing what runs next.

* **join:** runs futures together and fairly, and finishes when all of them have. Output order is repeatable. *(See the joining two tasks step.)*
* **Async channels:** `recv` waits without blocking, and yields `None` once the sender is dropped. Ownership of the sender decides when the channel closes. *(See the async channel step.)*
* **No extra threads:** async tasks share the thread that runs `block_on`, so they are cheap to create. *(See both steps.)*

Threads and async solve similar problems. Threads let the operating system run work in parallel. Async lets one thread interleave many waiting tasks. Later sections show how the two can be combined.

---

### Check your understanding

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Say you are given a helper function `wait_all(a, b)` with the following behavior:
* `wait_all` guarantees that `a` and `b` are executed to completion.
* `wait_all` makes no guarantees regarding fairness or initial execution order.

Then given the following code:

```rust
async fn print_letters() {
    let fut1 = async {
        print!("A");
        sleep().await;
        print!("B");
    };
    let fut2 = async {
        print!("C");
        sleep().await;
        print!("D");
    };
    wait_all(fut1, fut2).await;
}
```

Which of the following strings can possibly be printed after running `print_letters().await`?

○ `DABC`
◉ `ABCD`
○ `CADB`
○ `ACBD`
→ Yes. Each future prints its first letter before sleeping and its second letter after. Any interleaving that keeps `A` before `B` and `C` before `D` is allowed, so `ABCD`, `CADB`, and `ACBD` are all possible. `DABC` can't happen, because `D` is printed only after `C`.
````

````quiz
Say you are given a message-passing channel `channel()` which is non-blocking and *bounded*, meaning `send` returns a future that completes once there is capacity in the channel.
Assume you also have a function `join(a, b)` which *fairly* waits on both its arguments to completion. Then given this async code:

```rust
let (tx, mut rx) = channel(16);
let recv_fut = rx.recv();
let send_fut = tx.send(0);
let (n, _) = join(recv_fut, send_fut).await;
println!("{}", n.unwrap());
```

What will happen as a result of executing this code?

○ The program prints `0` and loops forever
○ The program prints nothing and loops forever
○ The program panics
◉ The program prints `0` and exits
→ Yes. The receive future is created before the send, and it's non-blocking, so `join` can poll both. The send completes once there is room in the channel. The receiver then gets `0`, and the program prints it and finishes.
````

---

*The summary is written for this course, and draws on section 17.2 of [The Rust Programming Language](https://doc.rust-lang.org/book/ch17-02-concurrency-with-async.html).*
