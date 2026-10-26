fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    // + takes ownership of s1, and borrows s2 as a &str. s1 can't be used after this line.
    let s3 = s1 + &s2;
    println!("{s3}");

    // s2 was only borrowed, so it is still usable.
    println!("{s2}");

    // format! works like println!, but returns a String, and borrows all of its arguments.
    let s = format!("{}-{}-{}", "tic", "tac", "toe");
    println!("{s}");

    // Try: use s1 after the + line. What does the compiler say?
}
