use chainsaw::prelude::*;
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

fn parse_fancy_enum(c: cs::Cursor) -> Result<(cs::Cursor, FancyColor), cs::ParseError> {
    c.text_alt(FancyColor::VARIANTS).parse_selection().validate()
}

fn main() {
    let _ = parse_fancy_enum("Burgundy".into());
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_parse_enum() -> Result<(), cs::ParseError> {
        let (c, color) = parse_fancy_enum("Burgundy Arrow".into())?;
        assert_eq!(color, FancyColor::Burgundy);
        assert_eq!(c.str()?, " Arrow");
        Ok(())
    }
}
