## Chapter 21: Final Project: Building a Multithreaded Web Server

This chapter builds one more project, a web server that serves a page, and then handles several requests at once with a thread pool. It brings together ideas from earlier chapters: TCP and HTTP,
the standard library's I/O, `Result` and `Option`, traits, and threads.

The server is written by hand, so you can see the protocol. Production code would use a crate, but building it yourself is how the techniques are learned.

The chapter is split into sections, as the book has them:

* [21.1 Building a single-threaded web server](#goto/s01-single-threaded): listening on a port, reading a request, writing a response, and serving HTML and a 404 page.
* [21.2 From a single-threaded to a multithreaded server](#goto/s02-multithreaded): a slow request, a thread per request, and a thread pool.
* [21.3 Graceful shutdown and cleanup](#goto/s03-graceful-shutdown): a `Drop` implementation that closes the channel, and waits for each worker.

---

## Summary

The final project is a working web server. It listens for connections, reads each request, and answers with HTML, or a 404 page. A fixed pool of threads handles requests concurrently, and the pool shuts
down cleanly when it's dropped. The project uses the TCP and HTTP handling from the standard library, and the thread, channel, and smart pointer techniques from Chapters 16 and 15.

Well done. You've reached the end of the book.

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
