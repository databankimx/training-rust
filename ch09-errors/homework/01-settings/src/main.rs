use std::collections::HashMap;

// Splits a "key=value" line into its parts. A line without "=", or with an empty key, is an error.
fn parse_setting(line: &str) -> Result<(String, String), String> {
    let (key, value) = line
        .split_once('=')
        .ok_or_else(|| format!("no '=' in {line:?}"))?;
    let key = key.trim();
    if key.is_empty() {
        return Err(format!("empty key in {line:?}"));
    }
    Ok((key.to_string(), value.trim().to_string()))
}

fn main() {
    let lines = ["port = 8080", "host=localhost", "retries", " = 3"];

    let mut settings: HashMap<String, String> = HashMap::new();
    for line in lines {
        match parse_setting(line) {
            Ok((key, value)) => {
                println!("{key} -> {value}");
                settings.insert(key, value);
            }
            Err(error) => println!("error: {error}"),
        }
    }

    // The port is required, so a missing or non-numeric port is a bug in the settings file, and expect is appropriate here.
    let port: u16 = settings
        .get("port")
        .expect("port must be set")
        .parse()
        .expect("port must be a number");
    println!("listening on port {port}");
}
