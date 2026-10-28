use std::time::Duration;

// Stands in for a network request. The delay differs for each city, so the replies arrive out of order.
async fn fetch_temperature(city: &str, delay_ms: u64, degrees_f: i32) -> (String, i32) {
    trpl::sleep(Duration::from_millis(delay_ms)).await;
    (city.to_string(), degrees_f)
}

// join_all runs the futures together, and returns the results in the order the futures were given.
async fn gather_temperatures(requests: Vec<(&str, u64, i32)>) -> Vec<(String, i32)> {
    let futures = requests
        .into_iter()
        .map(|(city, delay, temp)| fetch_temperature(city, delay, temp));
    trpl::join_all(futures).await
}

fn main() {
    let requests = vec![
        ("Dallas", 300, 88),
        ("Boston", 100, 61),
        ("Seattle", 200, 57),
    ];

    for (city, temp) in trpl::block_on(gather_temperatures(requests)) {
        println!("{city}: {temp}F");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn results_keep_the_order_of_the_requests() {
        let requests = vec![("Dallas", 300, 88), ("Boston", 100, 61)];
        let results = trpl::block_on(gather_temperatures(requests));
        assert_eq!(
            results,
            vec![("Dallas".to_string(), 88), ("Boston".to_string(), 61)]
        );
    }

    #[test]
    fn no_requests_gives_no_results() {
        assert!(trpl::block_on(gather_temperatures(Vec::new())).is_empty());
    }
}
