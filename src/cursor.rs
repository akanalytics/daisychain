use std::fmt;

use crate::{prelude::dc::ParseError, util};
use crate::logging::Loggable;

#[derive(Debug, Clone)]
pub struct Cursor<'a> {
    pub(crate) selection: Selection<'a>,
    pub(crate) cur: Option<&'a str>,
    pub(crate) err: Option<ParseError>,
    pub(crate) context: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Selection<'a> {
    Defaulted(&'a str),
    Start(&'a str, Option<&'a str>),
    Last(&'a str, &'a str),
}

// equal and error free
impl<'a> PartialEq for Cursor<'a> {
    #[allow(clippy::match_like_matches_macro)]
    fn eq(&self, other: &Self) -> bool {
        self.selection == other.selection
            && self.cur == other.cur
            && self.context == other.context
            && match (&self.err, &other.err) {
                (None, None) => true,
                _ => false,
            }
    }
}

impl<'a> From<&'a str> for Cursor<'a> {
    #[inline]
    fn from(s: &'a str) -> Self {
        let cur = Self {
            selection: Selection::Defaulted(s),
            cur: Some(s),
            err: None,
            context: "",
        };
        cur.log_success("Cursor::from", "");
        cur
    }
}

impl<'a> fmt::Display for Selection<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Defaulted(s) => write!(f, "Defaulted({})", util::formatter_str(s))?,
            Self::Start(s, opt_t) => write!(
                f,
                "Start('{}', '{}')",
                util::formatter_str(s),
                util::formatter_str(opt_t.unwrap_or_default())
            )?,
            Self::Last(s, e) => write!(
                f,
                "Last('{}','{}')",
                util::formatter_str(s),
                util::formatter_str(e)
            )?,
        };
        Ok(())
    }
}

impl<'a> Selection<'a> {
    pub fn start(&self) -> &'a str {
        match self {
            Selection::Defaulted(s) => s,
            Selection::Start(s, _) => s,
            Selection::Last(s, _) => s,
        }
    }

    pub fn move_cursor(self, to: &'a str) -> Self {
        match self {
            Selection::Defaulted(s) => Selection::Last(s, to),
            Selection::Start(..) => self,
            Selection::Last(_s, e) => Selection::Last(e, to),
        }
    }

    pub fn selection(&self, cur: &'a str) -> (&'a str, &'a str) {
        match self {
            Selection::Defaulted(s) => (s, cur),
            Selection::Start(s, opt_e) => (s, opt_e.unwrap_or(cur)),
            Selection::Last(s, e) => (s, e),
        }
    }
}
