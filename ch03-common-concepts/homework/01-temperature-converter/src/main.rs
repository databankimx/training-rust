use std::io;

fn main() {
    println!("Temperature converter");

    let value = read_temperature();
    let scale = read_scale();

    if scale == 'F' {
        println!("{value:.1}°F = {:.1}°C", fahrenheit_to_celsius(value));
    } else {
        println!("{value:.1}°C = {:.1}°F", celsius_to_fahrenheit(value));
    }
}

fn read_temperature() -> f64 {
    loop {
        let text = read_line("Enter the temperature:");
        match text.trim().parse::<f64>() {
            Ok(value) => return value,
            Err(_) => println!("That isn't a number. Try again."),
        }
    }
}

fn read_scale() -> char {
    loop {
        let text = read_line("Is it in F or C?");
        match text.trim().to_uppercase().as_str() {
            "F" => return 'F',
            "C" => return 'C',
            _ => println!("Please type F or C."),
        }
    }
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

fn read_line(prompt: &str) -> String {
    println!("{prompt}");
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    line
}
