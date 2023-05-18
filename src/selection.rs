use std::fmt;

use crate::util;

#[derive(Debug, Clone, PartialEq)]
pub enum Selection<'a> {
    Defaulted(&'a str),
    Start(&'a str, Option<&'a str>),
    Last(&'a str, &'a str),
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
