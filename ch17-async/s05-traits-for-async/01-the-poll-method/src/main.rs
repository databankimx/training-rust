use std::future::Future;
use std::pin::{Pin, pin};
use std::task::{Context, Poll, Waker};

// A future that isn't ready until it has been polled a few times.
struct CountDown {
    remaining: u32,
}

impl Future for CountDown {
    type Output = &'static str;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.remaining == 0 {
            Poll::Ready("done")
        } else {
            self.remaining -= 1;
            // A real future would store the waker, and call it when it can make progress. Here the loop below polls again directly.
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

fn main() {
    // An async runtime does this loop for us. Here we write it by hand, to see what await does.
    let mut cx = Context::from_waker(Waker::noop());
    let mut fut = pin!(CountDown { remaining: 3 });

    let result = loop {
        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(value) => break value,
            Poll::Pending => println!("still pending"),
        }
    };

    println!("ready: {result}");

    // Try: change remaining from 3 to 0. How many lines does the program print now?
}
