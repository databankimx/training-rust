use std::time::Duration;

// Stands in for fetching a page. The delay plays the part of the network, so the result is the same on every run.
async fn page_title(url: &str, delay_ms: u64) -> (String, Option<String>) {
    trpl::sleep(Duration::from_millis(delay_ms)).await;
    (url.to_string(), Some(format!("Title of {url}")))
}

fn main() {
    trpl::block_on(async {
        // Creating the futures does nothing yet. select polls both, and returns the first one to finish.
        let fut_1 = page_title("fast-site", 100);
        let fut_2 = page_title("slow-site", 300);

        let (url, title) = match trpl::select(fut_1, fut_2).await {
            trpl::Either::Left(left) => left,
            trpl::Either::Right(right) => right,
        };

        println!("{url} finished first");
        if let Some(title) = title {
            println!("Its title is: {title}");
        }

        // Try: swap the delays, so the fast site takes 500 ms. Which future wins now?
    });
}
