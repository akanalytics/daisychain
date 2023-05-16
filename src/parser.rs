use log::trace;

use crate::prelude::{Cursor, Matchable, ParseError};

pub trait Parser<'c, C, T> {
    type Error;
    fn parse(&mut self, s: C) -> Result<(C, T), Self::Error>;
}

pub type StrFunc<T, E> = for<'c> fn(&'c str) -> Result<(&'c str, T), E>;
pub type StrMethod<T, X> = for<'c> fn(x: &'c X, &'c str) -> Result<(&'c str, T), ParseError>;


impl<'c, T> Parser<'c, Cursor<'c>, T> for StrFunc<T, ParseError> {
    type Error = ParseError;
    fn parse(&mut self, c: Cursor<'c>) -> Result<(Cursor<'c>, T), ParseError> {
        trace!("#### fn(&'b str): {s}", s = c.cur.unwrap_or("-"));
        let (s, t) = (self)(c.str()?)?;
        Ok((Cursor::from(s), t))
    }
}

impl<'c, T, F> Parser<'c, Cursor<'c>, T> for F
where
    F: FnMut(Cursor<'c>) -> Result<(Cursor<'c>, T), ParseError>,
{
    type Error = ParseError;
    fn parse(&mut self, s: Cursor<'c>) -> Result<(Cursor<'c>, T), ParseError> {
        trace!("#### FnMut(SelectableStr): {s}", s = s.cur.unwrap_or("-"));
        (self)(s)
    }
}

impl<'c, T, X> Parser<'c, Cursor<'c>, T> for (
    &'c X,
    StrMethod<T,X>,
) {
    type Error = ParseError;
    fn parse(&mut self, c: Cursor<'c>) -> Result<(Cursor<'c>, T), ParseError> {
        trace!("#### fn(context, &str): {s}", s = c.cur.unwrap_or("-"));
        let (s, t) = (self.1)(self.0, c.str()?)?;
        Ok((Cursor::from(s), t))
    }
}








#[cfg(test)]
mod tests {
    use crate::{prelude::{Cursor, ParseError, Parser}, parser::{StrFunc, StrMethod}};

    #[test]
    fn test_casting() {

        // define a simple lexer+parser
        fn lp<'a, T>(s: Cursor<'a>, mut p: impl Parser<'a, Cursor<'a>, T, Error=ParseError>) {
            let _ = p.parse(s);
        }

        #[derive(Default)]
        struct StructEx;

        impl StructEx {
            fn parse_ex2<'a>(&self, s: &'a str) -> Result<(&'a str, String), ParseError> {
                Ok((s, String::from("Example2")))
            }
            fn parse_ex4<'a>(&self, s: Cursor<'a>) -> Result<(Cursor<'a>, String), ParseError> {
                Ok((s, String::from("Example4")))
            }
        }

        fn parse_ex1(s: &str) -> Result<(&str, String), ParseError> {
            Ok((s, String::from("Example1")))
        }

        fn parse_ex3(s: Cursor) -> Result<(Cursor, String), ParseError> {
            Ok((s, String::from("Example3")))
        }

        let selfie = StructEx;

        let f: for<'a> fn(&'a str) -> Result<(&'a str, String), ParseError> = parse_ex1;
        let tup_ex2 = (&selfie, StructEx::parse_ex2 as StrMethod<_, _>);
        lp("parse_ex1 as ...     ".into(), f);
        lp("parse_ex1 as StrFunc ".into(), parse_ex1 as StrFunc<_, _>);
        lp("(&selfie,f)          ".into(), tup_ex2);
        lp("parse_ex3            ".into(), parse_ex3);
        lp("|c| self.parse_ex4(c)".into(), |c| selfie.parse_ex4(c));
        lp("|c| parse_ex(c)      ".into(), |c| parse_ex3(c));
    }

}
