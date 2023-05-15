
// enum ParserFromClosure<P1, P2, S, T> {
//     TupleReturn(P1),
//     OutParam(P2),
//     Unused(Option<S>, Option<T>),
// }

// impl<P1, P2, S, T> ParserFromClosure<P1, P2, S, T>
// where
//     P1: FnMut(S) -> Result<(S, T), BadMatch>,
//     P2: FnMut(S, &mut Option<T>) -> Result<S, BadMatch>,
// {
//     fn invoke(mut self, s: S) -> Result<(S,T), BadMatch> {
//         match self {
//             Self::TupleReturn(mut p) => p(s),
//             Self::OutParam(mut p) => {
//                 let mut t = Option::<T>::None;
//                 let res = p(s, &mut t);
//                 let res = res.map( |s: S| (s, t.unwrap()) );
//                 res
//             }
//             _ => todo!(),
//         }
//     }
// }

// impl<'a, T, S, P> Parser<T, S> for ParserFromClosure<P, S, T>
// where
//     S: SelectableCursor<'a>,
//     P: FnMut(S) -> Result<(S, T), BadMatch>,
// {
//     fn parse(&mut self, s: S) -> Result<(S, T), BadMatch> {
//         self.invoke(s)
//     }
// }

// impl<'a, S, T, P> From<P> for ParserFromClosure<P, S, T>
// where
//     S: SelectableCursor<'a>,
//     P: FnMut(S) -> Result<(S, T), BadMatch>,
// {
//     fn from(p: P) -> Self {
//         ParserFromClosure::TupleReturn(p)
//     }
// }

// fn test_takes_parser<P1, P2, S, T>(s: S, p: impl Into<ParserFromClosure<P1, P2, S, T>>)
// where
//     P1: FnMut(S) -> Result<(S, T), BadMatch>,
//     P2: FnMut(S, &mut Option<T>) -> Result<S, BadMatch>,
// {
//     let _a = p.into().invoke(s);
// }

// #[test]
// fn test_generics() {
//     fn tp<'a>(s: Option<&'a str>) -> Result<&'a str, BadMatch> {
//         Ok(s.ws().unwrap())
//     }
//     fn tp2<'a>(s: &'a str, _d: &mut Option<f64>) -> Result<&'a str, BadMatch> {
//         Ok(Some(s).ws().unwrap())
//     }
//     test_takes_parser("", ParserFromClosure::OutParam(tp2));
// }

// impl<'a, T, S, P> Parser<T, S> for P where
// S: SelectableCursor<'a>,
// P: FnMut(S) -> Result<(S, T), BadMatch>
//  {
//     fn parse(&mut self, s: S) -> Result<(S,T), BadMatch> {
//         let mut p: P = self;
//         p(s)
//     }
// }

use std::marker::PhantomData;

use crate::{prelude::{Matchable, ParseError}, error};



pub struct Clipboard<'a, C> {
    // clip: &'a str,
    c:       C,
    phantom: PhantomData<&'a ()>,
}

// impl CaptureTo for i32 {}
impl<'a, C> Clipboard<'a, C>
where
    C: Matchable<'a>,
{
    fn new(c: C) -> Self {
        Self {
            c,
            phantom: PhantomData::default(),
        }
    }

    fn selected(&self) -> Result<&'a str, ParseError> {
        self.c.str()
    }

    pub fn as_i32(self) -> Result<i32, ParseError> {
        let mut dest = 0;
        self.put_i32(&mut dest).validate()?;
        Ok(dest)
    }

    pub fn put_i32(self, dest: &mut i32) -> C {
        if let Ok(s) = self.selected() {
            let t = s.parse::<i32>().map_err(|_e| error::failure("parse i32", s));
            if let Ok(t) = t {
                *dest = t;
            } else {
                return self.c.set_error(error::failure("parse i32", s));
            }
        }
        self.c
    }

    pub fn put_f64(self, dest: &mut f64) -> C {
        if let Ok(s) = self.selected() {
            let t = s.parse::<f64>().map_err(|_e| error::failure("parse f64", s));
            if let Ok(t) = t {
                *dest = t;
            } else {
                return self.c.set_error(error::failure("parse f64", s));
            }
        }
        self.c
    }

    pub fn put_str(self, dest: &mut &'a str) -> C {
        if let Ok(s) = self.selected() {
            *dest = s;
        }
        self.c
    }
}
