mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Added to waitlist");
        }
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // hosting is now a shortcut for crate::front_of_house::hosting, in this scope.
    hosting::add_to_waitlist();
}

fn main() {
    eat_at_restaurant();

    // Try: remove the use line, and write crate::front_of_house::hosting::add_to_waitlist() in eat_at_restaurant. Same output?
}
