## Writing a Response

A response has a shape similar to a request:

```text
HTTP-Version Status-Code Reason-Phrase CRLF
headers CRLF
message-body
```

The status line starts with the HTTP version, then a numeric status code, and then a short text reason. `200` means success, and `OK` is its reason phrase. The smallest valid successful response has no headers
and no body:

```text
HTTP/1.1 200 OK\r\n\r\n
```

The `\r\n` is a CRLF, the end of a line. The second CRLF, straight after the first, is the blank line that ends the headers. With no body, the response ends there.

This step writes that response to the stream:

```rust,ignore
let response = "HTTP/1.1 200 OK\r\n\r\n";
stream.write_all(response.as_bytes()).unwrap();
```

`as_bytes` turns the string into bytes, and `write_all` sends them. The function takes `mut stream`, because writing changes the stream. Stream data is only sent after the write, so this is the first step where the browser gets an answer.

**Running this step.** Press **Run**, and open `http://127.0.0.1:7878` in a browser. The page is blank, rather than an error. The request still isn't used; the server sends the same response for every path.

Stop the server before you run the next step.

---

### Try it in the runner

Change `200 OK` to `404 NOT FOUND` in the response string. The browser still shows a blank page, because the status code is only a label, and the browser has no body to display. The status code is what tells the browser, and any tools, that the request failed.

---

*For more detail, see section 21.1 "Building a Single-Threaded Web Server" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch21-01-single-threaded.html).*
