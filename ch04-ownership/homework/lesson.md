## Chapter 4 Homework

You've reached the end of Chapter 4, which is the hardest chapter in the book for many people. It covered ownership, moves,
references and borrowing, the rules that keep them safe, slices, and how to fix the errors the borrow checker reports. If
those ideas feel tangled, that's normal, and the exercises below are a good way to untangle them.

The exercises are not from the book. They're designed to practise the ideas together, and each one uses a different part of
the chapter.

---

### The exercises

1. **Longest word.** Write a function that takes a sentence and returns a slice of its longest word. You'll need string slices,
   and a signature that ties the result to the input.
2. **Fix the errors.** You're given a program that the borrow checker rejects, and you'll fix it in three different ways. You'll
   need the ideas from the fixing ownership errors step.
3. **Halves of a vector.** Write a function that splits a vector into two halves, and returns a mutable reference to each.
   You'll need `split_at_mut`, and you'll need to explain why the compiler accepts it.

---

### How to use these exercises

Try each one yourself before you look at the solution. The lesson gives hints. If you get stuck, the solution walkthrough is
behind a **Show example (spoiler)** button, which shows the code and explains how it works.

The compiler's errors are your best teacher here. When it rejects your code, read the error from top to bottom, and ask which
permission it says is missing, and which borrow is still in use.

---

*These exercises are not from the book. They're written to practise the ideas in Chapter 4.*
