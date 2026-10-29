mod back_of_house {
    pub struct Breakfast {
        // The customer chooses the toast, so the field is public.
        pub toast: String,
        // The chef chooses the fruit, so the field stays private.
        seasonal_fruit: String,
    }

    impl Breakfast {
        // A struct with a private field needs a public constructor, because callers can't set that field.
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }

        // A public method reads the private field, so callers can still see it.
        pub fn fruit(&self) -> &str {
            &self.seasonal_fruit
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");

    println!("I'd like {} toast please", meal.toast);
    println!("Today's fruit is {}", meal.fruit());

    // Try: uncomment a line that sets meal.seasonal_fruit directly. What does the compiler say?
}

fn main() {
    eat_at_restaurant();
}
