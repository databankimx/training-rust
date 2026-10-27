fn main() {
    // cfg!(debug_assertions) is decided when the program is compiled, so checking it costs nothing when the program runs.
    if cfg!(debug_assertions) {
        println!("This is a debug build, made with the dev profile");
    } else {
        println!("This is a release build, made with the release profile");
    }

    // Try: run cargo build --release in a terminal from this folder, then run the program. What does it print now?
}
