## Reading the Request

Now the server reads what the browser sends. HTTP is a text protocol, and a request has this shape:

```text
Method Request-URI HTTP-Version CRLF
headers CRLF
message-body
```

The first line is the request line. It says what the client wants: `GET` for asking for information, the path such as `/`, and the HTTP version. After it come the headers, and then a blank line. A `GET` request has
no body.

The `handle_connection` function reads those lines from the stream:

```rust,ignore
let buf_reader = BufReader::new(&stream);
let http_request: Vec<_> = buf_reader
    .lines()
    .map(|result| result.unwrap())
    .take_while(|line| !line.is_empty())
    .collect();
```

`BufReader` wraps the stream with buffering. `lines` splits the data at each newline, and `take_while` stops at the blank line that ends the request. The output in the Output pane, once a browser connects, looks like this, with
the header values depending on your browser:

```
Request: [
    "GET / HTTP/1.1",
    "Host: 127.0.0.1:7878",
    "User-Agent: ...",
    "Accept: text/html,...",
]
```

The request line tells you the method, `GET`, the path, `/`, and the version, `HTTP/1.1`. The server doesn't send a response yet, so the browser still shows an error page.

The `stream` parameter here is not `mut`, because only reading is done. The next step writes to the stream, and needs `mut`.

---

### Try it in the runner

Press **Run**, then open `http://127.0.0.1:7878/test` in a browser. The first line of the output changes from `GET / HTTP/1.1` to `GET /test HTTP/1.1`, so the path is part of the request. The browser still shows an error, because no response is sent yet.

Stop the server before you run the next step.

---

*For more detail, see section 21.1 "Building a Single-Threaded Web Server" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch21-01-single-threaded.html).*
