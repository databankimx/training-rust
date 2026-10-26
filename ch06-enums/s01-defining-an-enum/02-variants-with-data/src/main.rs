// The data in these variants is printed by Debug, but the compiler doesn't count that as reading it.
// The allow attribute silences the warning. Chapter 6.2 shows how to read the data with match.
#[allow(dead_code)]
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    // Each variant holds its own data. A version four address is four numbers, and a version six address is a string.
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("home: {home:?}");
    println!("loopback: {loopback:?}");

    // Try: change home to IpAddr::V4(127, 0, 0). Which line does Rust reject, and why?
}
