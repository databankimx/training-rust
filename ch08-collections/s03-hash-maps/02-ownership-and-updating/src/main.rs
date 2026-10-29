use std::collections::HashMap;

fn main() {
    let text = "hello world wonderful world";

    // entry finds the key, or inserts the default. or_insert returns a mutable reference to the value.
    let mut counts: HashMap<&str, i32> = HashMap::new();
    for word in text.split_whitespace() {
        let count = counts.entry(word).or_insert(0);
        *count += 1;
    }

    // Printing single keys avoids depending on the order of a hash map's entries.
    println!("world: {}", counts["world"]);
    println!("hello: {}", counts["hello"]);
    println!("wonderful: {}", counts["wonderful"]);

    // Try: insert "hello" with the value 25, and print it again. What happens to the old count?
}
