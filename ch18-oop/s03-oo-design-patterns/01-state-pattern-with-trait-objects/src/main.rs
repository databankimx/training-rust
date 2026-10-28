use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    println!("draft content: '{}'", post.content());
    assert_eq!("", post.content());

    post.request_review();
    println!("pending review content: '{}'", post.content());
    assert_eq!("", post.content());

    post.approve();
    println!("published content: '{}'", post.content());
    assert_eq!("I ate a salad for lunch today", post.content());

    // Try: add post.approve(); directly after add_text, before request_review. Does the content change at that point?
}
