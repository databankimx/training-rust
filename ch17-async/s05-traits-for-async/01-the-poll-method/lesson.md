## The Future Trait and poll

Every future in Rust implements the `Future` trait, which has an associated type `Output` and one method, `poll`:

```rust,ignore
pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

`poll` returns a `Poll`, which has two variants. `Ready(value)` means the future has finished, and `Pending` means it still has work to do, so the caller must check again later.

When you write `.await`, Rust compiles it into a loop that calls `poll`. If the result is `Ready`, the loop stops with the value. If the result is `Pending`, the task gives control to the
runtime, which polls the future again later. The runtime is what makes the loop non-blocking.

This sample is a future that becomes ready only after it has been polled a few times:

```rust,ignore
impl Future for CountDown {
    type Output = &'static str;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.remaining == 0 {
            Poll::Ready("done")
        } else {
            self.remaining -= 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
```

`main` runs the loop by hand, the way the runtime would:

```rust,ignore
let mut fut = pin!(CountDown { remaining: 3 });

let result = loop {
    match fut.as_mut().poll(&mut cx) {
        Poll::Ready(value) => break value,
        Poll::Pending => println!("still pending"),
    }
};
```

The output is:

```
still pending
still pending
still pending
ready: done
```

The first three polls return `Pending`, and the fourth returns `Ready`. A runtime does the same thing, but it keeps other futures moving while a future is pending, instead of
printing and polling in a tight loop.

The `Context` holds a `Waker`. A real future stores the waker, and calls it when it can make progress, so the runtime knows to poll it again. `Waker::noop()` is a waker that does nothing, which is
enough here because the loop polls every time anyway.

`pin!` pins the future on the stack. The next step explains why the `Pin` is needed.

---

### Try it in the runner

Press **Run**. The output should be the three `still pending` lines, followed by `ready: done`.

The comments in `src/main.rs` suggest starting `remaining` at 0. The future is then ready on its first poll, and the output is just `ready: done`.

---

*For more detail, see section 17.5 "A Closer Look at the Traits for Async" in [The Rust Programming Language](https://doc.rust-lang.org/book/ch17-05-traits-for-async.html).*
