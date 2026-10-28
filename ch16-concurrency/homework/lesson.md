## Chapter 16 Homework

This chapter covered threads, channels, mutexes with `Arc`, and the `Send` and `Sync` traits. The exercises below each use a different one of these. Each is a project you
write yourself in VS Code, starting with `cargo new`. Once you've tried one, reveal its example solution in the runner, and compare.

1. **Parallel sum.** Split a list of numbers into chunks, add each chunk on its own thread, and combine the results.
2. **A pipeline.** Pass numbers from one thread to another through channels, and collect the results in a third.
3. **Bank transfers.** Move money between accounts from several threads, using a mutex for each account, and keep the total correct.

If Windows reports "Access is denied" when you run a new project, security software may be blocking its program. Renaming the project, as the pipeline exercise does, is a quick way around it.

The exercises are not from the book.

---

*These exercises are not from the book. They practice the ideas in Chapter 16.*
