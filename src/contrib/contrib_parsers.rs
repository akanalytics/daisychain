// \b[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,4}\b

use once_cell::sync::Lazy;

use crate::prelude::*;

pub fn email<'a, C: Matchable<'a>>(c: C) -> C {
    static NAME: Lazy<Vec<char>> = Lazy::new(|| {
        ('A'..'Z')
            .chain('0'..'9')
            .chain(['.', '_', '%', '+', '-'])
            .collect()
    });
    static DOMAIN: Lazy<Vec<char>> =
        Lazy::new(|| ('A'..'Z').chain('0'..'9').chain(['.', '-']).collect());
    static TLD: Lazy<Vec<char>> = Lazy::new(|| ('A'..'Z').collect());

    c.chars_match(1.., |c| NAME.contains(&c.to_ascii_uppercase()))
        .text("@")
        .chars_match(1.., |c| DOMAIN.contains(&c.to_ascii_uppercase()))
    // .text(".")
    // .chars_match(2..=4, |c| TLD.contains(&c.to_ascii_uppercase()))
}

#[cfg(test)]
mod tests {
    use crate::{
        contrib::parsers::email,
        prelude::*,
    };
    use test_log::test;

    #[test]
    fn test_email() {
        assert_eq!(email(dc::Cursor::from("andy@google.com")).str().unwrap(), "");
        assert_eq!(email(dc::Cursor::from("google.com")).str().is_err(), true);
    }
}
