mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Added to waitlist");
        }
    }
}

pub fn eat_at_restaurant() {
    // Absolute path: starts from the crate root.
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path: starts from the current module, which is the crate root here.
    front_of_house::hosting::add_to_waitlist();
}

fn main() {
    eat_at_restaurant();

    // Try: change the absolute path to start with self:: instead of crate::. Does it still work from here?
}
