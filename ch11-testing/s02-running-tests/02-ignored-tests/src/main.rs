fn main() {
    println!("program ran");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // Imagine this test takes many minutes. The ignore attribute leaves it out of ordinary runs.
    #[test]
    #[ignore]
    fn expensive_test() {
        assert_eq!(1 + 1, 2);
    }

    // Try: remove #[ignore] from expensive_test, and run Test again. How does the summary change?
}
