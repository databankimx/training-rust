## Binding Values

Match arms can do more than choose code. A pattern can also *bind* a name to a part of the value it matched. That's how you pull data
out of an enum variant.

---

### A variant that holds data

The sample changes the `Quarter` variant so it holds a `UsState`. The state is data that only quarters have:

```rust,ignore
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
```

The `UsState` enum lists the states. Its details don't matter for this step, so it has two variants in the sample.

---

### Binding in the pattern

In the arm for `Quarter`, the pattern gives the inner value a name:

```rust,ignore
Coin::Quarter(state) => {
    println!("State quarter from {state:?}!");
    25
}
```

When a `Coin::Quarter` matches, `state` is bound to the `UsState` stored inside it. The arm can then use `state` in its code. The arm has
curly brackets because it has two lines, and its last expression, `25`, is the value of the arm.

The sample calls the function for each coin, and the output is:

```
1
5
10
State quarter from Alaska!
25
State quarter from Alabama!
25
```

The `println!` inside the arm runs before the `println!` in `main` that prints the returned value.

---

### Why the other arms have no binding

The `Penny`, `Nickel`, and `Dime` arms have no data to bind, so their patterns just name the variant. The same `match` can mix arms that bind
and arms that don't. Each arm's pattern describes the value it expects.

---

### Try it in the runner

Press **F5**. The Output pane should show:

```
1
5
10
State quarter from Alaska!
25
State quarter from Alabama!
25
```

The comments in `src/main.rs` suggest changing the arm to `Coin::Quarter(_)`. The program still compiles, but the state is no longer bound,
so nothing is printed for it. The `_` pattern matches the value without naming it.

---

*For more detail, see section 6.2 "The match Control Flow Construct" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch06-02-match.html).*
