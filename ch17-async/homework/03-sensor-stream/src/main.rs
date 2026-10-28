use trpl::StreamExt;

// Keeps the first four readings from 0 to 100, prints each one, and returns their total.
async fn total_of_valid(readings: Vec<i32>) -> i32 {
    let mut valid = trpl::stream_from_iter(readings)
        .filter(|reading| (0..=100).contains(reading))
        .take(4);

    let mut total = 0;
    while let Some(reading) = valid.next().await {
        println!("accepted {reading}");
        total += reading;
    }
    total
}

fn main() {
    let total = trpl::block_on(total_of_valid(vec![12, -1, 15, 300, 14, 13, -5, 16]));
    println!("total: {total}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn totals_the_first_four_valid_readings() {
        let total = trpl::block_on(total_of_valid(vec![12, -1, 15, 300, 14, 13, -5, 16]));
        assert_eq!(total, 54);
    }

    #[test]
    fn no_valid_readings_gives_zero() {
        let total = trpl::block_on(total_of_valid(vec![-1, 200]));
        assert_eq!(total, 0);
    }
}
