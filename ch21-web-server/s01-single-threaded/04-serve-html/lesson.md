## Serving HTML and a 404 Page

This step sends real content. The server checks the request line. If it's `GET / HTTP/1.1`, it sends `hello.html` with status 200. For any other request, it sends `404.html` with status 404, which
means "not found".

```rust,ignore
let request_line = buf_reader.lines().next().unwrap().unwrap();

let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
    ("HTTP/1.1 200 OK", "hello.html")
} else {
    ("HTTP/1.1 404 NOT FOUND", "404.html")
};
```

The `if` chooses two values, and a tuple pattern in the `let` takes them apart. The code after the `if` is shared by both cases, so it's written only once. It reads the chosen file, and builds the response:

```rust,ignore
let contents = fs::read_to_string(filename).unwrap();
let length = contents.len();

let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
stream.write_all(response.as_bytes()).unwrap();
```

The `Content-Length` header tells the browser how many bytes of body to expect. Without it, the browser can't tell where the response ends.

**Running this step.** The two HTML files are in this step's folder, and `cargo run` reads them from there. Press **Run**, and open two addresses in the browser:

- `http://127.0.0.1:7878` shows the page "Hello!" with the text "Hi from Rust".
- `http://127.0.0.1:7878/something-else` shows "Oops!", the 404 page.

The Output pane stays empty, because the server prints nothing.

Stop the server when you're finished. Chapter 21.2 makes this server handle several requests at once.

---

### Try it in the runner

Request `http://127.0.0.1:7878/something-else`. The status line of the response is now `HTTP/1.1 404 NOT FOUND`, and the body is the 404 page. The browser shows the same page for any path other than `/`.

---

*For more detail, see section 21.1 "Building a Single-Threaded Web Server" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch21-01-single-threaded.html).*
