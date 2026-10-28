## Chapter 17 Homework

This chapter covered futures, `async` and `await`, joining and racing futures, channels, streams, and how tasks relate to threads. The exercises below each use one of these
ideas. Each one is a project to build in VS Code. Use `cargo new`, and add `trpl = "0.3.0"` to its `Cargo.toml` with `cargo add trpl@0.3.0`. The first build downloads the
crate and its dependencies, as it did for the chapter's steps. Once you've tried an exercise, reveal its example solution in the runner, and compare.

1. **Gather forecasts.** Fetch several results at the same time with `join_all`, and print them in the order they were asked for.
2. **A timed receiver.** Read messages from an async channel, and stop when nothing arrives within a time limit.
3. **A sensor stream.** Filter a stream of readings, keep the first few valid ones, and total them.

The exercises are not from the book.

---

*These exercises are not from the book. They practice the ideas in Chapter 17.*
