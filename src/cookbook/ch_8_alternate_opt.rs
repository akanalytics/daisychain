use crate::cookbook::ch_2_simple_example::Time;
use crate::prelude::dc::{Cursor, ParseError};
use crate::prelude::*;
use strum::VariantNames;
use strum_macros::{EnumString, EnumVariantNames};

use super::ch_7_alternate::parse_time;

#[derive(PartialEq, Debug, EnumVariantNames, EnumString)]
pub enum Day {
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
}

/// eg
///   "13:52"
///   "Wed 13:52"
///   "Wed"
///
#[derive(PartialEq, Debug)]
pub enum Event {
    DayTime(Day, Time),
    TimeOnly(Time),
    DayOnly(Day),
}

pub fn parse_day(s: &str) -> Result<(&str, Day), ParseError> {
    let (c, day) = Cursor::from(s)
        .text_alt(Day::VARIANTS)
        .parse_selection()
        .validate()?;
    Ok((c.str()?, day))
}

pub fn parse_event(s: &str) -> Result<(&str, Event), ParseError> {
    let (c1, opt_day, opt_time) = Cursor::from(s)
        .parse_opt_with(parse_day)
        .ws()
        .parse_opt_with(parse_time)
        .ws()
        .validate()?;

    let s = c1.str()?;
    match (opt_day, opt_time) {
        (Some(d), Some(t)) => Ok((s, Event::DayTime(d, t))),
        (None, Some(t)) => Ok((s, Event::TimeOnly(t))),
        (Some(d), None) => Ok((s, Event::DayOnly(d))),
        (None, None) => Result::Err(ParseError::NoMatch {
            action: "Must specify day or time (or both)",
            args: "",
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_event() -> Result<(), ParseError> {
        use Event::*;
        let t = Time::new(11, 35);
        assert_eq!(parse_event("11:35")?.1, TimeOnly(t));
        assert_eq!(parse_event("Wed 11:35")?.1, DayTime(Day::Wed, t));
        assert_eq!(parse_event("Wed")?.1, DayOnly(Day::Wed));
        assert_eq!(parse_event("X").is_err(), true);

        // check that the cursor is left at the right terminating position
        assert_eq!(parse_event("11:35 X")?.0, "X");
        assert_eq!(parse_event("Wed 11:59 Y")?.0, "Y");
        assert_eq!(parse_event("Wed Z")?.0, "Z");
        Ok(())
    }
}
