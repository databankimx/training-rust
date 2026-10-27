//! The art crate models artistic concepts. Its main items are re-exported at the top level,
//! so users write `art::mix` rather than `art::utils::mix`.

pub mod kinds {
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    #[derive(Debug, PartialEq)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::{PrimaryColor, SecondaryColor};

    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        match (c1, c2) {
            (PrimaryColor::Red, PrimaryColor::Yellow) | (PrimaryColor::Yellow, PrimaryColor::Red) => SecondaryColor::Orange,
            (PrimaryColor::Yellow, PrimaryColor::Blue) | (PrimaryColor::Blue, PrimaryColor::Yellow) => SecondaryColor::Green,
            (PrimaryColor::Red, PrimaryColor::Blue) | (PrimaryColor::Blue, PrimaryColor::Red) => SecondaryColor::Purple,
            // Mixing a color with itself isn't covered in this example, so it falls back to Orange.
            _ => SecondaryColor::Orange,
        }
    }
}

// Re-exports put the main items at the top level, so the module structure isn't part of the public API.
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn red_and_yellow_make_orange() {
        assert_eq!(mix(PrimaryColor::Red, PrimaryColor::Yellow), SecondaryColor::Orange);
    }

    #[test]
    fn yellow_and_blue_make_green() {
        assert_eq!(mix(PrimaryColor::Blue, PrimaryColor::Yellow), SecondaryColor::Green);
    }
}
