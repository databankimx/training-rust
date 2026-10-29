## Building a Single-Threaded Web Server

A web server is two protocols working together. TCP moves bytes between two computers, and does not say what they mean. HTTP sits on top of TCP, and defines what a request and a response look like. Both are request-response
protocols: a client sends a request, and the server sends a response. This course writes the HTTP handling by hand, so you can see the bytes.

The standard library's `std::net` module provides the TCP part. The first step listens on `127.0.0.1:7878`. The address has two parts. `127.0.0.1` is your own computer, and `7878` is the port. Port 7878 is
unlikely to be used by another web server on your machine.

```rust,ignore
let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

for stream in listener.incoming() {
    let _stream = stream.unwrap();
    println!("Connection established!");
}
```

`bind` returns a `Result`, which fails if another program already has the port. The `unwrap` stops the program in that case, which is enough for a learning project. `incoming` gives one stream for each connection attempt.

**Running this step.** Press **Run**. The server now waits for connections, so the Output pane stays empty until a request arrives. Open `http://127.0.0.1:7878` in a browser. The page shows an error, such as "connection reset", because the server sends nothing back yet. The Output pane shows one line per connection:

```
Connection established!
Connection established!
```

A single page load can make several connections, and this step can make the browser keep trying. The server here never sends a response, so the browser treats each attempt as failed, and retries. Edge can also open connections in advance, and it may retry every few seconds for as long as the page is open. Each `Connection established!` line is one attempt, not one page load.

To stop the repeats, close the tab, or use an InPrivate window, which keeps no connections open between loads. The repeats end in the later steps, because they send a response.

Stop the server with the Stop button before you run the next step. Only one server can use port 7878 at a time.

---

### Try it in the runner

Load the address in a browser, and count the `Connection established!` lines for one page load. The number may be more than one, for the reasons above.

---

*For more detail, see section 21.1 "Building a Single-Threaded Web Server" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch21-01-single-threaded.html).*
