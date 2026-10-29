use word_stats::{average_word_length, longest_word, word_count};

fn main() {
    let text = "the quick brown fox";
    println!("words: {}", word_count(text));
    println!("average length: {:.2}", average_word_length(text));
    println!("longest: {:?}", longest_word(text));
}
