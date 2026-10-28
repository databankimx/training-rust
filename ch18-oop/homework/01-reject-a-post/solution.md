## Solution: Rejecting a Post

This walkthrough explains the example solution in `src/main.rs`. Read it after you've written your own version.

### Run the solution

Press **Run** for the output, and **Test** for the three tests. The output is:

```
after reject and approve: ''
after second review: 'A rough idea'
```

### How it works

`reject` is added to the `State` trait, so every state must say what rejection does. Each state's answer follows from the workflow. A draft has nothing to reject, and a published post stays
published. A post waiting for review goes back to draft, which is the one transition the method adds.

`Post::reject` follows the same pattern as `request_review` and `approve`. It takes the state out of the `Option`, transforms it, and puts the result back. The `take` is what lets the state
be consumed, as section 18.3 explained.

The walkthrough in `main` runs the workflow twice. After a rejection, approving a draft has no effect, so the text stays hidden. A second review and approval publish the post as normal.

The tests check each case that matters. A rejected post returns to draft. Rejecting a draft changes nothing, so the later review and approval still publish it. A published post can't be pulled
back, so its text remains visible.

### The extensions

Requiring two approvals needs a counter. It could live in `Post`, which would mean `Post` knows about approvals. Or the state could count them: `PendingReview` could hold a number, and return
`Published` only when it reaches the limit. Putting the counter in the state keeps the rule with the state it applies to.

With the enum version, `reject` would be a new `match` arm in every method that handles states, not just one new method. For a small workflow that's a little more code. With traits, the new state
behavior sits in one place, although `reject` still has to be added to each existing state, as it was here.

---

*This walkthrough accompanies the exercise in Chapter 18 of the course. It's not from the book.*
