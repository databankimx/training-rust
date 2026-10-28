## Testing with a Mock Object

A mock object is a test double: a stand-in for a real type that records what happens during a test, so the test can check it. This step builds a small library,
`LimitTracker`, which tracks a value against a maximum. It doesn't know how to deliver messages. It asks a `Messenger` to send them, and each application provides its own
`Messenger`. A quota tracker for API calls is one example.

```rust,ignore
pub trait Messenger {
    fn send(&self, msg: &str);
}
```

`set_value` compares the value with the maximum, and sends a message at each level: 75, 90, and 100 percent. The `main` function uses a messenger that prints each one:

```
Warning: You've used up over 75% of your quota!
Urgent warning: You've used up over 90% of your quota!
Error: You are over your quota!
```

Setting the value to 50 sends nothing, because it's below the first level. The levels are checked from the highest down, so a value of 95 gets the urgent warning, not the
ordinary one.

---

### Why a mock needs RefCell

The tests need to check which messages were sent, and the mock is the place to keep them. The trait's `send` takes `&self`, so the mock can't change its own field directly.
`RefCell` provides the change at runtime, inside the mock:

```rust,ignore
struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
}

impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        self.sent_messages.borrow_mut().push(String::from(message));
    }
}
```

The `send` method still has the signature the trait requires. Inside it, `borrow_mut` gives access to the vector, and `push` adds the message. The test then reads the list
with `borrow`. Changing the trait to take `&mut self` would have been the other option, but the trait is part of the library's design, and it shouldn't change only for the
sake of a test.

---

### Try it in the runner

Press **Run** for the three messages. Then press **Test** for the two unit tests. Both should pass.

The comments in `src/main.rs` suggest changing 95 to 90. The message that appears is the urgent warning, because the check is "at least 90 percent".

---

*For more detail, see section 15.5 "RefCell<T> and the Interior Mutability Pattern" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html).*
