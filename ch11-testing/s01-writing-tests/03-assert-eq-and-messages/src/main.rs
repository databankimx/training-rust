pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

fn main() {
    println!("{}", add_two(2));
    println!("{}", greeting("Carol"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        // assert_eq! prints both values if they differ, so the failure shows what was produced.
        assert_eq!(add_two(2), 4);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // The greeting text may change, so the test checks only that the name appears.
        assert!(
            result.contains("Carol"),
            "Greeting did not contain the name, the value was `{result}`"
        );
    }

    // Try: change add_two to add 3, and run the tests. What do the left and right values show?
}
