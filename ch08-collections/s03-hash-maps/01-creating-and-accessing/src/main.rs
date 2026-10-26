use std::collections::HashMap;

fn main() {
    // HashMap is not in the prelude, so it has to be brought into scope.
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // get returns Option<&i32>. copied turns it into Option<i32>, and unwrap_or supplies zero when the key is missing.
    let team = String::from("Blue");
    let score = scores.get(&team).copied().unwrap_or(0);
    println!("Blue: {score}");

    let missing = scores.get("Green").copied().unwrap_or(0);
    println!("Green: {missing}");

    // Try: remove the unwrap_or(0) and print the Option instead. What is printed for Green?
}
