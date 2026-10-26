mod garden {
    pub mod vegetables {
        // An implementation detail: only plant uses it.
        fn fertilizer() -> &'static str {
            "compost"
        }

        pub fn plant(name: &str) -> String {
            format!("planted {name} with {}", fertilizer())
        }
    }

    pub fn season() -> &'static str {
        "summer"
    }

    pub fn harvest() -> String {
        format!("harvested in {}", season())
    }
}

fn main() {
    println!("{}", garden::vegetables::plant("beans"));
    println!("{}", garden::harvest());
    println!("It is {}", garden::season());
}
