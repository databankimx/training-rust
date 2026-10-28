## Solution: Parallel Sum

This walkthrough explains the example solution in `src/main.rs`. Read it after you've written your own version.

### Run the solution

Press **Run** for the output, and **Test** for the tests. The output is:

```
sum: 500500
```

### How it works

The function first makes sure there's at least one thread, and works out the chunk size, rounding up so that no more than `threads` chunks are made. `chunks` then gives
slices of the list. A spawned thread needs values it owns, so each slice is copied into a `Vec` with `to_vec` before its thread starts. Each thread adds its own chunk, and returns
the partial sum through its `JoinHandle`.

The handles are collected in one loop, and joined in a second. Joining inside the first loop would wait for each thread before the next one starts, which runs them one after another and
loses the point. Once every handle is joined, the partial sums are added up with `sum`.

The tests cover the cases that tend to break. Ten values with three threads gives chunks of 4, 4, and 2. Two values with eight threads makes two one-value chunks, and the
extra threads are never started. An empty list makes no threads at all, and sums to zero.

### The extensions

`std::thread::available_parallelism` returns the number of threads the machine can run at once. It returns a `Result`, so a fallback is needed if it fails.

Speed depends on the size of the list. Starting a thread costs time, and for a small list that cost can be larger than the work saved. On a large list, the threads run in
parallel only if the machine has spare cores. Measuring with several runs gives the honest answer for your machine.

---

*This walkthrough accompanies the exercise in Chapter 16 of the course. It's not from the book.*
