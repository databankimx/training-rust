fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {a}");
    10
}

fn main() {
    println!("{}", prints_and_returns_10(4));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }

    // Try: make this_test_will_pass fail as well. Does its printed line appear in the output?
}
