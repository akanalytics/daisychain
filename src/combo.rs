use std::marker::PhantomData;

use crate::prelude::dc::ParseError;

fn type_suffix(type_name: &str) -> &str {
    if let Some(i) = type_name.rfind("::") {
        &type_name[i + 2..]
    } else {
        type_name
    }
}

pub trait Parser<'a, T>: Sized {
    type Input;
    type Error;

    fn name(&self, indent: &str) -> String {
        format!(
            "{indent}parser({input}) -> Result<(), {error}>",
            input = std::any::type_name::<Self::Input>(),
            error = std::any::type_name::<Self::Error>()
        )
    }

    fn parse(&mut self, inp: Self::Input) -> Result<(Self::Input,T), Self::Error>;

    fn chain_lex<P2: Parser<'a, ()>>(self, p2: P2) -> Chain<'a, Self, P2>
    where
        Self: Parser<'a, T>,
        P2: Parser<'a, (), Input = Self::Input,  Error = Self::Error>,
    {
        Chain {
            p1: self,
            p2,
            pdlt: Default::default()
        }
    }

    fn chain_parser<S, P2: Parser<'a, S>>(self, p2: P2) -> Chain<'a, Self, P2>
    where
        Self: Parser<'a, T>,
        P2: Parser<'a, S, Input = Self::Input, Error = Self::Error>,
    {
        Chain {
            p1: self,
            p2,
            pdlt: Default::default()
        }
    }
}

pub struct Chain<'a, P1, P2>
// where
//     S: ParserT1<'a, S0>,
//     T: ParserT0<'a, Input = S::Input, Error = S::Error>,
{
    p1: P1,
    p2: P2,
    pdlt: PhantomData<&'a ()>,
    // pd0: PhantomData<S0>,
    // pd1: PhantomData<S1>,
    // pd2: PhantomData<S2>,
}



impl<'a, P1, P2, S> Parser<'a, S> for Chain<'a, P1, P2>
where
    P1: Parser<'a, S>,
    P2: Parser<'a, (), Input = P1::Input, Error = P1::Error>,
{
    type Input = P1::Input;
    type Error = P1::Error;

    fn name(&self, indent: &str) -> String {
        let indent = indent.replace("└──", "|  ");
        format!(
            "{indent}chain\n{s}\n{t}\n{indent}",
            // s = std::any::type_name::<P1>(),
            s = self.p1.name(&format!("{indent}└──")),
            // t = std::any::type_name::<P2>(),
            t = self.p2.name(&format!("{indent}└──")),
        )
    }

    // P1: T0 -> (T0, (T1, T2))
    // P2: T0 -> T0
    // return P1(T0), (T1, T2))
    fn parse(&mut self, inp: Self::Input) -> Result<(Self::Input,S), Self::Error> {
        let (io1, o1) : (Self::Input,S) = self.p1.parse(inp)?;
        let o2 : Result<(P1::Input ,S), P2::Error> = match self.p2.parse(io1) {
            Ok((io2,())) => Ok((io2,o1)),
            Err(e) => Err(e),
        };
        o2
    }
}






// pub trait ParserT0<'a>: Parser<'a> {}

// pub trait ParserT1<'a, T0>: Sized {
//     type Input;
//     type Error;

//     fn name(&self, indent: &str) -> String {
//         format!(
//             "{indent}parser({input}) -> Result<({input},{t0}), {error}>",
//             input = std::any::type_name::<Self::Input>(),
//             t0 = std::any::type_name::<T0>(),
//             error = std::any::type_name::<Self::Error>()
//         )
//     }
//     fn parse(&mut self, inp: Self::Input) -> Result<(Self::Input, T0), Self::Error>;
//     fn chain_lex<P2: ParserT0<'a>>(self, p2: P2) -> Chain<'a, Self, P2, T0, (), ()>
//     where
//         //     // Self: ParserT0<'a>,
//         P2: ParserT0<'a, Input = Self::Input, Error = Self::Error>,
//     {
//         Chain {
//             p1: self,
//             p2,
//             pdlt: Default::default(),
//             pd0: Default::default(),
//             pd1: Default::default(),
//             pd2: Default::default(),
//         }
//     }
// }





// pub trait ParserT2<'a, T0, T1> {
//     type Input;
//     type Error;

//     fn name(&self, indent: &str) -> String {
//         format!(
//             "{indent}parser({input}) -> Result<({t0}, {t1}), {error}>",
//             input = std::any::type_name::<Self::Input>(),
//             t0 = std::any::type_name::<T0>(),
//             t1 = std::any::type_name::<T1>(),
//             error = std::any::type_name::<Self::Error>()
//         )
//     }
//     fn parse(&mut self, inp: Self::Input) -> Result<(Self::Input, T0, T1), Self::Error>;
// }

// pub trait Chainable0<'a>:  Parser<'a> + Sized {
//     fn chain<P2>(self, p2: P2) -> Chain<Self, P2>
//     where
//         P2: Parser<'a, Input = Self::Input, Output = (), Error = Self::Error>,
//     {
//         Chain { p1: self, p2 }
//     }
// }

// pub trait Chainable1<'a>:  Parser<'a> + Sized {
//     fn chain<S, T, P2>(self, p2: P2) -> Chain<Self, P2>
//     where
//     Self: Parser<'a, Output = (S,) , Error = >,
//     P2: Parser<'a, Input = Self::Input, Output = (Self::Input, T) , Error = Self::Error>,
//     {
//         Chain { p1: self, p2 }
//     }
// }

