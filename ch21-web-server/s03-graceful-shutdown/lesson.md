## 21.3 Graceful Shutdown and Cleanup

The thread pool serves requests, but stopping the program stops every thread at once. This sub-section adds a `Drop` implementation that closes the job channel, and waits for each worker to finish its job.

* [Shutting down the pool](#goto/01-shut-down-the-pool): `Drop`, closing the channel, `join`, and the two-request server that shows the shutdown.

**Running this step:** the server stops itself after two connections. Make the two requests with `curl`, not a browser, because a browser's extra connections would use them up.

---

*This overview is adapted from the Brown University edition of The Rust Programming Language.*
