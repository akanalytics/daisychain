use chainsaw::prelude::*;

#[derive(PartialEq, Debug)]
struct QuotedText {
    quote: char,
    text: String,
}

impl QuotedText {
    fn new(quote: char, text: &str) -> Self {
        Self {
            quote,
            text: text.to_string(),
        }
    }
}

/// eg "'Hello World!', said Ferris"
/// lexing and parsing together
///
fn parse_quoted_text(c: cs::Cursor) -> Result<(cs::Cursor, QuotedText), cs::ParseError> {
    // step 1: find out which quote char is used
    let (c, quote) = c
        .chars_in(1..=1, &['"', '\''])
        .parse_selection()
        .validate()?;

    // step 2: use the quote character to extract the text between quotes
    let (c, text) = c
        .chars_not_in(0.., &[quote])
        .parse_selection()
        .chars_in(1..=1, &[quote])
        .validate()?;
    Ok((c, QuotedText { quote, text }))
}

/// alternative implementation using "bind"
/// 
fn parse_quoted_text_v2(c: cs::Cursor) -> Result<(cs::Cursor, QuotedText), cs::ParseError> {
    let mut quote = char::default();
    let (c, text) = c
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
    fn test_parse_quoted_text() -> Result<(), cs::ParseError> {
        let s = "'Hello World!', said Ferris";
        let (c, qt) = parse_quoted_text(cs::Cursor::from(s))?;
        assert_eq!(qt, QuotedText::new('\'', "Hello World!"));
        assert_eq!(c.str()?, ", said Ferris");

        let (cursor, qt) = parse_quoted_text("\"Hi\", he said".into())?;
        assert_eq!(qt, QuotedText::new('"', "Hi"));
        assert_eq!(cursor.str()?, ", he said");

        let (cursor, qt) = parse_quoted_text_v2("\"Hi\", he said".into())?;
        assert_eq!(qt, QuotedText::new('"', "Hi"));
        assert_eq!(cursor.str()?, ", he said");

        let res = parse_quoted_text("'Hi, ".into());
        assert!(res.is_err());
        Ok(())
    }
}
