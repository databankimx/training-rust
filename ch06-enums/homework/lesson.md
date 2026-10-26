## Chapter 6 Homework

This chapter covered enums, and the ways to handle their variants: `match`, `if let`, and `let...else`. The exercises below
practice those ideas together, including one that uses `Option` in a realistic setting.

The exercises are not from the book. Each one uses a different part of the chapter.

1. **Traffic light.** Model a traffic light as an enum, with a method that moves to the next light, and a method that gives its duration.
2. **Command evaluator.** Model a few commands as an enum, and write a function that runs each one with `match`.
3. **Optional port.** Start from a function that uses a verbose `match`, and rewrite it with `let...else`.

---

### How to use these exercises

Try each one yourself before you look at the solution. The lesson gives hints. If you get stuck, the solution walkthrough is behind
a **Show example (spoiler)** button, which shows the code and explains how it works.

When the compiler rejects a `match`, read the error. It names the pattern that isn't covered, and that's usually the arm you forgot.

---

*These exercises are not from the book. They're written to practice the ideas in Chapter 6.*
