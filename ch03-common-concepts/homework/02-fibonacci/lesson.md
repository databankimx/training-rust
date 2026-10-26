## Homework: Fibonacci Numbers

**The problem:** Generate the nth Fibonacci number.

The Fibonacci sequence starts with 0 and 1. Each number after that is the sum of the two before it. Your
program should ask for a position, and print the number at that position. Position 0 is the first number,
0, and position 1 is the second, 1.

Try writing your own version before you look at the example.

---

### Hints

* You need two values, the two most recent numbers in the sequence. Each time round a loop, you can
  move both of them forward one step.
* Work out what the loop should do on the first step, and what the answer is when the loop has run
  for a position of 0 or 1. Special cases are often the easiest way to make a loop correct.
* The numbers grow quickly. Eventually they won't fit in the type you've chosen. Decide what your program
  should do when that happens, rather than letting it panic.
* You met a method in the scalar types step that returns `None` on overflow instead of panicking. It
  might help.

---

### Test your program

Check these by hand before you trust the program:

* Position 0 should give 0.
* Position 1 should give 1.
* Position 10 should give 55.
* Position 20 should give 6765.

For a very large position, your program should say the number is too big, rather than crash.

---

### Extensions

* Change the type of your numbers to `u128`. What is the largest position you can now reach?
* Print the first ten Fibonacci numbers, one per line, without asking the player for input.
* Write a version that doesn't stop at the type's limit. What would it need to return instead of a number?

---

*This exercise comes from the summary at the end of Chapter 3 in [The Rust Programming Language](https://rust-book.cs.brown.edu/ch03-05-control-flow.html#summary).*
