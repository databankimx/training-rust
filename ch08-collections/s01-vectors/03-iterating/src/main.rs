fn main() {
    let mut v = vec![100, 32, 57];

    // Iterating over &v gives immutable references to each element.
    for i in &v {
        println!("{i}");
    }

    // Iterating over &mut v gives mutable references. The * reads and writes the value behind each one.
    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{i}");
    }

    // Try: put v.push(1) inside the second for loop. What does the compiler say about v?
}
