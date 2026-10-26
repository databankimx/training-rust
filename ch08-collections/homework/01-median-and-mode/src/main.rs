use std::collections::HashMap;

// The median is the middle value of the sorted list. With an even count, it's the average of the two middle values.
fn median(values: &[i32]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    let mut sorted = values.to_vec();
    sorted.sort();
    let mid = sorted.len() / 2;
    if sorted.len() % 2 == 0 {
        Some((sorted[mid - 1] + sorted[mid]) as f64 / 2.0)
    } else {
        Some(sorted[mid] as f64)
    }
}

// The mode is the value that occurs most often. Among ties, this returns the smallest value, so the answer is repeatable.
fn mode(values: &[i32]) -> Option<i32> {
    let mut counts: HashMap<i32, usize> = HashMap::new();
    for &v in values {
        *counts.entry(v).or_insert(0) += 1;
    }
    counts
        .into_iter()
        .max_by_key(|&(value, count)| (count, std::cmp::Reverse(value)))
        .map(|(value, _)| value)
}

fn main() {
    let values = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    println!("median: {}", median(&values).unwrap());
    println!("mode: {}", mode(&values).unwrap());
}
