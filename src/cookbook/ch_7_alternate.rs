use std::unreachable;

use crate::cookbook::ch_2_simple_example::Time;
use crate::prelude::*;

pub fn parse_time(s: &str) -> Result<(&str, Time), ParsingError> {
    Cursor::from(s)
        .chars_any(5..=5)
        .parse_selection::<Time>()
        .validate()
}

/// where different text formats represent different data types, an enum variant is appropriate
///
/// where the same data type presents in different formats, simple aternation can be used to try
/// and match against different parsers. Cursor's can be cloned to save the position for re-parsing
pub fn parse_clock(s: &str) -> Result<(&str, Time), ParsingError> {
    let (c1, time) = Cursor::from(s)
        .chars_any(5..=5)
        .parse_selection::<Time>()
        .ws()
        .validate()?;

    // Cursor methods move out of the cursor they are called on.
    // Because we might return the cursor 'c1' if parsing AM/PM doesnt succeed,
    // we need to clone 'c1' first
    if let Ok((c2, ampm)) = Cursor::from(c1)
        .clone()
        .text_alt(&["AM", "PM"])
        .parse_selection_as_str() // explicit method as &str doesnt impl FromStr
        .ws()
        .validate()
    {
        let time = match ampm.to_lowercase().as_str() {
            "am" => time,
            "pm" => Time::new(time.hours + 12, time.mins),
            _ => unreachable!(),
        };
        return Ok((c2, time)); // assume 24 hour clock
    }
    // if we haven't matched on am/pm, return the cursor at c1
    // - c2 is a compile error as c2 not in scope,
    // - c is a compile error as method chains move out of c
    Ok((c1, time))
}

// sometimes if we don't care too much about capturing/not-capturing ws around an optional field,
// its easiest to alt_match on "" as final choice for an optional field
pub fn parse_clock_v2(c: &str) -> Result<(&str, Time), ParsingError> {
    let (c1, time, ampm) = Cursor::from(c)
        .debug_context("clockv2")
        .chars_any(5..=5)
        .parse_selection::<Time>()
        .ws()
        .text_alt(&["AM", "PM", ""]) // always match
        .parse_selection_as_str()
        .ws()
        .validate()?;

    {
        let time = match ampm {
            "AM" | "" => time,
            "PM" => Time::new(time.hours + 12, time.mins),
            _ => unreachable!(),
        };
        return Ok((c1, time)); // assume 24 hour clock
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_parse_clock() -> Result<(), ParsingError> {
        assert_eq!(parse_clock("11:35 AM".into())?.1, Time::new(11, 35));
        assert_eq!(parse_clock("11:59 PM".into())?.1, Time::new(23, 59));
        assert_eq!(parse_clock("01:59".into())?.1, Time::new(1, 59));
        assert_eq!(parse_clock("1:59".into()).is_err(), true);

        // check that the cursor is left at the right terminating position
        assert_eq!(parse_clock("11:35 AM X".into())?.0, "X");
        assert_eq!(parse_clock("11:59 PM Y".into())?.0, "Y");
        assert_eq!(parse_clock("01:59 Z".into())?.0, "Z");
        Ok(())
    }

    #[test]
    fn test_parse_clock_v2() -> Result<(), ParsingError> {
        assert_eq!(parse_clock_v2("11:35 AM".into())?.1, Time::new(11, 35));
        assert_eq!(parse_clock_v2("11:59 PM".into())?.1, Time::new(23, 59));
        assert_eq!(parse_clock_v2("01:59".into())?.1, Time::new(1, 59));
        assert_eq!(parse_clock_v2("1:59".into()).is_err(), true);

        // check that the cursor is left at the right terminating position
        assert_eq!(parse_clock_v2("11:35 AM X".into())?.0, "X");
        assert_eq!(parse_clock_v2("11:59 PM Y".into())?.0, "Y");
        assert_eq!(parse_clock_v2("01:59 Z".into())?.0, "Z");
        Ok(())
    }
}
