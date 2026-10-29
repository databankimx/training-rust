trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

// A method written directly on Human. Plain method calls prefer it over the trait methods.
impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    let person = Human;

    // Plain method syntax picks the method on Human directly.
    person.fly();

    // Naming the trait picks that trait's fly method.
    Pilot::fly(&person);
    Wizard::fly(&person);

    // Dog::baby_name calls the one written on Dog. The Animal version needs fully qualified syntax.
    println!("A baby dog is called a {}", Dog::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    // Try: delete the inherent fly method on Human. What does person.fly() do now, and why does the compiler complain?
}
