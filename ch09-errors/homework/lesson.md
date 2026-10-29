## Chapter 9 Homework

This chapter covered when to panic, and when to return a `Result`. The exercises below practice both, and the functions that pass errors to their callers.

The exercises are not from the book. Each one uses a different part of the chapter.

1. **Settings.** Parse `key=value` lines into settings. Bad lines are reported as errors, and a required setting is checked with `expect`.
2. **Scores report.** Chain functions that return `Result`, and combine them with `?`.
3. **Convert a panic.** A program panics on bad input. Change it to return a `Result`, and let `main` decide what to print.

---

### How to use these exercises

Try each one yourself before you look at the solution. The lesson gives hints. If you get stuck, the solution walkthrough is behind a **Show example (spoiler)** button, which shows the code and explains how it works.

When you change a function to return a `Result`, the compiler will tell you where each caller needs to handle the error. Follow those messages.

---

*These exercises are not from the book. They're written to practice the ideas in Chapter 9.*
