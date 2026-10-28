## 21.2 From a Single-Threaded to a Multithreaded Server

A single-threaded server handles one request at a time, so one slow request holds up everything behind it. This sub-section shows the problem, tries a thread per request, and then builds a thread pool
with a fixed number of threads.

* [Simulating a slow request](#goto/01-simulated-slow-request): a `/sleep` route that blocks the server for five seconds.
* [Spawning a thread for each request](#goto/02-thread-per-request): the fix, and why it doesn't scale.
* [Building a thread pool](#goto/03-thread-pool): `ThreadPool`, `Worker`, a shared channel, and the final code.

**Running these steps:** each one is a server. Press **Run**, test in a browser at `http://127.0.0.1:7878`, and press **Stop** before the next step. Only one server can use port 7878 at a time.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
