## Solution: Generic Stack

This walkthrough explains the example solution in `src/main.rs`. Read it after you've tried the exercise yourself.

### Run the solution

Press **F5**. The Output pane shows:

```
stack: [1, 2, 3]
top: Some(3)
popped: Some(3)
stack: [1, 2], length 2
```

### How it works

The struct is generic over `T`, and the methods that don't print or compare items are in an `impl<T>` block. They work for any type, so `Stack<i32>`,
`Stack<String>`, and any other type all have them.

`Display` is in a separate block, `impl<T: Display>`. That bound is the only thing the printing code needs: each item must have a `Display` implementation,
so `write!(f, "{item}")` works. A stack of a type that isn't displayable simply doesn't have `Display`, and the compiler says so, rather than failing when the code
runs.

`fmt` writes the opening bracket, then each item with a comma before every item except the first, then the closing bracket. The `?` after each `write!` passes
any formatting error back to the caller. The final `write!` returns the result directly, since it's the last expression.

`peek` returns `Option<&T>`, so it borrows the top item without removing it. `pop` returns `Option<T>`, which moves the item out of the stack. Both return `Option`,
because an empty stack has no top item.

### The extensions

`is_empty` needs no bound, because it only checks the length. It belongs in the unbounded block, like `len`. Putting it in the bounded block would restrict it
for no reason.

Deriving `Debug` requires each item to be `Debug`, and the derived output shows the field name and the vector: `Stack { items: [1, 2, 3] }`.

A `Result` for `pop` would name the failure, such as "stack is empty". `Option` is usually the better fit, because an empty stack is an ordinary state, not an error.
The caller can use `match` or `?`, and a `None` already says what happened.

---

*This walkthrough accompanies the exercise in Chapter 10 of the course. It's not from the book.*
