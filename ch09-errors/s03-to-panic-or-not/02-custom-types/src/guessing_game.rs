// The Guess type can only be built through new, so every Guess is known to be between 1 and 100.
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("The guess value must be between 1 and 100, got {value}.");
        }
        Guess { value }
    }

    // A getter. The field is private, so callers read the value through this method.
    pub fn value(&self) -> i32 {
        self.value
    }
}
