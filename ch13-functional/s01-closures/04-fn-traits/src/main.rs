// height is only printed through Debug, which the compiler doesn't count as reading it, so the allow attribute silences the warning.
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    // sort_by_key calls its closure once for each item, so the closure needs FnMut at least.
    // This one only reads the width, so it's also Fn.
    list.sort_by_key(|r| r.width);

    println!("{list:?}");

    // Try: change the key to r.height. What is the new order?
}
