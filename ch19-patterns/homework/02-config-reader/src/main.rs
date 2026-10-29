// Splits a line into a key and a value. A line with no '=' gives None.
fn parse_line(line: &str) -> Option<(&str, &str)> {
    let Some((key, value)) = line.split_once('=') else {
        return None;
    };
    Some((key.trim(), value.trim()))
}

// Reads the settings. Comments and blank lines are skipped, and problems are collected as errors.
fn read_config(text: &str) -> (Option<u16>, Vec<String>) {
    let mut port = None;
    let mut errors = Vec::new();

    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let Some((key, value)) = parse_line(trimmed) else {
            errors.push(format!("bad line: {trimmed}"));
            continue;
        };

        if key == "port" {
            // A port that doesn't fit in a u16 is an error, and the previous port is kept.
            if let Ok(number) = value.parse::<u16>() {
                port = Some(number);
            } else {
                errors.push(format!("bad line: {trimmed}"));
            }
        }
    }

    (port, errors)
}

fn main() {
    let text = "# server settings\nport = 8080\n\nname = demo\nport = 99999\noops\n";
    let (port, errors) = read_config(text);
    println!("port: {}", port.map_or(String::from("none"), |p| p.to_string()));
    println!("errors: {errors:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_a_key_and_value() {
        assert_eq!(parse_line("port = 80"), Some(("port", "80")));
    }

    #[test]
    fn comments_and_blank_lines_are_skipped() {
        assert_eq!(read_config("# note\n\n"), (None, vec![]));
    }

    #[test]
    fn a_valid_port_is_kept() {
        assert_eq!(read_config("port=80"), (Some(80), vec![]));
    }

    #[test]
    fn a_port_too_large_is_an_error() {
        assert_eq!(
            read_config("port = 70000"),
            (None, vec![String::from("bad line: port = 70000")])
        );
    }
}
