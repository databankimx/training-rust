// The mod declaration tells the compiler to load src/front_of_house.rs. It is not an include.
mod front_of_house;

fn main() {
    front_of_house::hosting::add_to_waitlist();

    // Try: change the message in src/front_of_house/hosting.rs, in your own editor, and run again. What prints now?
}
