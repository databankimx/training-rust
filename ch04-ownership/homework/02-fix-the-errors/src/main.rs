fn main() {
    let mut scores = vec![10, 20, 30];

    // Copy the best score out before the push, so no borrow of scores is still in use.
    let best = scores[2];
    scores.push(40);

    println!("best: {best}");
    println!("scores: {:?}", scores);
}
