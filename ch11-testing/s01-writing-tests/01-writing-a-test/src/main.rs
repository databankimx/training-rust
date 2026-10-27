pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

fn main() {
    println!("2 + 2 = {}", add(2, 2));
}

#[cfg(test)]
mod tests {
    // The tests module is compiled only for cargo test, so the program itself doesn't include it.
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // Try: change the expected value to 5, and run the tests again. What does the failure message show?
}
