use std::unreachable;

use daisychain::prelude::*;

use crate::ch_2_simple_example::Time;

/// where different text formats represent different data types, an enum variant is appropriate
/// 
/// where the same data type presents in different formats, simple aternation can be used to try
/// and match against different parsers. Cursor's can be cloned to save the position for re-parsing

pub fn parse_clock(c: dc::Cursor) -> Result<(dc::Cursor, Time), dc::ParseError> {
    let (c1, time) = c.chars_any(5..=5).parse_selection::<Time>().validate()?;

    // Cursor methods move out of the cursor they are called on.
    // Because we might return the cursor 'c1' if parsing AM/PM doesnt succeed,
    // we need to clone 'c1' first
    if let Ok((c2, ampm)) = c1
        .clone()
        .debug_context("clock")
        .ws()
        .text_alt(&["am", "AM", "pm", "PM"])
        .parse_selection_as_str() // explicit method as &str doesnt impl FromStr
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

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_parse_clock() -> Result<(), dc::ParseError> {
        assert_eq!(parse_clock("11:35 AM".into())?.1, Time::new(11, 35));
        assert_eq!(parse_clock("11:59 PM".into())?.1, Time::new(23, 59));
        assert_eq!(parse_clock("01:59".into())?.1, Time::new(1, 59));
        assert_eq!(parse_clock("1:59".into()).is_err(), true);

        // check that the cursor is left at the right terminating position
        assert_eq!(parse_clock("11:35 AM X".into())?.0.str()?, " X");
        assert_eq!(parse_clock("11:59 PM Y".into())?.0.str()?, " Y");
        assert_eq!(parse_clock("01:59 Z".into())?.0.str()?, " Z");
        Ok(())
    }
}
