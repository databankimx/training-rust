## Building a Thread Pool

Spawning a thread for each request has no limit. A thread pool fixes that: it creates a fixed number of threads when the server starts, and sends each connection to one of them. Extra connections wait in
a queue until a thread is free.

The design has three parts, and each one comes from an earlier chapter:

* **`ThreadPool`** holds the workers, and the sending end of a channel. `execute` sends a job down the channel.
* **`Worker`** is a thread that loops, takes a job from the channel, and runs it.
* **The receiver** is shared by all the workers. It's wrapped in `Arc<Mutex<...>>`, because the channel allows one receiver, and each job must be taken by exactly one worker. Chapter 16 covered both.

A job is a closure that runs once, so it's stored as a boxed trait object:

```rust,ignore
type Job = Box<dyn FnOnce() + Send + 'static>;
```

In each worker, the lock is taken only for the `recv` call. The statement ends and releases the lock before the job runs:

```rust,ignore
let job = receiver.lock().unwrap().recv().unwrap();
println!("Worker {id} got a job; executing.");
job();
```

If the lock were held while the job ran, the other workers would wait, and the pool would behave like one thread. The book's Listing 21-21 shows this mistake, using `while let`, which keeps the lock until the end of the loop body.

**Running this step.** Press **Run**. The Output pane prints a line when a worker takes a job, and the worker's number. The pool is working when you see four `got a job` lines in a row, before any of the four jobs finishes.

**Testing it.** Browsers are not a reliable way to test this. Edge, like most Chromium-based browsers, sends several requests to the same address one after another, so the tabs load in sequence, even though the server handles them at the same time. Check the server with five requests sent together, from a second PowerShell window:

```powershell
1..5 | ForEach-Object { Start-Job { curl.exe -s http://127.0.0.1:7878/sleep > $null } } | Wait-Job | Out-Null
```

All five requests are sent at once. Four finish about five seconds later, and the fifth finishes about five seconds after that, which is roughly ten seconds in total. If you time the command with `Measure-Command`, the total should be about ten seconds, not twenty-five.

Stop the server when you're finished. Section 21.3 adds graceful shutdown, so the pool can be closed cleanly.

---

### Try it in the runner

Run the PowerShell commands above while the server runs. The Output pane should show `Worker 0`, `Worker 1`, `Worker 2`, and `Worker 3` taking jobs at the start, then the first worker again for the fifth job.

---

*For more detail, see section 21.2 "From a Single-Threaded to a Multithreaded Server" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch21-02-multithreaded.html).*
