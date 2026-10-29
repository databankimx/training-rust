## Spawning Threads and Waiting with join

A thread runs part of a program at the same time as the rest of it. The standard library's `thread::spawn` starts a new thread, and takes a closure with the code it should
run. Rust uses one operating system thread for each thread you create, which is called a 1:1 model.

Threads make a program faster on work that can be split up, and they add complexity. Once threads run at the same time, nothing guarantees which one runs first, so the output
can change from run to run. The problems that come with this are race conditions, deadlocks, and bugs that appear only in some timings.

`thread::spawn` returns a `JoinHandle`. Calling its `join` method waits until the thread has finished:

```rust,ignore
let handle = thread::spawn(|| {
    for i in 1..10 {
        println!("hi number {i} from the spawned thread!");
        thread::sleep(Duration::from_millis(1));
    }
});

handle.join().unwrap();
```

When the main thread ends, every spawned thread is stopped, whether it has finished or not. `join` prevents that by holding the main thread until the spawned one is done.

The sample calls `join` before the main thread's own loop. The spawned thread finishes first, and then the main loop runs. The output is always the same:

```
hi number 1 from the spawned thread!
hi number 2 from the spawned thread!
...
hi number 9 from the spawned thread!
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 3 from the main thread!
hi number 4 from the main thread!
```

If `join` is called after the main loop instead, the two loops overlap. The lines from each thread then interleave, and the order changes between runs. That's the normal behavior of
threads, and it's why `join` matters when you need a particular order. `thread::sleep` makes the interleaving easier to see, but it doesn't control the order. It only gives
the other thread a chance to run.

`unwrap` on `join` handles the case where the spawned thread panicked. `join` returns an error in that case, and the program stops with a message.

---

### Try it in the runner

Press **Run**. The output should show the nine spawned lines first, then the four main lines.

The comments in `src/main.rs` suggest moving `handle.join().unwrap();` below the main loop. Then the two loops overlap, and the lines interleave. Run it a few times, and you may see a different order each time.

---

*For more detail, see section 16.1 "Using Threads to Run Code Simultaneously" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch16-01-threads.html).*
