struct Config {
    port: Option<u16>,
}

// Returns a message for the port the program should listen on.
// The early return for a missing port keeps the main path flat, using let...else.
fn describe(config: &Config) -> String {
    let Some(port) = config.port else {
        return String::from("no port configured, using 8080");
    };

    format!("listening on port {port}")
}

fn main() {
    let configured = Config { port: Some(3000) };
    let unconfigured = Config { port: None };

    println!("{}", describe(&configured));
    println!("{}", describe(&unconfigured));
}
