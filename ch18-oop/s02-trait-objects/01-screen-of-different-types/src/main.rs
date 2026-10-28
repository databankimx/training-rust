use screen_of_different_types::{Button, Draw, Screen};

// SelectBox is defined here, outside the library. It works with Screen because it implements Draw.
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
    // Box<dyn Draw> lets one vector hold a SelectBox and a Button. Each is called through the same draw method.
    let screen = Screen {
        components: vec![
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
        ],
    };

    screen.run();

    // Try: add a String to the components vector, like Box::new(String::from("hi")). What does the compiler say?
}
