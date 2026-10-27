// Converts Celsius to Fahrenheit. Temperatures below absolute zero (-273.15 C) are an error.
fn celsius_to_fahrenheit(c: f64) -> Result<f64, String> {
    if c < -273.15 {
        return Err(format!("{c} C is below absolute zero"));
    }
    Ok(c * 9.0 / 5.0 + 32.0)
}

fn main() {
    for c in [0.0, 100.0, -40.0, -300.0] {
        println!("{c:?} C -> {:?}", celsius_to_fahrenheit(c));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn freezing_point() {
        assert_eq!(celsius_to_fahrenheit(0.0), Ok(32.0));
    }

    #[test]
    fn boiling_point() {
        assert_eq!(celsius_to_fahrenheit(100.0), Ok(212.0));
    }

    #[test]
    fn same_in_both_scales_at_minus_40() {
        assert_eq!(celsius_to_fahrenheit(-40.0), Ok(-40.0));
    }

    #[test]
    fn below_absolute_zero_is_an_error() {
        assert!(celsius_to_fahrenheit(-300.0).is_err());
    }
}
