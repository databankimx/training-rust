## Disambiguating Between Methods with the Same Name

Rust doesn't stop a type from having two methods with the same name, one from each of two traits, or one from a trait and one written directly on the type. When that happens, Rust needs a way to
know which method you mean.

The sample has a `Human` that implements `Pilot` and `Wizard`, and has a `fly` method of its own. All three have the same name:

```rust,ignore
impl Pilot for Human {
    fn fly(&self) { /* ... */ }
}

impl Wizard for Human {
    fn fly(&self) { /* ... */ }
}

impl Human {
    fn fly(&self) { /* ... */ }
}
```

A plain method call, `person.fly()`, uses the method written directly on `Human`. To call a trait's version, name the trait:

```rust,ignore
Pilot::fly(&person);
Wizard::fly(&person);
```

The output is:

```
*waving arms furiously*
This is your captain speaking.
Up!
```

**Functions with no `self`.** The method names are disambiguated by the receiver, the value the method is called on. A function that doesn't take `self` has no receiver, so the compiler can't tell which type or trait is meant.
The sample has `Dog::baby_name`, written on `Dog`, and an `Animal` trait with its own `baby_name`:

```rust,ignore
println!("A baby dog is called a {}", Dog::baby_name());
println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
```

`Dog::baby_name()` calls the function on `Dog`, which prints `Spot`. The second line uses *fully qualified syntax*, `<Dog as Animal>`, which says exactly which implementation to call. The output is:

```
A baby dog is called a Spot
A baby dog is called a puppy
```

Fully qualified syntax has the form `<Type as Trait>::function(...)`. You only need it when Rust can't work out the answer from the code, and you can use it anywhere to be explicit.

---

### Try it in the runner

Press **Run**. The output should match the five lines above.

The comments in `src/main.rs` suggest deleting the `fly` method written on `Human`. Then `person.fly()` is ambiguous, because two traits both provide one, and the compiler reports that there are multiple applicable items.

---

*For more detail, see section 20.2 "Advanced Traits" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch20-02-advanced-traits.html).*
