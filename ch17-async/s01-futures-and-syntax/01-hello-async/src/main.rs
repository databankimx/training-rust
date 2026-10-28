use std::time::Duration;

// Calling an async function doesn't run it. It returns a future, and the body runs only when the future is awaited.
async fn greet(name: &str) -> String {
    // sleep stands in for slow work, such as waiting for a network response.
    trpl::sleep(Duration::from_millis(100)).await;
    format!("Hello, {name}, from an async function!")
}

fn main() {
    // block_on runs a future to completion on a runtime, and returns the value it produces.
    let message = trpl::block_on(greet("Ada"));
    println!("{message}");

    // Try: call greet("Grace") as well, and print that message too. Does the order of the two lines change?
}
