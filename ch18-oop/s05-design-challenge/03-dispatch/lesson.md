## Design Trade-offs: Dispatch

**Context:** You are designing a simple event system that calls callback functions in response to events.

**Functionality:** An `Events` struct provides functions to register callbacks. A callback is either parallelizable (runnable across multiple threads) or sequential (must be run only on the main thread).

The three proposed designs are:

```rust,ignore
pub struct Events {
    // ...
}

// Option 1: parallel and sequential are two separate methods
impl Events {
    pub fn register<E, F: Fn(E)>(&mut self, f: F) { /* .. */ }
    pub fn register_sequential<E, F: Fn(E)>(&mut self, f: F) { /* .. */ }
}

// Option 2: parallel and sequential are two members of an enum
pub enum Callback<F> {
    Parallel(F),
    Sequential(F),
}
impl Events {
    pub fn register<E, F: Fn(E)>(&mut self, f: Callback<F>) { /* .. */ }
}

// Option 3: parallel and sequential are markers in a trait method
pub trait Register<Marker, F, E> {
    fn register(&mut self, f: F);
}
pub struct Parallel;
pub struct Sequential;
impl<F, E> Register<Parallel, F, E> for Events
where
    F: Fn(E),
{
    fn register(&mut self, f: F) { /* .. */ }
}
impl<F, E> Register<Sequential, F, E> for Events
where
    F: Fn(Sequential, E),
{
    fn register(&mut self, f: F) { /* .. */ }
}
```

Here is how a client registers a callback under each design:

```rust,ignore
// Option 1
events.register(|e: OnClick| { /* .. */ });
events.register_sequential(|e: OnClick| { /* .. */ });

// Option 2
events.register(Callback::Parallel(|e: OnClick| { /* .. */ }));
events.register(Callback::Sequential(|e: OnClick| { /* .. */ }));

// Option 3
events.register(|e: OnClick| { /* .. */ });
events.register(|_: Sequential, e: OnClick| { /* .. */ });
```

---

### Requirement 1: a callback is parallelizable by default, and the API reflects that default

Does the design satisfy this requirement?

````quiz
Option 1, with separate `register` and `register_sequential` methods.

◉ Yes
→ Yes. The plain `register` is the parallel default, and the sequential version has a longer name, so the default is clear.
○ No
→ Not quite. The simplest method is the parallel one, so the API reflects that default.
````

````quiz
Option 2, with a `Callback` enum that requires `Parallel` or `Sequential`.

○ Yes
→ Not quite. Both variants must be named at every call, so neither one is the default.
◉ No
→ Yes. Because both `Parallel` and `Sequential` must be written, the API doesn't show which mode is the default.
````

````quiz
Option 3, with marker types on the trait method.

◉ Yes
→ Yes. A plain `register` call is the parallel default, and sequential needs an explicit marker type.
○ No
→ Not quite. Without a marker, the callback is parallel, so the API reflects the default.
````

---

### Requirement 2: the API exports as few methods as possible

Does the design satisfy this requirement?

````quiz
Option 1, with separate `register` and `register_sequential` methods.

○ Yes
→ Not quite. This design exports two distinctly named methods, which is more than necessary.
◉ No
→ Yes. Two methods are exported, when one would do.
````

````quiz
Option 2, with a single `register` method that takes a `Callback`.

◉ Yes
→ Yes. Only one method is exported, and the enum carries the choice.
○ No
→ Not quite. There's a single `register` method, so the API exports as few methods as possible.
````

````quiz
Option 3, with a single `register` method through the `Register` trait.

◉ Yes
→ Yes. The trait exports one `register` method, and the markers pick the implementation.
○ No
→ Not quite. There's a single `register` name, so the API exports as few methods as possible.
````

---

### Requirement 3: the API leans on the compiler's type inference as little as possible

Does the design satisfy this requirement?

````quiz
Option 1, with separate `register` and `register_sequential` methods.

◉ Yes
→ Yes. Each method takes a closure with a known type, and nothing depends on choosing between implementations.
○ No
→ Not quite. Option 1 needs little type inference beyond the closure type.
````

````quiz
Option 2, with a `Callback` enum.

◉ Yes
→ Yes. The enum variant names the mode directly, so the compiler doesn't need to infer it.
○ No
→ Not quite. The variant is written at the call site, so little inference is needed.
````

````quiz
Option 3, with marker types on the trait method.

○ Yes
→ Not quite. The compiler must infer the `Marker` type to choose between the two `register` implementations, so this design relies on inference.
◉ No
→ Yes. Two calls to `register` look alike, but they use different implementations, and the compiler infers which one applies from the closure's argument types.
````

---

*The design challenge is adapted from section 18.5 of [The Rust Programming Language](https://doc.rust-lang.org/book/ch18-05-design-challenge.html), and the explanations draw on the book's rationale.*
