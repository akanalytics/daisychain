use chainsaw::prelude::*;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
struct Time {
    hours: u32,
    mins: u32,
}

impl Time {
    pub fn new(hours: u32, mins: u32) -> Self {
        Self { hours, mins }
    }
}

impl FromStr for Time {
    type Err = cs::ParseError;

    /// eg "09:23" or "23:59"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_cur, hours, mins) = cs::Cursor::from(s)
            .digits(2..=2)
            .parse_selection::<u32>() // chainsaw will use u32::FromStr
            .text(":")
            .digits(2..=2)
            .parse_selection() // often no need to specify type explicitly
            .end_of_stream() // ensure we are at end-of-string
            .validate()?;
        Ok(Time { hours, mins })
    }
}

fn main() {
    let _ = Time::new(0, 0);
}

#[cfg(test)]
mod tests {

    use super::*;
    use test_log::test;

    #[test]
    fn test_parse_0923() {
        assert_eq!(Time::from_str("09:23").unwrap(), Time::new(9, 23));
        assert!(Time::from_str("09+23").is_err());
        assert!(Time::from_str("09:23X").is_err());
    }

    #[test]
    fn test_parse_three_times() -> Result<(), cs::ParseError> {
        let s = "09:23 11:45 23:59";
        let valid_chars: Vec<_> = "0123456789:".chars().collect();
        let valid_chars = valid_chars.as_slice();

        let (_c, t1, t2, t3) = cs::Cursor::from(s)
            .chars_in(1.., valid_chars)
            .parse_selection::<Time>() // use the Time::FromStr we've just defined
            .ws()
            .chars_in(1.., valid_chars)
            .parse_selection::<Time>()
            .ws()
            .chars_in(1.., valid_chars)
            .parse_selection::<Time>()
            .validate()?;
        assert_eq!(t1, Time::new(9, 23));
        assert_eq!(t2, Time::new(11, 45));
        assert_eq!(t3, Time::new(23, 59));
        Ok(())
    }
}
