use std::{thread, time::Duration};

fn main() {
    trpl::block_on(async {
        let a = async {
            println!("'a' started.");
            slow("a", 30);
            slow("a", 10);
            slow("a", 20);
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            slow("b", 10);
            slow("b", 15);
            slow("b", 350);
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'b' finished.");
        };

        // select returns when the first future finishes.
        trpl::select(a, b).await;
    });

    // Try: move the trpl::sleep call to the start of each future. Does the output change?
}

// thread::sleep blocks the whole thread, not just this future. Nothing else can run until it returns.
fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}
