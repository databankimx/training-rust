// A Document is a list of words. Rust makes the ownership explicit in each function signature.
type Document = Vec<String>;

fn new_document(words: Vec<String>) -> Document {
    words
}

fn add_word(this: &mut Document, word: String) {
    this.push(word);
}

fn get_words(this: &Document) -> &[String] {
    this.as_slice()
}

fn main() {
    let words = vec!["hello".to_string()];
    let d = new_document(words);

    // to_vec() clones each word, so d2 gets its own copy of the list.
    let words_copy = get_words(&d).to_vec();
    let mut d2 = new_document(words_copy);
    add_word(&mut d2, "world".to_string());

    println!("d: {:?}", d);
    println!("d2: {:?}", d2);

    // Try: remove the to_vec() and pass get_words(&d) straight to new_document. What does Rust say?
}
