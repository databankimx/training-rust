use std::sync::mpsc;
use std::thread;

// Sends each number through a producer and a worker thread, and collects the squares in order.
fn square_through_threads(numbers: Vec<u64>) -> Vec<u64> {
    let (numbers_tx, numbers_rx) = mpsc::channel::<u64>();
    let (squares_tx, squares_rx) = mpsc::channel::<u64>();

    // The producer owns numbers_tx. When its closure ends, the transmitter is dropped, and the worker's loop finishes.
    let producer = thread::spawn(move || {
        for n in numbers {
            numbers_tx.send(n).unwrap();
        }
    });

    // The worker owns squares_tx. When numbers_rx ends, its loop finishes, and squares_tx is dropped.
    let worker = thread::spawn(move || {
        for n in numbers_rx {
            squares_tx.send(n * n).unwrap();
        }
    });

    // Collecting ends when squares_tx is dropped, which happens after the worker has finished.
    let squares: Vec<u64> = squares_rx.iter().collect();

    producer.join().unwrap();
    worker.join().unwrap();
    squares
}

fn main() {
    let squares = square_through_threads((1..=5).collect());
    println!("squares: {squares:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_list_gives_empty_list() {
        assert_eq!(square_through_threads(Vec::new()), Vec::<u64>::new());
    }

    #[test]
    fn single_number() {
        assert_eq!(square_through_threads(vec![3]), vec![9]);
    }

    #[test]
    fn short_list_keeps_its_order() {
        assert_eq!(square_through_threads(vec![2, 3, 4]), vec![4, 9, 16]);
    }
}
