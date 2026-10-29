use std::time::Duration;

fn main() {
    trpl::block_on(async {
        // Each future is an async block. Creating them does nothing yet.
        let fut1 = async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        // join runs both futures to completion, checking each in turn. The result is the same every run.
        trpl::join(fut1, fut2).await;
    });

    // Try: change the second loop to run from 1 to 10. What happens to the output after the second task ends?
}
