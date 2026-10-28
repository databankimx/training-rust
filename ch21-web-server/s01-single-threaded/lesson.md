## 21.1 Building a Single-Threaded Web Server

This sub-section builds a web server by hand, with the standard library's TCP and string handling. It starts with a listener, reads the request, writes a response, and finally serves HTML pages and a 404 page.

* [Listening for connections](#goto/01-listen-for-connections): `TcpListener`, and the port 7878.
* [Reading the request](#goto/02-read-the-request): the HTTP request format, and `BufReader`.
* [Writing a response](#goto/03-write-a-response): the status line, and sending bytes with `write_all`.
* [Serving HTML and a 404 page](#goto/04-serve-html): choosing a file from the request, and `Content-Length`.

**Running these steps:** each one is a server that keeps running. Press **Run**, then test in a browser at `http://127.0.0.1:7878`. The Output pane shows nothing until a request arrives. Press **Stop** before you run the next step,
because only one server can use port 7878 at a time.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
