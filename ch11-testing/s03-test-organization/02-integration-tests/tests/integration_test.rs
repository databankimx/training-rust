// Each file in tests is its own crate, so it brings the library into scope with use.
// It can only call the public API, just as code outside the library would.
use adder::add_two;

#[test]
fn it_adds_two() {
    assert_eq!(4, add_two(2));
}
