use std::str::FromStr;

use chainsaw::prelude::*;

use strum::VariantNames;
use strum_macros::{EnumString, EnumVariantNames};

#[derive(PartialEq, Debug, EnumVariantNames)]
#[strum(serialize_all = "UPPERCASE")]

enum Color {
    Red,
    Blue,
    Green,
}

impl FromStr for Color {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Red" => Ok(Self::Red),
            "Blue" => Ok(Self::Blue),
            "Green" => Ok(Self::Green),
            _ => Err(ParseError::NoMatch {
                action: "matching color",
                args: "",
            }),
        }
    }
}

/// uses the FromStr trait impl above
fn parse_enum(c: Cursor) -> Result<(Cursor, Color), ParseError> {
    c.text_alt(&["Red", "Blue", "Green"]).parse_selection()
}

// Example using strum crate.
//
// strum gives us
//   derive(EnumVariantNames) + trait strum::VariantNames => FancyColor::VARIANTS
//   derive(EnumString) + trait FromStr => FancyColor::from_str
// the parse methods can then use VARIANTS and FromStr
// created by strum derive directives
//

#[derive(PartialEq, Debug, EnumVariantNames, EnumString)]
enum FancyColor {
    Burgundy,
    Azure,
    Lime,
}

fn parse_fancy_enum(c: Cursor) -> Result<(Cursor, FancyColor), ParseError> {
    c.text_alt(FancyColor::VARIANTS).parse_selection()
}

fn main() {
    let _ = parse_enum(cursor("Red"));
    let _ = parse_fancy_enum(cursor("Burgundy"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_parse_enum() -> Result<(), ParseError> {
        // from_str expects the whole string to match
        assert_eq!(Color::from_str("Red")?, Color::Red);
        assert_eq!(Color::from_str("Red Arrow").is_err(), true);

        // parse_enum consumes only what it needs for matching
        let (c, color) = parse_enum("Red Arrow".into())?;
        assert_eq!(color, Color::Red);
        assert_eq!(c.str()?, " Arrow");

        let (c, color) = parse_fancy_enum("Burgundy Arrow".into())?;
        assert_eq!(color, FancyColor::Burgundy);
        assert_eq!(c.str()?, " Arrow");

        Ok(())
    }
}
