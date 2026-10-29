use std::cmp::Reverse;

// The closure owns count. move gives it ownership, so the counter keeps its value between calls.
fn make_counter() -> impl FnMut() -> u32 {
    let mut count = 0;
    move || {
        count += 1;
        count
    }
}

// F: Fn means the closure only reads its environment, so it can be called twice.
fn apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(f(x))
}

// sort_by_key is stable, and Reverse flips the order, so the longest words come first.
fn sort_by_length(words: &mut Vec<&str>) {
    words.sort_by_key(|word| Reverse(word.len()));
}

fn main() {
    let mut counter = make_counter();
    let counts: Vec<String> = (0..3).map(|_| counter().to_string()).collect();
    println!("counter: {}", counts.join(" "));

    println!("apply_twice: {}", apply_twice(|x| x * 3, 2));

    let mut words = vec!["fox", "quick", "brown", "jumps"];
    sort_by_length(&mut words);
    println!("sorted: {words:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_counts_up() {
        let mut c = make_counter();
        assert_eq!(c(), 1);
        assert_eq!(c(), 2);
        assert_eq!(c(), 3);
    }

    #[test]
    fn counters_are_independent() {
        let mut a = make_counter();
        let mut b = make_counter();
        a();
        a();
        assert_eq!(b(), 1);
    }

    #[test]
    fn apply_twice_applies_f_twice() {
        assert_eq!(apply_twice(|x| x + 5, 1), 11);
    }

    #[test]
    fn sorts_longest_first() {
        let mut words = vec!["a", "abc", "ab"];
        sort_by_length(&mut words);
        assert_eq!(words, vec!["abc", "ab", "a"]);
    }
}
