use adder::add_two;

// Brings the shared helpers in, as a module of this test crate.
mod common;

#[test]
fn it_adds_two() {
    assert_eq!(common::setup(), "setup done");
    assert_eq!(4, add_two(2));
}

// Try: move setup into a file called tests/common.rs instead of tests/common/mod.rs. What appears in the output?
