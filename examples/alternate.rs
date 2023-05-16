use chainsaw::prelude::*;

#[derive(PartialEq, Debug)]
enum AmPm {
    AM,
    PM,
}

#[derive(PartialEq, Debug)]
enum Clock {
    H12(u32, u32, AmPm), // "11:59 AM", "2:20 PM"
    H24(u32, u32),       // "14:30"
}

fn parse_clock(c: cs::Cursor) -> Result<(cs::Cursor, Clock), cs::ParseError> {
    let (c1, h, m) = c
        .digits(1..=2)
        .parse_selection()?
        .text(":")
        .digits(2..=2)
        .parse_selection()?
        .validate_new()?;

    // Cursor methods move out of the cursor they are called on.
    // Because we might return the cursor 'c' if parsing AM/PM doesnt succeed,
    // we need to clone 'c1' first
    //
    // Also, we explicitly mention "as_str" in parse_selection,
    // as &str doesnt impl FromStr
    //
    // no '?' after the parse_selection as we are if let matching on the error
    //
    if let Ok((c2, ampm)) = c1
        .clone()
        .ws()
        .text_alt(&["am", "AM", "pm", "PM"])
        .parse_selection_as_str()
    {
        match ampm.to_lowercase().as_str() {
            "am" => return Ok((c2, Clock::H12(h, m, AmPm::AM))),
            "pm" => return Ok((c2, Clock::H12(h, m, AmPm::PM))),
            _ => {}
        }
    }
    // if we haven't matched on am/pm, return the cursor at c1
    // - c2 is a compile error as c2 not in scope,
    // - c is a compile error as method chains move out of c
    Ok((c1, Clock::H24(h, m)))
}

// improved version disallows "1:59" as 24H, and insists on "01:59"
fn parse_clock_v2(c: cs::Cursor) -> Result<(cs::Cursor, Clock), cs::ParseError> {
    if let Ok((c1, h, m, ampm)) = c
        .clone()
        .digits(1..=2)
        .parse_selection()?
        .text(":")
        .digits(2..=2)
        .parse_selection()?
        .ws()
        .text_alt(&["am", "AM", "pm", "PM"])
        .parse_selection_as_str()?
        .validate_new()
    {
        match ampm.to_lowercase().as_str() {
            "am" => return Ok((c1, Clock::H12(h, m, AmPm::AM))),
            "pm" => return Ok((c1, Clock::H12(h, m, AmPm::PM))),
            _ => {}
        }
    }

    // parse the alternative representation using cursor at 'c'
    let (c2, h, m) = c
        .digits(2..=2) // mandate 2 digits
        .parse_selection()?
        .text(":")
        .digits(2..=2)
        .parse_selection()?
        .validate_new()?;
    Ok((c2, Clock::H24(h, m)))
}

fn main() {
    let _ = parse_clock(cs::Cursor::from("12:23 AM"));
    let _ = parse_clock_v2(cs::Cursor::from("01:59"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_parse_clock() -> Result<(), cs::ParseError> {
        use AmPm::*;
        use Clock::*;
        assert_eq!(parse_clock("11:35 AM X".into()).unwrap().1, H12(11, 35, AM));
        assert_eq!(parse_clock("11:59 PM Y".into()).unwrap().1, H12(11, 59, PM));
        assert_eq!(parse_clock("1:59Z".into()).unwrap().1, H24(1, 59));
        assert_eq!(parse_clock_v2("1:59Z".into()).is_err(), true);
        assert_eq!(parse_clock("01:59Z".into()).unwrap().1, H24(1, 59));

        // check that the cursor is left at the right terminating position
        assert_eq!(parse_clock("11:35 AM X".into()).unwrap().0.str()?, " X");
        assert_eq!(parse_clock("11:59 PM Y".into()).unwrap().0.str()?, " Y");
        assert_eq!(parse_clock("01:59 Z".into()).unwrap().0.str()?, " Z");
        Ok(())
    }
}
