## Homework: Parallel Sum

**The problem:** Add up a list of numbers faster by splitting it into chunks, and adding each chunk on its own thread. Write a function
`parallel_sum(values: Vec<u64>, threads: usize) -> u64` that does this.

Create a project in VS Code with `cargo new parallel_sum`. In `src/main.rs`:

* Split the values into `threads` chunks of roughly equal size. The last chunk may be a little smaller. `chunks` on a slice does most of the work.
* Start one thread for each chunk. Each thread gets its own copy of its chunk, because a spawned thread needs values it owns.
* Wait for the threads with `join`, and add their partial sums.
* In `main`, call `parallel_sum` on the numbers 1 to 1,000 with 4 threads, and print the result.

Write tests in a `tests` module. They should check that the parallel sum matches a plain sum for a short list, and that it works when there are more threads than values.

---

### Hints

* `values.chunks(size)` gives slices. Calling `to_vec()` on each one gives a `Vec` the thread can own.
* Store the `JoinHandle`s in a `Vec` as you spawn them, then join them in a second loop. Joining in the same loop as spawning runs the threads one after another.
* The chunk size is the number of values divided by the threads, rounded up. Think about what happens when there are fewer values than threads.

---

### Check your work

Your program should print:

```
sum: 500500
```

The sum of 1 to 1,000 is 500,500, which you can check with the formula n × (n + 1) / 2.

---

### Extensions

* Use `std::thread::available_parallelism` to choose the number of threads, instead of a fixed number.
* Compare the time taken with 1 thread and with 4 threads, on a much larger list. Does it always get faster, and why might it not?

---

*This exercise is not from the book. It practices threads and `join` from Chapter 16.*
