fn read(y: bool) {
    if y {
        println!("y is true!");
    }
}

fn main() {
    let x = true;
    read(x);

    // Try: uncomment the next two lines and run the program. Rust rejects it, because `z`
    // is used before it is defined. What would happen at runtime if this were allowed?
    // read(z);
    // let z = true;
}
