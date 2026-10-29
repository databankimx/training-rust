struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// Iterator has an associated type, Item. Counter fixes it once, as u32, so callers never need to write the type.
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let all: Vec<u32> = Counter::new().collect();
    println!("all counted: {all:?}");

    // zip pairs each number with the next one, and the products that divide by 3 are summed.
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    println!("sum: {sum}");

    // Try: change the limit in next from 5 to 6. What does the sum become?
}
