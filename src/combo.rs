use std::str::FromStr;
use std::{fmt::Debug, marker::PhantomData};

use log::Level::Trace;
use log::{log_enabled, trace};

use crate::{prelude::ParsingError, LABEL, LOG_TARGET};

fn type_suffix(type_name: &str) -> &str {
    if let Some(i) = type_name.rfind("::") {
        &type_name[i + 2..]
    } else {
        type_name
    }
}

pub trait Parser<'a>: Sized {
    type Input: Debug;
    type Output: Debug;
    type Error;

    fn name(&self, indent: &str) -> String {
        format!(
            "{indent}parser({input}) -> Result<(), {error}>",
            input = std::any::type_name::<Self::Input>(),
            error = std::any::type_name::<Self::Error>()
        )
    }

    fn validate(&mut self, inp: Self::Input) -> Result<Self::Output, Self::Error>;

    // fn results(&mut self, inp: Self::Input) -> Result<(Self::Input,<T as DeTuple>::Output), Self::Error> where T: DeTuple {
    //     self.parse(inp).map(|(i,t)| (i,t.detuple()))
    // }

    fn chain_parser<T, P2: Parser<'a, Output = T>>(self, p2: P2) -> Chain<'a, Self, P2>
    where
        P2: Parser<'a, Input = Self::Input, Error = Self::Error>,
        // Self::Input: Clone,
        // P2: Parser<'a, Input = Self::Input, Error = Self::Error>,
        // (Self::Output, T): ConcatTuple<Self::Output, P2::Input>,
    {
        Chain {
            p1: self,
            p2,
            pdlt: Default::default(),
        }
    }
}

fn func_ws(s: &str) -> Result<&str, ParsingError> {
    Ok(s.trim_start())
}

type Lexer = for<'b> fn(&'b str) -> Result<&'b str, ParsingError>;

impl<'a> StrParser<'a, &'a str> for Lexer {}

pub trait StrParser<'a, O>: Parser<'a, Input = &'a str, Output = O, Error = ParsingError> {
    fn ws(self) -> Chain<'a, Self, Lexer> {
        self.chain_parser(func_ws)
    }

    fn debug_context(self, span_name: &'static str) -> Self {
        if log_enabled!(target: LOG_TARGET, Trace) {
            LABEL.with(|f| f.set("")); // blank the span name before logging
                                       // self.log_success("debug_context", span_name);
            LABEL.with(|f| f.set(span_name));
        }

        self
    }

    fn parse_selection<T: FromStr + Debug>(self) -> ParseSelection<'a, T> {
        ParseSelection {
            pd: Default::default(),
        }
    }

    fn find(self, needle: &'a str) -> Find<'a, Self> {
        Find {
            needle,
            chain: Chain {
                p1: self,
                p2: SP,
                pdlt: Default::default(),
            },
        }
    }
}

pub struct ParseSelection<'a, T> {
    pd: PhantomData<&'a T>,
}

impl<'a, T> Parser<'a> for ParseSelection<'a, T>
where
    T: FromStr + Debug,
{
    type Input = &'a str;
    type Output = (&'a str, T);
    type Error = ParsingError;

    fn validate(&mut self, inp: Self::Input) -> Result<Self::Output, Self::Error> {
        match inp.parse::<T>() {
            Ok(t) => Ok((inp, t)),
            Err(..) => {
                let e = ParsingError::NoMatch {
                    action: "FromStr",
                    args: "",
                };
                Err(e)
            }
        }
    }
}

// impl<'a, T, O> StrParser<'a, O> for ParseSelection<'a, (&'a str, T)>
// where
//     T: FromStr + Debug,
//     ParseSelection<'a, (&'a str, T)>: Parser<'a>,
// {
// }

pub struct Find<'a, P> {
    needle: &'a str,
    chain: Chain<'a, P, SP>,
}

impl<'a, P> Parser<'a> for Find<'a, P>
where
    Chain<'a, P, SP>: StrParser<'a, &'a str>,
{
    type Input = &'a str;
    type Output = &'a str;
    type Error = ParsingError;

    fn validate(&mut self, inp: Self::Input) -> Result<Self::Output, Self::Error> {
        self.chain.validate(inp).and_then(|s| {
            s.find(self.needle)
                .map(|i| &s[i..])
                .ok_or(ParsingError::NoMatch {
                    action: "",
                    args: "",
                })
        })
    }
}

