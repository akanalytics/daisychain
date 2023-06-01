use crate::prelude::*;
use strum::VariantNames;
use strum_macros::{EnumString, EnumVariantNames};

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

fn parse_fancy_enum(s: &str) -> Result<(&str, FancyColor), dc::ParseError> {
    dc::Cursor::from(s)
        .text_alt(FancyColor::VARIANTS)
        .parse_selection()
        .validate()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_parse_enum() -> Result<(), dc::ParseError> {
        let (c, color) = parse_fancy_enum("Burgundy Arrow".into())?;
        assert_eq!(color, FancyColor::Burgundy);
        assert_eq!(c, " Arrow");
        Ok(())
    }
}
