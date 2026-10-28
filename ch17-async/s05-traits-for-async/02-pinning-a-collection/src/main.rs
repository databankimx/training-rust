use std::future::Future;
use std::pin::Pin;

fn main() {
    trpl::block_on(async {
        // Each async block has its own type, so the Vec holds trait objects. Box::pin puts each future on the heap, pinned.
        let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> = vec![
            Box::pin(async { println!("one") }),
            Box::pin(async { println!("two") }),
            Box::pin(async { println!("three") }),
        ];

        // join_all polls the futures in the order they are in the Vec.
        trpl::join_all(futures).await;
    });

    // Try: change the Vec type to Vec<Box<dyn Future<Output = ()>>> and each Box::pin to Box::new. Which trait does the compiler say is missing?
}