pub struct Chain<'a, P1, P2> {
    p1: P1,
    p2: P2,
    pdlt: PhantomData<&'a ()>,
}

impl<'a, P1, P2, O> StrParser<'a, O> for Chain<'a, P1, P2>
where
    Chain<'a, P1, P2>: Parser<'a, Input = &'a str, Output = O, Error = ParsingError>,
    P1: StrParser<'a, O>,
    P2: StrParser<'a, &'a str>,
{
}

impl<'a, P1, P2, S, T> Parser<'a> for Chain<'a, P1, P2>
where
    S: Clone + Debug,
    T: Debug,
    P1::Input: Clone,
    P1: Parser<'a, Output = S>,
    P2: Parser<'a, Output = T, Input = P1::Input, Error = P1::Error>,
    (S, T): ConcatTuple<S, P2::Input>,
{
    type Input = P1::Input;
    type Error = P1::Error;
    type Output = <(P1::Output, P2::Output) as ConcatTuple<P1::Output, P2::Input>>::Output;

    fn name(&self, indent: &str) -> String {
        let indent = indent.replace("└──", "|  ");
        format!(
            "{indent}chain ({input}) -> Result<{output}, {error}>\n{s}\n{t}\n{indent}",
            input = std::any::type_name::<Self::Input>(),
            output = std::any::type_name::<Self::Output>(),
            error = type_suffix(std::any::type_name::<Self::Error>()),
            // s = std::any::type_name::<P1>(),
            s = self.p1.name(&format!("{indent}└──")),
            // t = std::any::type_name::<P2>(),
            t = self.p2.name(&format!("{indent}└──")),
        )
    }

    //
    //  (str)  -> (str, (S)
    //  (str)  -> (str, Lex())
    //  -----
    //  (str) ->  (str, (S, Lex))
    //  (str) ->  (str, S)
    //
    //
    //
    //  (str)  -> (str, S)
    //  (str)  -> (str, Par(T))
    //  -----
    //  (str) ->  (str, (S, Par<T>))
    //
    //
    //  (str)  -> (str, (S0, S1))
    //  (str)  -> (str, T)
    //  -----
    //  (str) ->  (str, ((S0,S1), T))
    //
    //
    //  (str)  -> (str, ((S0, S1), S2))
    //  (str)  -> (str, T)
    //  -----
    //  (str) ->  (str, (((S0,S1), S2), T))

    fn validate(&mut self, inp: Self::Input) -> Result<Self::Output, Self::Error> {
        let o1: P1::Output = self.p1.validate(inp.clone())?;
        trace!("o1: {p1}({inp:?}) -> Ok({o1:?})", p1 = self.p1.name(""));
        let s = <(P1::Output, P2::Output) as ConcatTuple<P1::Output, P2::Input>>::input2_from(
            o1.clone(),
        );
        let o2: P2::Output = self.p2.validate(s.clone())?;
        trace!("o2: {p2}({s:?}) -> Ok({o2:?})", p2 = self.p2.name(""));
        let o12: Self::Output = ConcatTuple::concat((o1, o2));
        trace!("o12: {o12:?}");
        Ok(o12)
    }

    // S: Clone + Debug,
    // T: Debug,
    // P1::Input: Clone,
    // P1: Parser<'a, Output = S>,
    // P2: Parser<'a, Output = T, Input = P1::Input, Error = P1::Error>,
    // (S, T): ConcatTuple<S, P2::Input>,
}

pub trait ConcatTuple<O1, I2>
where
    Self: Debug,
{
    type Output: Debug;
    fn concat(self) -> Self::Output;
    fn input2_from(o1: O1) -> I2;
}

impl<S: Debug> ConcatTuple<S, S> for (S, S) {
    type Output = S;
    fn concat(self) -> Self::Output {
        let (_s0, s1) = self;
        s1
    }
    fn input2_from(o1: S) -> S {
        o1
    }
}

