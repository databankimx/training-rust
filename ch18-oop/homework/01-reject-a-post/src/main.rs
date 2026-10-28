pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;

    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // A draft has nothing to reject, so it stays as it is.
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }

    // A rejected review goes back to draft, so the author can revise the post.
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // A published post can't be pulled back to draft by this workflow.
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

fn main() {
    let mut post = Post::new();
    post.add_text("A rough idea");
    post.request_review();
    post.reject();
    post.approve();
    println!("after reject and approve: '{}'", post.content());

    post.request_review();
    post.approve();
    println!("after second review: '{}'", post.content());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejected_post_returns_to_draft() {
        let mut post = Post::new();
        post.add_text("text");
        post.request_review();
        post.reject();
        post.approve();
        assert_eq!(post.content(), "");
    }

    #[test]
    fn rejecting_a_draft_does_nothing() {
        let mut post = Post::new();
        post.add_text("text");
        post.reject();
        post.request_review();
        post.approve();
        assert_eq!(post.content(), "text");
    }

    #[test]
    fn published_post_cannot_be_rejected() {
        let mut post = Post::new();
        post.add_text("text");
        post.request_review();
        post.approve();
        post.reject();
        assert_eq!(post.content(), "text");
    }
}
