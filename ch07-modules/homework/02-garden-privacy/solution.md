## Solution: Garden Privacy

This walkthrough explains the example solution in `src/main.rs`. Read it after you've tried the exercise yourself.

### Run the solution

Press **F5**. The Output pane shows:

```
planted beans with compost
harvested in summer
It is summer
```

### How it works

The compiler rejected two calls. The solution decides each one on purpose.

`fertilizer` stays private. It's an implementation detail of `plant`, and nothing outside `vegetables` needs it. `plant` can call it, because a function in
a module can use the private items of that module. Keeping it private means you can change how the fertilizer works, without breaking callers.

`season` becomes `pub`. It describes the garden, and callers might reasonably ask for it. `harvest` already used it, and `main` can now also read it.

The solution's `main` calls only the items that are public. The line that called `fertilizer` is gone, because the public `plant` gives callers what they need.

### The extensions

If `fertilizer` were public, every caller would depend on it. Changing the fertilizer later would then be a breaking change for them. A good rule is to expose
the smallest set of items that callers actually need.

`pub(crate)` makes `season` visible across the whole crate, but not to other crates. In a library, that's a good choice for helpers that other modules
of your crate need, but that outside callers shouldn't use.

Moving `harvest` into `vegetables` works, because `vegetables` is a child of `garden`, so it can use the private `season` through `super::season()`. The
function `harvest` would then need to be `pub` for `main` to call it, as `garden::vegetables::harvest()`.

---

*This walkthrough accompanies the exercise in Chapter 7 of the course. It's not from the book.*
