use chainsaw::prelude::*;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
enum Color {
    Red,
    Blue,
    Green,
}

impl FromStr for Color {
    type Err = cs::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Red" => Ok(Self::Red),
            "Blue" => Ok(Self::Blue),
            "Green" => Ok(Self::Green),
            _ => Err(cs::ParseError::NoMatch {
                action: "matching color",
                args: "",
            }),
        }
    }
}

/// uses the FromStr trait impl above
fn parse_enum(c: cs::Cursor) -> Result<(cs::Cursor, Color), cs::ParseError> {
    c.text_alt(&["Red", "Blue", "Green"]).parse_selection().validate()
}

fn main() {
    let _ = parse_enum("Red".into());
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_parse_enum() -> Result<(), cs::ParseError> {
        // from_str expects the whole string to match
        assert_eq!(Color::from_str("Red")?, Color::Red);
        assert_eq!(Color::from_str("Red Arrow").is_err(), true);

        // parse_enum consumes only what it needs for matching
        let (c, color) = parse_enum("Red Arrow".into())?;
        assert_eq!(color, Color::Red);
        assert_eq!(c.str()?, " Arrow");
        Ok(())
    }
}
