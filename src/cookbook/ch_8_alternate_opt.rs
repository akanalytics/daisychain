use crate::cookbook::ch_2_simple_example::Time;
use crate::prelude::*;
use strum::VariantNames;
use strum_macros::{EnumString, EnumVariantNames};

use super::ch_7_alternate::parse_time;

#[derive(PartialEq, Debug)]
pub enum OneChar {
    Digit(u32),
    Letter(char),
}

pub fn parse_char(s: &str) -> Result<(&str, OneChar), ParsingError> {
    let (c, opt_digit, opt_letter) = Cursor::from(s)
        .chars_any(1..=1)
        .parse_opt_selection::<u32>()
        .parse_opt_selection::<char>()
        .validate()?;
    let one_char = match (opt_digit, opt_letter) {
        // order important, and note "_" since a digit also a letter
        (Some(d), _) => OneChar::Digit(d), 
        (None, Some(c)) => OneChar::Letter(c),
        _ => unreachable!(),
    };

    Ok((c, one_char))
}

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

pub fn parse_day(s: &str) -> Result<(&str, Day), ParsingError> {
    let (c, day) = Cursor::from(s)
        .text_alt(Day::VARIANTS)
        .parse_selection()
        .validate()?;
    Ok((c, day))
}

pub fn parse_event(s: &str) -> Result<(&str, Event), ParsingError> {
    let (c1, opt_day, opt_time) = Cursor::from(s)
        .parse_opt_with(parse_day)
        .ws()
        .parse_opt_with(parse_time)
        .ws()
        .validate()?;

    match (opt_day, opt_time) {
        (Some(d), Some(t)) => Ok((c1, Event::DayTime(d, t))),
        (None, Some(t)) => Ok((c1, Event::TimeOnly(t))),
        (Some(d), None) => Ok((c1, Event::DayOnly(d))),
        (None, None) => Result::Err(ParsingError::NoMatch {
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
    fn test_onechar() -> Result<(), ParsingError> {
        use OneChar::*;
        assert_eq!(parse_char("1")?.1, Digit(1));
        assert_eq!(parse_char("A")?.1, Letter('A'));
        assert!(parse_char("").is_err());

        // check that the cursor is left at the right terminating position
        assert_eq!(parse_char("1X")?.0, "X");
        assert_eq!(parse_char("AY")?.0, "Y");
        Ok(())
    }

    #[test]
    fn test_event() -> Result<(), ParsingError> {
        use Event::*;
        let t = Time::new(11, 35);
        assert_eq!(parse_event("11:35")?.1, TimeOnly(t));
        assert_eq!(parse_event("Wed 11:35")?.1, DayTime(Day::Wed, t));
        assert_eq!(parse_event("Wed")?.1, DayOnly(Day::Wed));
        assert!(parse_event("X").is_err());

        // check that the cursor is left at the right terminating position
        assert_eq!(parse_event("11:35 X")?.0, "X");
        assert_eq!(parse_event("Wed 11:59 Y")?.0, "Y");
        assert_eq!(parse_event("Wed Z")?.0, "Z");
        Ok(())
    }
}