impl<S, T> ConcatTuple<(S, T), S> for ((S, T), S)
where
    S: Debug,
    T: Debug,
{
    type Output = (S, T);
    fn concat(self) -> Self::Output {
        let ((_s0, t), s2) = self;
        (s2, t)
    }
    fn input2_from(o1: (S, T)) -> S {
        let (s0, _t) = o1;
        s0
    }
}

impl<S, T0, T1> ConcatTuple<(S, T0, T1), S> for ((S, T0, T1), S)
where
    Self: Debug,
    (S, T0, T1): Debug,
{
    type Output = (S, T0, T1);
    fn concat(self) -> Self::Output {
        let ((_s0, t0, t1), s2) = self;
        (s2, t0, t1)
    }
    fn input2_from(o1: (S, T0, T1)) -> S {
        let (s0, _t0, _t1) = o1;
        s0
    }
}

impl<S, S0, T0> ConcatTuple<(S, S0), S> for ((S, S0), (S, T0))
where
    Self: Debug,
    (S, S0, T0): Debug,
{
    type Output = (S, S0, T0);
    fn concat(self) -> Self::Output {
        let ((_s0, s0), (t, t0)) = self;
        (t, s0, t0)
    }
    fn input2_from(o1: (S, S0)) -> S {
        let (s0, _s0) = o1;
        s0
    }
}

// ConcatTuple<(&str, i32), &str>` is not implemented for `((&str, i32), (&str, i32))

// struct IO<I,O>{ i:I, o:O }

// impl<T0, T1, S> ConcatTuple for (T0, T1, Lex<S>) {
//     type Output = (T0, T1, S);

//     fn concat(self) -> Self::Output {
//         let (t0, t1, Lex(s)) = self;
//         (t0, t1, s)
//     }
// }

// impl<T0, S> ConcatTuple for (&str, T0), &str) {
//     type Output = (T0, S);
//     fn concat(self) -> Self::Output {
//         let (t0, Lex(s)) = self;
//         (t0, s)
//     }
// }

// impl<T0, T1, S> ConcatTuple for (T0, T1, Par<S>) {
//     type Output = (T0, T1, S);
//     fn concat(self) -> Self::Output {
//         let (t0, t1, Par(s)) = self;
//         (t0, t1, s)
//     }
// }

// impl<T0, S> ConcatTuple for (T0, Par<S>) {
//     type Output = (T0, S);
//     fn concat(self) -> Self::Output {
//         let (t0, Par(s)) = self;
//         (t0, s)
//     }
// }

#[derive(Debug, PartialEq)]
pub struct Lex<T>(T);

#[derive(Debug, PartialEq)]
pub struct Par<T>(T);

