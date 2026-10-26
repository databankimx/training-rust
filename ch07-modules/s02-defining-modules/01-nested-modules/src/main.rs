// front_of_house holds two nested modules. Each module is private to its parent unless marked pub.
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Added a party to the waitlist");
        }
    }

    pub mod serving {
        pub fn take_order() {
            println!("Took an order");
        }
    }
}

fn main() {
    front_of_house::hosting::add_to_waitlist();
    front_of_house::serving::take_order();

    // Try: remove pub from `mod hosting`. Which line does the compiler reject, and what does it say?
}
