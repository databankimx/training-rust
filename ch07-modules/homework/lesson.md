## Chapter 7 Homework

This chapter covered packages, modules, paths, `use`, and splitting modules into files. The exercises below practice those ideas together.

The exercises are not from the book. Each one uses a different part of the chapter, and each is a single file, so you can run and edit
it entirely in the runner.

1. **Bank account.** Put a struct in a module, keep its balance private, and expose it through public methods. Then use it with `use`.
2. **Garden privacy.** A program calls functions it can't reach. Decide which items should be public, and which should stay private.
3. **Shapes and use.** A module tree of shapes, with a re-export and a nested `use`.

---

### How to use these exercises

Try each one yourself before you look at the solution. The lesson gives hints. If you get stuck, the solution walkthrough is behind
a **Show example (spoiler)** button, which shows the code and explains how it works.

When the compiler rejects a path, read the error. It usually says which item is private, or which module it can't find. Ask whether that item
should really be reachable from where you're calling it.

---

*These exercises are not from the book. They're written to practice the ideas in Chapter 7.*
