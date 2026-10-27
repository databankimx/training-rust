use std::fmt::{self, Display};

// A stack that works for any element type T.
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    fn len(&self) -> usize {
        self.items.len()
    }
}

// Printing the stack needs each item to be printable, so the impl has the bound T: Display.
impl<T: Display> Display for Stack<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for (i, item) in self.items.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{item}")?;
        }
        write!(f, "]")
    }
}

fn main() {
    let mut s = Stack::new();
    s.push(1);
    s.push(2);
    s.push(3);

    println!("stack: {s}");
    println!("top: {:?}", s.peek());
    println!("popped: {:?}", s.pop());
    println!("stack: {s}, length {}", s.len());
}
