## Simulating a Slow Request

The single-threaded server handles one request at a time, so one slow request holds up all the others. This step shows that problem before it's fixed.

The server gets a new route, `/sleep`. Its handler waits for five seconds before it sends the page:

```rust,ignore
let (status_line, filename) = match &request_line[..] {
    "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
    "GET /sleep HTTP/1.1" => {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    }
    _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
};
```

The `match` is used rather than `if`, because there are three cases. The slice pattern `&request_line[..]` lets the string literals be matched, as Chapter 19 covered.

**Running this step.** Press **Run**, then open two browser windows. In one, load `http://127.0.0.1:7878/sleep`. In the other, load `http://127.0.0.1:7878`. The second window waits until the first has finished its five seconds, even though the page is quick to produce. The Output pane stays empty.

Stop the server before you run the next step.

---

### Try it in the runner

Load `/sleep` in one tab, and `/` in a second tab straight after it. The second tab waits for about five seconds, so the server can handle only one request at a time.

---

*For more detail, see section 21.2 "From a Single-Threaded to a Multithreaded Server" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch21-02-multithreaded.html).*
