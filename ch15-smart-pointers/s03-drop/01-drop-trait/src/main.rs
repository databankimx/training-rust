struct CustomSmartPointer {
    data: String,
}

// drop runs automatically when a CustomSmartPointer goes out of scope. This one prints, so the timing is visible.
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    // The underscore marks these as unused, without ending their lives early. Each value still lives until the end of main.
    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    // Try: swap the order of the two declarations, so _d is created first. Which value is dropped first now?
}
