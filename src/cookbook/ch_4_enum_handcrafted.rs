use crate::prelude::*;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
enum Color {
    Red,
    Blue,
    Green,
}

impl FromStr for Color {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Red" => Ok(Self::Red),
            "Blue" => Ok(Self::Blue),
            "Green" => Ok(Self::Green),
            _ => Err(ParsingError::NoMatch {
                action: "matching color",
                args: "",
            }),
        }
    }
}

/// uses the FromStr trait impl above
fn parse_enum(s: &str) -> Result<(&str, Color), ParsingError> {
    Cursor::from(s)
        .text_alt(&["Red", "Blue", "Green"])
        .parse_selection()
        .validate()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_parse_enum() -> Result<(), ParsingError> {
        // from_str expects the whole string to match
        assert_eq!(Color::from_str("Red")?, Color::Red);
        assert!(Color::from_str("Red Arrow").is_err());

        // parse_enum consumes only what it needs for matching
        let (c, color) = parse_enum("Red Arrow")?;
        assert_eq!(color, Color::Red);
        assert_eq!(c, " Arrow");
        Ok(())
    }
}
