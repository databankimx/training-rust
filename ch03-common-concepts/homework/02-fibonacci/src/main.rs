use std::io;

fn main() {
    let n = read_number("Which Fibonacci number do you want? Enter a whole number:");

    match fibonacci(n) {
        Some(f) => println!("The Fibonacci number at position {n} is {f}."),
        None => println!("That number is too big to fit in a u64."),
    }
}

fn fibonacci(n: u32) -> Option<u64> {
    if n == 0 {
        return Some(0);
    }

    let mut previous: u64 = 0;
    let mut current: u64 = 1;

    for _ in 1..n {
        let next = previous.checked_add(current)?;
        previous = current;
        current = next;
    }

    Some(current)
}

fn read_number(prompt: &str) -> u32 {
    loop {
        println!("{prompt}");
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        match line.trim().parse::<u32>() {
            Ok(n) => return n,
            Err(_) => println!("Please enter a whole number, zero or more."),
        }
    }
}