// pub trait Parser0<'a>: Sized {
//     type Input;
//     type Error;

//     fn parse(&mut self, inp: Self::Input) -> Result<Self::Input, Self::Error>;

//     fn chain0<P2>(self, p2: P2) -> Chain<Self, P2>
//     where
//         P2: Parser0<'a, Input = Self::Input, Error = Self::Error>,
//     {
//         Chain { p1: self, p2 }
//     }
// }

// impl<'a, F> Parser<'a, ()> for F
// where
//     F: FnMut(&'a str) -> Result<(&'a str, ()), ParseError>,
// {
//     type Error = ParseError;
//     type Input = &'a str;

//     fn name(&self, indent: &str) -> String {
//         format!(
//             "{indent}ParserT0 {func}({input}) -> Result<({input}), {error}>",
//             func = type_suffix(std::any::type_name::<Self>()),
//             input = std::any::type_name::<Self::Input>(),
//             error = type_suffix(std::any::type_name::<Self::Error>())
//         )
//     }
//     fn parse(&mut self, s: &'a str) -> Result<(&'a str, ()), ParseError> {
//         // trace!("#### FnMut(SelectableStr): {s}", s = s.cur.unwrap_or("-"));
//         (self)(s)
//     }
// }


impl<'a, F2, T> Parser<'a, T> for F2
where
    F2: FnMut(&'a str) -> Result<(&'a str, T), ParseError>,
{
    type Error = ParseError;
    type Input = &'a str;
    fn name(&self, indent: &str) -> String {
        format!(
            "{indent}ParserT1 {func}({input}) -> Result<({input}, {t}), {error}>",
            func = type_suffix(std::any::type_name::<Self>()),
            t = type_suffix(std::any::type_name::<T>()),
            input = std::any::type_name::<Self::Input>(),
            error = type_suffix(std::any::type_name::<Self::Error>())
        )
    }
    fn parse(&mut self, s: &'a str) -> Result<(&'a str, T), ParseError> {
        // trace!("#### FnMut(SelectableStr): {s}", s = s.cur.unwrap_or("-"));
        (self)(s)
    }
}


struct SP;

impl SP {
    fn make_parser<'a, T, E>(
        &self,
        p: impl Parser<'a, T, Input = &'a str, Error = E>,
    ) -> impl Parser<'a, T,  Input = &'a str, Error = E> {
        p
    }
}

impl<'a> Parser<'a, ()> for SP {
    type Input = &'a str;
    type Error = ParseError;
    fn name(&self, indent: &str) -> String {
        format!(
            "{indent}SP({input}) -> Result<(), {error}>",
            input = std::any::type_name::<Self::Input>(),
            error = std::any::type_name::<Self::Error>()
        )
    }

    fn parse(&mut self, inp: Self::Input) -> Result<(Self::Input,()), Self::Error> {
        Ok((inp,()))
    }
}

/// (a, (b,c)) -> (a,b,c)
/// (a, (b, (c,d))) ->

trait DeTuple {
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
        combo::{Parser, SP},
        prelude::dc::ParseError,
    };
    #[test]
    fn test_detuple() {
        println!("{:?}", (1, (2, 3, ())).detuple());
        println!("{:?}", (1, (2, (3, 4, ()))).detuple());
    }

    #[test]
    fn test_combo() {
        // define a simple lexer+parser
        assert_eq!(SP.parse("cat").unwrap().0, "cat");
        fn tail_parser<'a>(s: &'a str) -> Result<(&'a str, ()), ParseError> {
            Ok((&s[1..], ()))
        }

        fn ws<'a>(s: &'a str) -> Result<(&'a str, ()), ParseError> {
            Ok((s.trim_start(), ()))
        }

        fn num_parser<'a>(s: &'a str) -> Result<(&'a str, i32), ParseError> {
            Ok((&s[1..], s[0..=0].parse::<i32>()?))
        }
        #[derive(Default)]
        struct StructEx;

        let mut parser1 = SP
            .chain_lex(tail_parser)
            .chain_lex(ws)
            .chain_lex(tail_parser);

        let parser2 = SP
            .chain_lex(|s| tail_parser(s))
            .chain_lex(ws)
            .chain_lex(tail_parser);

        let s = parser1.parse("c   t2dog").unwrap();
        assert_eq!(s.0, "2dog");
        println!("{}", parser1.name(""));
        println!("{}", parser2.name(""));

        let mut parser3 = SP.make_parser(num_parser);
        println!("{}", parser3.name(""));

        let (s, d) = parser3.parse("5dog").unwrap();
        assert_eq!(s, "dog");
        assert_eq!(d, 5);

        let mut parser4 = parser3.chain_lex(parser1);
        let (s, d) = parser4.parse("6d   gs").unwrap();
        assert_eq!(s, "s");
        assert_eq!(d, 6);
        println!("{}", parser4.name(""));

        // let mut parser5 = parser4.chain_parser(num_parser);
        // let (s, d) = parser5.parse("6d   gs").unwrap();
        // assert_eq!(s, "s");
        // assert_eq!(d, 6);
        // println!("{}", parser4.name(""));
    }
}
