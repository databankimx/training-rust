// An infinite iterator over the Fibonacci sequence: 0, 1, 1, 2, 3, 5, and so on.
struct Fibonacci {
    current: u64,
    next: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { current: 0, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        // Work out the value after the next one before moving the fields on, so no number is lost.
        let result = self.current;
        let following = self.current + self.next;
        self.current = self.next;
        self.next = following;
        Some(result)
    }
}

fn main() {
    let first_ten: Vec<u64> = Fibonacci::new().take(10).collect();
    println!("first ten: {first_ten:?}");

    // take_while stops at the first value that fails the test, so the sequence never runs on past the limit.
    let even_sum: u64 = Fibonacci::new()
        .take_while(|&x| x < 4_000_000)
        .filter(|x| x % 2 == 0)
        .sum();
    println!("even sum below 4000000: {even_sum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_six_values() {
        let values: Vec<u64> = Fibonacci::new().take(6).collect();
        assert_eq!(values, vec![0, 1, 1, 2, 3, 5]);
    }

    #[test]
    fn even_values_below_100_sum_to_44() {
        let even_sum: u64 = Fibonacci::new()
            .take_while(|&x| x < 100)
            .filter(|x| x % 2 == 0)
            .sum();
        assert_eq!(even_sum, 44);
    }
}
