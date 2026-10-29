use art::{PrimaryColor, mix};

fn main() {
    // PrimaryColor and mix are imported from the top level of the art crate, not from its kinds and utils modules.
    let color = mix(PrimaryColor::Red, PrimaryColor::Yellow);
    println!("Red and yellow make {color:?}");

    // Try: import art::kinds::PrimaryColor instead, and use the long path. Does the program still compile?
}
