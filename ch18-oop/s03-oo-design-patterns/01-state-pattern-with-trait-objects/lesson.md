## The State Pattern with Trait Objects

The state pattern is an object-oriented design pattern. A value holds a state object, and its behavior depends on that state. Each state decides which requests it accepts, and which state comes
next. The value itself knows nothing about the rules for each state.

This step builds a blog post that moves through three states: draft, pending review, and published. Only a published post returns its text. The rules live in three structs, and each one implements a
trait called `State`:

```rust,ignore
trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;

    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}
```

`Post` holds the current state in a `Box<dyn State>`, inside an `Option`. The method `request_review` takes ownership of the state, and returns the next one. `Option::take` moves the state
out of the field, and leaves `None` behind, so the old state can't be used afterward:

```rust,ignore
pub fn request_review(&mut self) {
    if let Some(s) = self.state.take() {
        self.state = Some(s.request_review())
    }
}
```

The `self: Box<Self>` receiver means the method can be called only on a boxed state, which is what allows the state to be consumed. Each state chooses its own transition. A draft moves to pending review on
`request_review`, and stays a draft on `approve`. A pending review is approved into `Published`. A published post stays published:

```rust,ignore
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
```

The `content` method has a default that returns an empty string, and `Published` overrides it to return the text. The lifetime `'a` ties the returned text to the post it comes from.

The program prints the content at each stage:

```
draft content: ''
pending review content: ''
published content: 'I ate a salad for lunch today'
```

The text is stored from the start, but nothing is shown until the post is published.

---

### Trade-offs

Adding a state, such as `Scheduled`, means writing one struct and implementing the trait on it. The `Post` code doesn't change. The cost is that the states know about each other, because `PendingReview`
must name `Published` as its next state. The book also notes that the default methods can't simply return `self` for every transition, because a trait object doesn't know its concrete type.

The next step shows an alternative, which turns some of these mistakes into compile errors.

---

### Try it in the runner

Press **Run**. The output should match the three lines above.

The comments in `src/main.rs` suggest calling `approve` directly after `add_text`, before the review is requested. The state is still `Draft`, so `approve` has no effect, and the content stays empty.

---

*For more detail, see section 18.3 "Implementing an Object-Oriented Design Pattern" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch18-03-oo-design-patterns.html).*
