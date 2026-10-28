use type_inference_with_trait_objects::{Button, Draw, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!(
            "Drawing a {}x{} select box with {} options",
            self.width,
            self.height,
            self.options.len()
        );
    }
}

fn main() {
    // Without the annotation on components, the compiler infers the vector's type from its first element, and rejects the Button.
    // The annotation tells it that the vector holds trait objects.
    let components: Vec<Box<dyn Draw>> = vec![
        Box::new(SelectBox {
            width: 75,
            height: 10,
            options: vec![
                String::from("Yes"),
                String::from("Maybe"),
                String::from("No"),
            ],
        }),
        Box::new(Button {
            width: 50,
            height: 10,
            label: String::from("OK"),
        }),
    ];

    let screen = Screen { components };
    screen.run();

    // Try: remove the type annotation on components. What does the compiler say?
}
