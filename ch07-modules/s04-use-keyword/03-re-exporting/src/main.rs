mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Added to waitlist");
        }
    }
}

// pub use makes hosting available at the crate root, for any code that can reach the root.
pub use front_of_house::hosting;

mod customer {
    // The customer module reaches hosting through the re-exported name, not the full path.
    pub fn visit() {
        crate::hosting::add_to_waitlist();
    }
}

fn main() {
    customer::visit();

    // Try: change `pub use` to `use`. Does the program still compile? What changes for a library crate?
}
