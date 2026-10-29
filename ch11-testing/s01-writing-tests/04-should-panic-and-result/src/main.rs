pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {value}.");
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {value}.");
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("guess: {}", Guess::new(50).value());
}

#[cfg(test)]
mod tests {
    use super::*;

    // The test passes only if the code panics, and the message contains the expected text.
    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    // A test can return a Result, and use ? to fail on any error.
    #[test]
    fn parses_a_number() -> Result<(), std::num::ParseIntError> {
        let n: i32 = "5".parse()?;
        assert_eq!(n, 5);
        Ok(())
    }

    // Try: change the expected text to "greater than 100". Does the test still pass, and why not?
}
