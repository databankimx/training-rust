// Nested paths bring several items with a shared prefix into scope in one line.
use std::collections::{BTreeSet, HashMap};

fn main() {
    let mut scores: HashMap<&str, u32> = HashMap::new();
    scores.insert("Ferris", 10);

    let names: BTreeSet<&str> = ["Ferris", "Corro"].into_iter().collect();

    println!("{:?}", scores.get("Ferris"));
    println!("{:?}", names);

    // Try: add `use std::collections::*;` at the top. Does anything change? Why is the glob operator risky?
}
