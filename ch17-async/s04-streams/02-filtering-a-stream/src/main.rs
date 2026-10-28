use trpl::StreamExt;

fn main() {
    trpl::block_on(async {
        let stream = trpl::stream_from_iter(1..=20);

        // filter keeps the items the closure approves of. take stops the stream after the given number of items.
        let mut evens = stream.filter(|n| n % 2 == 0).take(3);

        while let Some(value) = evens.next().await {
            println!("even: {value}");
        }
    });

    // Try: change take(3) to take(5). What is the last line of the output?
}
