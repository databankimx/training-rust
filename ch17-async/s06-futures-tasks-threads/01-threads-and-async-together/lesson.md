## Putting It All Together: Futures, Tasks, and Threads

Chapter 16 used threads for concurrency, and this chapter used async. In many programs the right answer is both, each doing the part it suits.

The two models trade off differently:

- A thread is managed by the operating system. Each one takes memory, and only systems with threads can use them. A thread runs to completion, and the operating system decides when to switch.
- A task is managed by the runtime, and many tasks can share one thread. Tasks switch at await points, and they're cheap to create. Some runtimes move tasks between threads, which is called work stealing.

Two rules of thumb follow from this. If the work is CPU-bound and can be split into parts, use threads. If the work is I/O-bound, with messages that arrive at unpredictable times, use async.

This step combines the two. A thread does blocking work, sending a number every second. The main thread uses an async channel to wait for each number:

```rust,ignore
thread::spawn(move || {
    for i in 1..11 {
        tx.send(i).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

trpl::block_on(async {
    while let Some(message) = rx.recv().await {
        println!("{message}");
    }
});
```

The thread can sleep, because the sleep blocks only that thread. The async side can wait on the channel without holding up anything else. The output is the numbers 1 to 10, one per second:

```
1
2
3
4
5
6
7
8
9
10
```

The program takes about ten seconds. A real application might use the same arrangement for video encoding: the encoding runs on a thread, because it's CPU-bound, and an async
channel tells the user interface when each job is done.

---

### Try it in the runner

Press **Run**. The numbers 1 to 10 should appear one per second.

The comments in `src/main.rs` suggest changing the range so it ends at 5. The program then finishes in about four seconds, and prints 1 to 4.

---

*For more detail, see section 17.6 "Putting It All Together: Futures, Tasks, and Threads" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch17-06-futures-tasks-threads.html).*
