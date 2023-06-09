use crate::prelude::{Cursor, ParsingError, *};

#[derive(PartialEq, Debug)]
enum Number {
    Binary(u32),
    Hex(u32),
    Decimal(u32),
}

/// the idea is to use `if let` to try and parse the enum variants in succession.
///
fn parse_number(s: &str) -> Result<(&str, Number), ParsingError> {
    // try first variant (using clone to save the initial cursor position)
    if let Ok((c, s)) = Cursor::from(s)
        .clone()
        .debug_context("binary")
        .text("0b")
        .chars_in(1.., &['0', '1'])
        .parse_selection_as_str()
        .validate()
    {
        return Ok((c, Number::Binary(u32::from_str_radix(s, 2)?)));
    }

    // try second variant (using clone to save the initial cursor position)
    let hex_chars = b"0123456789ABCDEF".map(|c| c as char);
    if let Ok((c, hex)) = Cursor::from(s)
        .clone()
        .debug_context("hex")
        .text("0x")
        .chars_in(1.., hex_chars.as_slice())
        .parse_selection_as_str()
        .validate()
    {
        return Ok((c, Number::Hex(u32::from_str_radix(hex, 16)?)));
    }

    // try third variant - no need to clone
    if let Ok((c, int)) = Cursor::from(s)
        .debug_context("decimal")
        .digits(1..)
        .parse_selection()
        .validate()
    {
        return Ok((c, Number::Decimal(int)));
    }

    Result::Err(ParsingError::NoMatch {
        action: "Unknown format",
        args: "",
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_parse_number() {
        // from_str expects the whole string to match
        assert_eq!(parse_number("123").unwrap().1, Number::Decimal(123));
        assert_eq!(parse_number("0b1001").unwrap().1, Number::Binary(9));
        assert_eq!(parse_number("0xFF").unwrap().1, Number::Hex(255));
        assert!(parse_number("n/a").is_err());

        // 0b201 will parse as a decimal zero, with cursor moved to 0|b201
        let (c, var) = parse_number("0b201").unwrap();
        assert_eq!(var, Number::Decimal(0));
        assert_eq!(c, "b201");
    }
}
