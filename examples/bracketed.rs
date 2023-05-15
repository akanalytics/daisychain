use chainsaw::prelude::*;

#[derive(PartialEq, Debug)]
struct QuotedText {
    quote: char,
    text:  String,
}

/// eg "'Hello World!', said Ferris"
/// lexing and parsing together
///
fn parse_quoted_text(c: Cursor) -> Result<(Cursor, QuotedText), ParseError> {
    // step 1: find out which quote char is used
    let (c, quote) = c.chars_in(1..1, &['"', '\'']).parse_selection()?;

    // step 2: use the quote character to extract the text between quotes
    let (c, text) = c
        .chars_not_in(1.., &[quote])
        .parse_selection()?
        .chars_in(1..1, &[quote])
        .validate()?;
    Ok((c, QuotedText { quote, text }))
}

#[cfg(test)]
use test_log::test;

#[test]
fn test_parse_quoted_text() -> Result<(), ParseError> {
    let s = r##""Hello World!", said Ferris"##;
    let (c, qt) = parse_quoted_text(cursor(s))?;
    assert_eq!(qt, QuotedText {
        quote: '"',
        text:  "Hello World!".to_string(),
    });
    assert_eq!(c.str()?, ", said Ferris");
    Ok(())
}

fn main() {
    let _ = parse_quoted_text(cursor("\"Hellow World!\""));
}
