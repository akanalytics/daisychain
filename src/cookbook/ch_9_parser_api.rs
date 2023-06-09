use std::str::FromStr;

use crate::prelude::*;

#[derive(PartialEq, Debug)]
struct Money(f32);

///
/// FromStr style parser:
///
/// uses Rust standard trait FromStr
///
impl FromStr for Money {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // we use a sub-selection to group dollars+cents
        // together into a single selection
        // for parsing into an f32
        let (_c, float) = Cursor::from(s)
            .debug_context("Money FromStr")
            .text("$")
            .select(|c| c.digits(1..).text(".").digits(2..=2))
            .parse_selection::<f32>()
            .validate()?;
        Ok(Money(float))
    }
}

///
/// stir-style (free-function) parser:
///
/// the function takes a &str position and returns a Result of (&str, T)
///
fn parse_str_money(s: &str) -> Result<(&str, Money), ParsingError> {
    // convert from a &str using Cursor::from,
    // and convert back to a &str using cursor.str()
    let (c, float) = Cursor::from(s)
        .debug_context("str_money")
        .text("$")
        .select(|c| c.digits(1..).text(".").digits(2..=2))
        .parse_selection::<f32>()
        .validate()?;
    Ok((c, Money(float)))
}

// ///
// /// Cursor-style (free-function) parser:
// ///
// /// the function takes a Cursor and returns a Result of (Cursor, T)
// ///
// fn parse_money(s: Cursor) -> Result<(Cursor, Money), dc::ParseError> {
//     let (c, float) = Cursor::from(s)
//         .debug_context("money")
//         .text("$")
//         .select(|c| c.digits(1..).text(".").digits(2..=2))
//         .parse_selection()
//         .validate()?;
//     Ok((c, Money(float)))
// }

///
/// Stir-style (associated-function or method) parser:
///
/// the function takes a Cursor and returns a Result of (Cursor, T)
///

// eg MoneyParser("$".to_string())
struct MoneyParser {
    currency: String,
}

/// because we have two references as assoc-function parameters
/// rust needs to be told about lifetimes for Cursor/&str
impl MoneyParser {
    fn parse<'a>(&self, s: &'a str) -> Result<(&'a str, Money), ParsingError> {
        let (c, float) = Cursor::from(s)
            .debug_context("MoneyParser::parse")
            .text(&self.currency)
            .select(|c| c.digits(1..).text(".").digits(2..=2))
            .parse_selection()
            .validate()?;
        Ok((c, Money(float)))
    }
}

fn parse_lots_of_money(s: &str) -> Result<Vec<Money>, ParsingError> {
    let mp = MoneyParser {
        currency: "£".to_string(),
    };
    let (c, m1, m2) = Cursor::from(s)
        .select(|c| c.text("$").digits(1..).text(".").digits(2..=2))
        .parse_selection() // uses <Money as FromStr>
        .ws()
        .parse_with(parse_str_money) // uses stir-style free function
        .ws()
        .validate()?;

    let (_c, m3) = Cursor::from(c)
        .parse_with(|c| mp.parse(c)) // uses stir-style free function
        .validate()?;

    let mut vec = vec![];
    vec.extend([m1, m2, m3]);
    Ok(vec)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_parse_lots_of_money() {
        let s = "$1.15 $2.25 £4.45";
        let vec = parse_lots_of_money(s).unwrap();
        assert_eq!(vec[0], Money(1.15));
        assert_eq!(vec[1], Money(2.25));
        assert_eq!(vec[2], Money(4.45));
    }
}
