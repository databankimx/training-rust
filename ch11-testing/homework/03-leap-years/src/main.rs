// A year is a leap year if it is divisible by 4, except century years, which must be divisible by 400.
fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn main() {
    for year in [2024, 1900, 2000, 2023] {
        println!("{year}: {}", is_leap_year(year));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordinary_leap_year() {
        assert!(is_leap_year(2024));
    }

    #[test]
    fn common_year_is_not_leap() {
        assert!(!is_leap_year(2023));
    }

    #[test]
    fn century_is_not_leap() {
        assert!(!is_leap_year(1900));
    }

    #[test]
    fn fourth_century_is_leap() {
        assert!(is_leap_year(2000));
    }
}
