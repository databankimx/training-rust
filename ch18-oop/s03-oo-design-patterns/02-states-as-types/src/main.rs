use states_as_types::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    // Each call returns the next state, and the old one is consumed.
    let post = post.request_review();
    let post = post.approve();

    println!("published content: '{}'", post.content());

    // Try: uncomment the line below. What does the compiler say about a DraftPost?
    // println!("{}", Post::new().content());
}
