use std::thread;

// Splits the values into chunks, sums each chunk on its own thread, and adds the partial sums.
fn parallel_sum(values: Vec<u64>, threads: usize) -> u64 {
    // Use at least one thread, so the chunk size calculation below can't divide by zero or underflow.
    let threads = threads.max(1);
    // Round the chunk size up, so the number of chunks is no more than threads. At least 1, so chunks never gets 0.
    let chunk_size = (values.len() + threads - 1) / threads;
    let chunk_size = chunk_size.max(1);

    // Each thread owns its chunk, so the chunk is copied into a Vec before the thread starts.
    let mut handles = Vec::new();
    for chunk in values.chunks(chunk_size) {
        let owned = chunk.to_vec();
        handles.push(thread::spawn(move || owned.iter().sum::<u64>()));
    }

    // Spawn every thread first, then join them. Joining inside the spawning loop would run them one after another.
    handles.into_iter().map(|handle| handle.join().unwrap()).sum()
}

fn main() {
    let values: Vec<u64> = (1..=1000).collect();
    println!("sum: {}", parallel_sum(values, 4));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_a_plain_sum() {
        let values: Vec<u64> = (1..=10).collect();
        assert_eq!(parallel_sum(values, 3), 55);
    }

    #[test]
    fn works_with_more_threads_than_values() {
        let values: Vec<u64> = vec![4, 5];
        assert_eq!(parallel_sum(values, 8), 9);
    }

    #[test]
    fn empty_list_sums_to_zero() {
        assert_eq!(parallel_sum(Vec::new(), 4), 0);
    }
}
