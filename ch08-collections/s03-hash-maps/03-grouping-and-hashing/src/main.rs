use std::collections::HashMap;

fn main() {
    // Map each character to the list of positions where it appears.
    let mut positions: HashMap<char, Vec<usize>> = HashMap::new();
    for (i, c) in "hello!".chars().enumerate() {
        positions.entry(c).or_insert(Vec::new()).push(i);
    }

    // Add up the positions of 'l', which appears at indexes 2 and 3.
    let mut sum = 0;
    for i in positions.get(&'l').unwrap() {
        sum += *i;
    }
    println!("{}", sum);

    // Try: change 'l' to 'z'. What does unwrap do now, and why does the program panic?
}
