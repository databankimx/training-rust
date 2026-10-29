use std::cell::RefCell;
use std::rc::{Rc, Weak};

// A parent owns its children through Rc. A child refers back to its parent through Weak, which doesn't keep the parent alive.
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        // Give leaf a weak reference to branch. Weak doesn't raise the strong count, so there's no cycle.
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("branch strong = {}, weak = {}", Rc::strong_count(&branch), Rc::weak_count(&branch));
        println!("branch children = {}", branch.children.borrow().len());
        println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
        println!("leaf parent value = {:?}", leaf.parent.borrow().upgrade().map(|p| p.value));
    }

    // branch has gone out of scope, so its value has been dropped, and upgrade returns None.
    println!("leaf parent value after scope = {:?}", leaf.parent.borrow().upgrade().map(|p| p.value));
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));

    // Try: change the value of branch from 5 to 50, and predict the parent value printed inside the scope.
}
