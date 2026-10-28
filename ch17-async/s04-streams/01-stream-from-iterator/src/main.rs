// StreamExt provides next, and the other utility methods, for streams. Without this import, next isn't found.
use trpl::StreamExt;

fn main() {
    trpl::block_on(async {
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|n| n * 2);
        // stream_from_iter turns an iterator into a stream, so its items can be awaited.
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(value) = stream.next().await {
            println!("The value was: {value}");
        }
    });

    // Try: change the map to |n| n + 100. Does the output change in any other way than the values?
}
