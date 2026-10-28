## Homework: A Pipeline

**The problem:** Build a two-stage pipeline with channels. A producer thread sends numbers to a worker thread, which squares each one and sends the result on to the main thread.
Write a function `square_through_threads(numbers: Vec<u64>) -> Vec<u64>` that does this, and returns the squares in the order the numbers were sent.

Create a project in VS Code with `cargo new two_stage_pipeline`. In `src/main.rs`:

* Create two channels: one for the numbers, and one for the squares.
* Start a producer thread, which sends each number on the first channel, and then finishes.
* Start a worker thread, which takes numbers from the first channel, squares each one, and sends it on the second channel.
* In the function's own thread, collect the squares from the second channel into a `Vec`, and join both threads.
* In `main`, call the function with the numbers 1 to 5, and print the result.

Write tests in a `tests` module for an empty list, a single number, and a short list.

---

### Hints

* A receiver used as an iterator ends when every transmitter for its channel has been dropped. Each thread drops its transmitter when its closure ends, so think about which closure owns each one.
* The worker's loop is `for n in numbers_rx`. The body sends `n * n` on the other channel.
* `mpsc::channel::<u64>()` names the type of the channel, which helps when the compiler can't infer it.

---

### Check your work

Your program should print:

```
squares: [1, 4, 9, 16, 25]
```

The order matches the input, because one thread sends in order, and the next thread handles the messages in the same order.

---

### Extensions

* Add a third stage that keeps only the squares greater than 10. Where does the filter belong: in the worker, or in a new thread?
* What happens if you forget to drop the producer's transmitter, for example by keeping a clone in `main`? Which part of the program would wait forever?

---

*This exercise is not from the book. It practices channels and thread ownership from Chapter 16.*
