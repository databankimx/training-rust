fn main() {
    if cfg!(debug_assertions) {
        println!("This is a debug build, made with the dev profile");
    } else {
        println!("This is a release build, made with the release profile");
    }

    // Try: remove the [profile.dev] section from Cargo.toml, and run again. Which message appears?
}
