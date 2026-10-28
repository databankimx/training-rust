struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    // An underscore prefix stops the unused-variable warning. _d still lives until the end of main.
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    // std::mem::drop takes the value by moving it in, so the value is dropped right away.
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");

    // Try: call c.drop() instead of drop(c). What does the compiler say?
}
