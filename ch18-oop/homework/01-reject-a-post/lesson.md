## Homework: Rejecting a Post

**The problem:** The blog post workflow from section 18.3 has no way back. A post waiting for review should be able to be rejected, which returns it to the draft state. Add that transition to the
trait-object version of the state pattern.

Create a project in VS Code with `cargo new blog_reject`, and copy in the `State` trait and the structs from section 18.3's first step. Then:

* Add `fn reject(self: Box<Self>) -> Box<dyn State>;` to the `State` trait. Every state must implement it.
* `Draft` and `Published` return themselves, because a draft has nothing to reject, and a published post stays published.
* `PendingReview` returns a new `Draft`.
* Add a `reject` method to `Post`, which takes the state out, rejects it, and puts the result back, in the same way as `request_review` and `approve`.

Write tests in a `tests` module. They should check that a post returns to draft after a rejection, that rejecting a draft does nothing, and that a published post can't be rejected back to draft.
The tests can check the content, since only a published post returns text.

---

### Hints

* `PendingReview::reject` should return `Box::new(Draft {})`. Think about which state the post should be in afterwards.
* A published post can't go back to draft, so its `reject` returns `self`, as its `approve` does.
* The test for the draft-after-reject case can request a review, reject, approve, and check that the content is still empty. Approving a draft has no effect, so the text won't appear.

---

### Check your work

With the sequence `add_text`, `request_review`, `reject`, `approve`, the content should still be empty. After a second `request_review` and `approve`, the content should be the text.

---

### Extensions

* Require two approvals before publishing. Where does the counter live, in `Post` or in a state?
* Think about the enum version from section 18.3. How many places would change to add `reject` to it?

---

*This exercise is not from the book. It practices the state pattern from Chapter 18.*
