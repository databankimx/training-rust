use std::{future::Future, time::Duration};

use trpl::Either;

// Waits for a future, but gives up after limit. Returns None if the timer finishes first.
async fn within<F: Future>(future: F, limit: Duration) -> Option<F::Output> {
    match trpl::select(future, trpl::sleep(limit)).await {
        Either::Left(output) => Some(output),
        Either::Right(_) => None,
    }
}

fn main() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        for word in ["first", "second", "third"] {
            tx.send(word).unwrap();
        }

        // tx stays alive until the end of this block, so the channel never closes. Only the timeout can end the loop.
        loop {
            match within(rx.recv(), Duration::from_millis(500)).await {
                Some(Some(word)) => println!("got {word}"),
                Some(None) => {
                    println!("channel closed");
                    break;
                }
                None => {
                    println!("nothing arrived within 500 ms");
                    break;
                }
            }
        }

        drop(tx);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quick_future_returns_some() {
        let result = trpl::block_on(within(async { 7 }, Duration::from_millis(500)));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn slow_future_returns_none() {
        let slow = async {
            trpl::sleep(Duration::from_secs(5)).await;
            7
        };
        let result = trpl::block_on(within(slow, Duration::from_millis(100)));
        assert_eq!(result, None);
    }
}
