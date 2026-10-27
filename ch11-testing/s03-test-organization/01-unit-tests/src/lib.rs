pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

// A private function. Only code in this crate can call it, but its unit test can.
fn internal_adder(left: i32, right: i32) -> i32 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }

    // Try: add a test that checks add_two(0) is 2. Does it need to be inside this module?
}
