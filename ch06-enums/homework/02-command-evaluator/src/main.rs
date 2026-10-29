#[derive(Debug)]
enum Command {
    Add(i32, i32),
    Negate(i32),
    Quit,
}

// Returns Some with the result for a command that produces one, and None when the program should stop.
fn run(cmd: &Command) -> Option<i32> {
    match cmd {
        Command::Add(a, b) => Some(a + b),
        Command::Negate(n) => Some(-n),
        Command::Quit => None,
    }
}

fn main() {
    let commands = [Command::Add(2, 3), Command::Negate(7), Command::Quit];

    for cmd in &commands {
        match run(cmd) {
            Some(value) => println!("result: {value}"),
            None => println!("stopped"),
        }
    }
}
