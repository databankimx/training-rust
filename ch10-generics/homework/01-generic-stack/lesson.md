## Homework: Generic Stack

**The problem:** A stack is a last-in, first-out collection. Write a generic `Stack<T>` that holds a `Vec<T>`, and give it these methods:

* `new() -> Self`, which makes an empty stack.
* `push(&mut self, item: T)`, which adds an item on top.
* `pop(&mut self) -> Option<T>`, which removes and returns the top item, if there is one.
* `peek(&self) -> Option<&T>`, which looks at the top item without removing it.
* `len(&self) -> usize`, which returns the number of items.

Then implement `Display` for `Stack<T>`, so it prints its items in square brackets, separated by commas. Printing needs each item to be displayable, so the
implementation needs a trait bound.

Try writing it yourself before you look at the example.

---

### Hints

* Put the basic methods in an `impl<T>` block. They don't need any bound, because they never print or compare an item.
* Put `Display` in its own `impl<T: Display>` block. The bound is what allows `write!` to print each item.
* `Vec` already has `pop` and `last`. Your methods can call them directly.
* In `fmt`, `write!` returns a `Result`, and `?` passes any error up. Loop with `enumerate` so you can skip the comma before the first item.

---

### Test your code

Your program should print these four lines:

```
stack: [1, 2, 3]
top: Some(3)
popped: Some(3)
stack: [1, 2], length 2
```

The `peek` call doesn't remove anything, and `pop` does. Check the last line by hand.

---

### Extensions

* Add an `is_empty(&self) -> bool` method. Should it be in the bounded block or the unbounded one, and why?
* Implement `Debug` for `Stack<T>` by deriving it, and print the stack with `{:?}`. What does the derived output look like?
* Make `pop` return a `Result` with an error message when the stack is empty, instead of `Option`. Which is a better fit for a stack, and why?

---

*This exercise is not from the book. It practices generic types and trait bounds from Chapter 10.*
