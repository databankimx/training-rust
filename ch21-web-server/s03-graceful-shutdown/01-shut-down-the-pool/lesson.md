## Graceful Shutdown and Cleanup

The thread pool serves requests, but it doesn't clean up. When you stop the program with Ctrl-C, every thread stops at once, even one that's in the middle of a request. This step adds a `Drop`
implementation, so the pool shuts down in an orderly way when it goes out of scope.

The shutdown has two parts.

1. **Close the channel.** `Drop` takes the sender out of the pool, and drops it. With no sender left, `recv` returns an error in every worker. Each worker then reads the error, prints a message, and
   leaves its loop:

```rust,ignore
impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
```

2. **Wait for each worker.** `join` waits until the thread finishes. A worker that is in the middle of a job finishes that job first, so no request is cut off part way.

The sender is an `Option`, and each thread handle is an `Option` too. `take` moves the value out, which `join` needs, because it takes ownership of the thread. Chapter 18 covered `Option::take`. The book also mentions `Vec::drain` as an alternative, but this version uses `Option::take`, which is the form in the final code.

To see the shutdown, the server accepts only two connections, through `take(2)`. After the second, the loop ends, and the pool is dropped:

```rust,ignore
for stream in listener.incoming().take(2) {
    // ...
}
println!("Shutting down.");
```

**Running this step.** Press **Run**. Make exactly two requests from a second PowerShell window, which avoids the extra connections a browser makes:

```powershell
curl.exe http://127.0.0.1:7878
curl.exe http://127.0.0.1:7878
```

When the second request is answered, the program prints the shutdown messages and exits, and the Output pane shows the line `Finished with exit code 0`. The output is similar to this, although the order of the worker lines can vary:

```
Worker 0 got a job; executing.
Worker 3 got a job; executing.
Shutting down.
Shutting down worker 0
Worker 1 disconnected; shutting down.
Worker 2 disconnected; shutting down.
Worker 3 disconnected; shutting down.
Worker 0 disconnected; shutting down.
Shutting down worker 1
Shutting down worker 2
Shutting down worker 3
```

Use `curl` rather than a browser here, because a browser opens extra connections, and they would use up the two the server accepts.

---

### Try it in the runner

Run the step, make the two requests with `curl`, and check that the program exits on its own, with the shutdown messages. A third request made after that gets no answer, because the server has stopped.

---

*For more detail, see section 21.3 "Graceful Shutdown and Cleanup" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch21-03-graceful-shutdown-and-cleanup.html).*