impl<'a, F2> Parser<'a> for F2
where
    F2: FnMut(&'a str) -> Result<&'a str, ParsingError>,
{
    type Error = ParsingError;
    type Input = &'a str;
    type Output = &'a str;
    fn name(&self, indent: &str) -> String {
        format!(
            "{indent}Lex {func}({input}) -> Result<({input}, {t}), {error}>",
            func = type_suffix(std::any::type_name::<Self>()),
            t = type_suffix(std::any::type_name::<()>()),
            input = std::any::type_name::<Self::Input>(),
            error = type_suffix(std::any::type_name::<Self::Error>())
        )
    }
    fn validate(&mut self, s: &'a str) -> Result<&'a str, ParsingError> {
        // trace!("#### FnMut(SelectableStr): {s}", s = s.cur.unwrap_or("-"));
        let func = self;
        match (func)(s) {
            Ok(s) => Ok(s),
            Err(e) => Err(e),
        }
    }
}

impl<'a, F2, T: Debug> Parser<'a> for Par<F2>
where
    F2: FnMut(&'a str) -> Result<(&'a str, T), ParsingError>,
{
    type Error = ParsingError;
    type Input = &'a str;
    type Output = (&'a str, T);
    fn name(&self, indent: &str) -> String {
        format!(
            "{indent}Par {func}({input}) -> Result<({input}, {t}), {error}>",
            func = type_suffix(std::any::type_name::<Self>()),
            t = type_suffix(std::any::type_name::<T>()),
            input = std::any::type_name::<Self::Input>(),
            error = type_suffix(std::any::type_name::<Self::Error>())
        )
    }
    fn validate(&mut self, s: &'a str) -> Result<(&'a str, T), ParsingError> {
        // trace!("#### FnMut(SelectableStr): {s}", s = s.cur.unwrap_or("-"));
        let Par(func) = self;
        match (func)(s) {
            Ok((s, t)) => Ok((s, t)),
            Err(e) => Err(e),
        }
    }
}

pub struct SP;

impl SP {
    fn make_parser<'a, T: Debug, E>(
        &self,
        p: impl Parser<'a, Output = T, Input = &'a str, Error = E>,
    ) -> impl Parser<'a, Output = T, Input = &'a str, Error = E> {
        p
    }
}

impl<'a> StrParser<'a, &'a str> for SP {}

impl<'a> Parser<'a> for SP {
    type Input = &'a str;
    type Error = ParsingError;
    type Output = &'a str;
    fn name(&self, indent: &str) -> String {
        format!(
            "{indent}SP({input}) -> Result<(), {error}>",
            input = std::any::type_name::<Self::Input>(),
            error = std::any::type_name::<Self::Error>()
        )
    }

    fn validate(&mut self, inp: Self::Input) -> Result<Self::Input, Self::Error> {
        Ok(inp)
    }
}

/// (a, (b,c)) -> (a,b,c)
/// (a, (b, (c,d))) ->

pub trait DeTuple {
    type Output;
    fn detuple(self) -> Self::Output;
}

impl<A, B, C> DeTuple for (A, (B, C, ())) {
    type Output = (A, B, C);

    fn detuple(self) -> Self::Output {
        (self.0, (self.1).0, (self.1).1)
    }
}

impl<A, B, C, D> DeTuple for (A, (B, (C, D, ()))) {
    type Output = (A, B, C, D);

    fn detuple(self) -> Self::Output {
        (self.0, self.1 .0, self.1 .1 .0, self.1 .1 .1)
    }
}

#[cfg(test)]
mod tests {
    use super::DeTuple;
    use crate::{
        combo::{Par, Parser, SP},
        prelude::ParsingError,
    };
    use test_log::test;
    #[test]
    fn test_detuple() {
        println!("{:?}", (1, (2, 3, ())).detuple());
        println!("{:?}", (1, (2, (3, 4, ()))).detuple());
    }

    #[test]
    fn test_combo() {
        // define a simple lexer+parser
        assert_eq!(SP.validate("cat").unwrap(), "cat");
        fn tail_lexer(s: &str) -> Result<&str, ParsingError> {
            Ok(&s[1..])
        }

        fn ws(s: &str) -> Result<&str, ParsingError> {
            Ok(s.trim_start())
        }

        fn num_parser(s: &str) -> Result<(&str, i32), ParsingError> {
            Ok((&s[1..], s[0..=0].parse::<i32>()?))
        }
        #[derive(Default)]
        struct StructEx;

        let mut parser1 = SP
            .chain_parser(tail_lexer)
            .chain_parser(ws)
            .chain_parser(tail_lexer);

        let mut _parser1b = tail_lexer.chain_parser(ws).chain_parser(tail_lexer);

        let parser2 = SP
            .chain_parser(|s| tail_lexer(s))
            .chain_parser(ws)
            .chain_parser(tail_lexer);

        let s = parser1.validate("c   t2dog").unwrap();
        println!("{}", parser1.name(""));
        println!("{}", parser2.name(""));
        assert_eq!(s, "2dog");

        let mut parser3 = SP.make_parser(Par(num_parser));
        println!("{}", parser3.name(""));

        let (s, d) = parser3.validate("5dog").unwrap();
        assert_eq!(s, "dog");
        assert_eq!(d, 5);

        let mut parser4 = parser3.chain_parser(parser1);
        let (s, d) = parser4.validate("6d   gs").unwrap();
        assert_eq!(s, "s");
        assert_eq!(d, 6);
        println!("{}", parser4.name(""));

        // Parser<Input=&str, Output=(i32, i32), Error=ParseError>::name(
        let mut parser5 = parser4.chain_parser(Par(num_parser));
        println!("{}", parser5.name(""));
        let (s, d1, d2) = parser5.validate("7d   g4x").unwrap();
        assert_eq!(s, "x");
        assert_eq!(d1, 7);
        assert_eq!(d2, 4);
        println!("{}", parser5.name(""));
    }
}
