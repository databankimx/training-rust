## Solution: A Pipeline

This walkthrough explains the example solution in `src/main.rs`. Read it after you've written your own version.

### Run the solution

Press **Run** for the output, and **Test** for the tests. The output is:

```
squares: [1, 4, 9, 16, 25]
```

### How it works

There are two channels, and each thread owns one end of each. The producer owns the transmitter for the numbers, and the worker owns the receiver for them, plus the transmitter for
the squares. The main thread owns the last receiver.

Each thread finishes when its closure ends. Dropping a transmitter is what tells the receiving side that no more values are coming. When the producer finishes, its transmitter is dropped,
so the worker's loop over the numbers ends. When the worker finishes, its transmitter for the squares is dropped, so the main thread's `collect` ends. That chain of drops is the
whole shutdown protocol, and nothing has to be signaled explicitly.

The `collect` happens before the joins. Each thread is joined after the values are taken, so neither thread can block on a full queue, which channels don't have anyway. The joins
then confirm that both threads ended without a panic.

Because one thread sends in order, and the next handles messages in order, the output is the same on every run. A pipeline with several workers would need the results sorted,
or tagged with their positions, to keep the order.

### The extensions

A filter belongs in the worker, if the stages are kept simple. Adding a third thread for the filter is also valid, and makes each stage do one job, at the cost of one more channel.

If `main` kept a clone of the producer's transmitter, the worker's loop would never end, because one transmitter would still be alive. `collect` in `main` would wait forever too, since the
worker would never drop its own transmitter. The rule is that a channel ends only when every transmitter is dropped, and a stray clone keeps it open.

---

*This walkthrough accompanies the exercise in Chapter 16 of the course. It's not from the book.*
