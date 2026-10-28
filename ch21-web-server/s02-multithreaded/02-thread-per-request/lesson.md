## Spawning a Thread for Each Request

The first fix is to give each connection its own thread. The `for` loop hands each stream to `thread::spawn`, and the handling runs on that thread:

```rust,ignore
for stream in listener.incoming() {
    let stream = stream.unwrap();

    thread::spawn(|| {
        handle_connection(stream);
    });
}
```

**Running this step.** Press **Run**. Load `http://127.0.0.1:7878/sleep` in one tab, and `http://127.0.0.1:7878` in another. The second tab now loads straight away, because the slow request has its own thread.

This works, but it doesn't scale. Every connection creates a thread, with no limit. A flood of requests, deliberate or not, would start a huge number of threads and use up the machine's resources. The next step limits the number of threads, and reuses them.

Stop the server before you run the next step.

---

### Try it in the runner

Load `/sleep` in one tab, then `/` in two more tabs. The `/` tabs load immediately, because each connection has its own thread, and the sleep doesn't block them.

---

*For more detail, see section 21.2 "From a Single-Threaded to a Multithreaded Server" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch21-02-multithreaded.html).*
