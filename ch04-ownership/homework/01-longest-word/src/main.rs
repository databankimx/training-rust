fn longest_word(sentence: &str) -> &str {
    let mut longest = "";
    for word in sentence.split_whitespace() {
        if word.len() > longest.len() {
            longest = word;
        }
    }
    longest
}

fn main() {
    let sentence = String::from("the quick brown fox jumped");
    let word = longest_word(&sentence);
    println!("{word}");
}
