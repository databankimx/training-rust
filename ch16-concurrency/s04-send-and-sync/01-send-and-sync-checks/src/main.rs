use std::cell::RefCell;
use std::sync::{Arc, Mutex};

// Each function compiles only for a type that has the trait it names. The check happens when the program is compiled, and costs nothing at runtime.
fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}

fn main() {
    assert_send::<Arc<Mutex<i32>>>();
    assert_sync::<Arc<Mutex<i32>>>();
    assert_send::<Mutex<i32>>();
    assert_sync::<Mutex<i32>>();
    assert_send::<RefCell<i32>>();

    println!("Arc<Mutex<i32>> is Send and Sync");
    println!("Mutex<i32> is Send and Sync");
    println!("RefCell<i32> is Send");

    // Try: uncomment the next line. Why does it stop the program from compiling?
    // assert_sync::<RefCell<i32>>();
}
