use std::ops::Deref;

// A wrapper that reports when it's created and dropped. Deref lets it be used like the value it holds.
struct Logged<T> {
    value: T,
    name: &'static str,
}

impl<T> Logged<T> {
    fn new(name: &'static str, value: T) -> Logged<T> {
        println!("created {}", name);
        Logged { value, name }
    }
}

impl<T> Deref for Logged<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}

impl<T> Drop for Logged<T> {
    fn drop(&mut self) {
        println!("dropped {}", self.name);
    }
}

fn main() {
    let first = Logged::new("first", String::from("hello"));
    let second = Logged::new("second", vec![1, 2, 3]);

    // len is found through Deref, so there's no need to write .value.
    println!("first has {} bytes", first.len());
    println!("second has {} items", second.len());

    // Values are dropped in reverse order of creation, when main ends.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn behaves_like_its_inner_value() {
        let number = Logged::new("number", 7);
        assert_eq!(*number, 7);
    }
}
