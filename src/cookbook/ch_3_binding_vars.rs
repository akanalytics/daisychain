use crate::prelude::*;

#[derive(PartialEq, Debug)]
struct QuotedText {
    quote: char,
    text: String,
}

impl QuotedText {
    fn new(quote: char, text: String) -> Self {
        Self { quote, text }
    }
}

/// eg "'Hello World!', said Ferris"
/// lexing and parsing together
///
fn parse_quoted_text(inp: &str) -> Result<(&str, QuotedText), ParsingError> {
    // step 1: find out which quote char is used
    let (c, quote) = Cursor::from(inp)
        .chars_in(1..=1, &['"', '\''])
        .parse_selection()
        .validate()?;

    // step 2: use the quote character to extract the text between quotes
    let (c, text) = Cursor::from(c)
        .chars_not_in(0.., &[quote])
        .parse_selection()
        .chars_in(1..=1, &[quote])
        .validate()?;
    Ok((c, QuotedText { quote, text }))
}

/// alternative implementation using "bind"
///
fn parse_quoted_text_v2(inp: &str) -> Result<(&str, QuotedText), ParsingError> {
    let mut quote = char::default();
    let (c, text) = Cursor::from(inp)
        .chars_in(1..=1, &['"', '\''])
        .parse_selection()
        .bind(&mut quote) // store the quote found, to use below in the matching method-chain
        .chars_not_in(0.., &[quote])
        .parse_selection()
        .chars_in(1..=1, &[quote])
        .validate()?;
    Ok((c, QuotedText { quote, text }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_parse_quoted_text() -> Result<(), ParsingError> {
        let s = "'Hello World!', said Ferris";
        let (c, qt) = parse_quoted_text(s)?;
        assert_eq!(qt, QuotedText::new('\'', "Hello World!".to_string()));
        assert_eq!(c, ", said Ferris");

        let (c, qt) = parse_quoted_text("\"Hi\", he said")?;
        assert_eq!(qt, QuotedText::new('"', "Hi".to_string()));
        assert_eq!(c, ", he said");

        let (c, qt) = parse_quoted_text_v2("\"Hi\", he said")?;
        assert_eq!(qt, QuotedText::new('"', "Hi".to_string()));
        assert_eq!(c, ", he said");

        let res = parse_quoted_text("'Hi, ");
        assert!(res.is_err());
        Ok(())
    }
}
