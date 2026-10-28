## Joining Two Tasks with join

Chapter 16 used threads for concurrency. Async code can do the same job without starting new operating system threads, and the syntax is close to the threads version. The
difference is that the runtime, not the operating system, decides which future runs next.

The sample has two loops. Each prints a line, then sleeps for half a second, which is where a loop gives control back to the runtime:

```rust,ignore
let fut1 = async {
    for i in 1..10 {
        println!("hi number {i} from the first task!");
        trpl::sleep(Duration::from_millis(500)).await;
    }
};

let fut2 = async {
    for i in 1..5 {
        println!("hi number {i} from the second task!");
        trpl::sleep(Duration::from_millis(500)).await;
    }
};

trpl::join(fut1, fut2).await;
```

`trpl::join` takes two futures, and returns one future. That future finishes when both inputs have finished, and its output is a tuple of their outputs. The sample ignores the tuple,
because each block produces `()`.

`join` is fair. It checks each future in turn, so neither can run far ahead of the other while both are ready. That's why the output is the same on every run:

```
hi number 1 from the first task!
hi number 1 from the second task!
hi number 2 from the first task!
hi number 2 from the second task!
hi number 3 from the first task!
hi number 3 from the second task!
hi number 4 from the first task!
hi number 4 from the second task!
hi number 5 from the first task!
hi number 6 from the first task!
hi number 7 from the first task!
hi number 8 from the first task!
hi number 9 from the first task!
```

The second task ends after 4, so the first task prints alone from 5 onward. Threads can interleave differently on each run, because the operating system chooses which thread runs. Here the
runtime makes that choice in a fixed, fair order.

Notice that no thread is spawned, and no task either. The two async blocks run on the single thread that called `block_on`.

---

### Try it in the runner

Press **Run**. The output should match the list above.

The comments in `src/main.rs` suggest letting the second loop run to 10. The two loops then alternate for the same steps, and the first task keeps going alone after the second one ends.

---

### Quiz

*These questions are adapted from the quizzes in the [Brown University edition of The Rust Programming Language](https://rust-book.cs.brown.edu/).*

````quiz
Consider the following function:

```rust
async fn print_letters() {
    let a = async { print!("A"); };
    let b = async { print!("B"); };
    let c = async { print!("C"); };
    c.await;
    b.await;
    a.await;
}
```

Which of the following strings can possibly be printed after running `print_letters().await`?

○ `ABC`
○ `BCA`
◉ `CBA`
○ `ACB`
→ Yes. Creating a future doesn't run it, and the futures are awaited in the order `c`, `b`, `a`. So the prints happen in that order.
````

---

*For more detail, see section 17.2 "Applying Concurrency with Async" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch17-02-concurrency-with-async.html).*
