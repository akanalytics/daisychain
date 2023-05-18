use std::str::FromStr;

use daisychain::prelude::{
    *, {Cursor, ParseError},
};

use crate::{section_7_alternate::parse_clock, section_2_simple_example::Time};

///
///
///
///
#[derive(Debug, PartialEq)]
struct TimePeriod(Time, Time);

/// where items are easily tokenized because of fixed length or space separated,
/// often using FromStr to composite parsers works nicely
///
impl FromStr for TimePeriod {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_c, time1, time2) = Cursor::from(s)
            .chars_any(5..=5)
            .parse_selection() // uses .parse_selection::<Time>()
            .text("-")
            .chars_any(5..=5)
            .parse_selection() // uses .parse_selection::<Time>()
            .end_of_stream()
            .validate()?;
        Ok(TimePeriod(time1, time2))
    }
}

/// where the data is not easily lexed (tokenized appropriately) a
/// cursor/stream approach can be used.
///
/// eg a train timetable where UK uses AM/PM and continental trains use 24hour clock
///
///     "London Arrive 11:20 PM Depart 11:30 PM
///      Paris Arrive 13:05 Depart 13:10
///      Frankfurt Arrive 10:30 Depart 10:35"
///
/// This is not easly tokenized by fixed width columns or by whitespace separated words
///
///

#[derive(Debug, PartialEq)]
struct TrainTime {
    city: String,
    arr: Time,
    dep: Time,
}

///
/// parse_with expects a closure/function that matches
///  fn(Cursor) -> Result<(Cursor,T), ParseError>
///
fn parse_traintime(c: Cursor) -> Result<(Cursor, TrainTime), ParseError> {
    let (c, city, arr, dep) = c
        .word()
        .parse_selection()
        .ws()
        .text("Arrive")
        .ws()
        .parse_with(parse_clock) // free function accepted
        .ws()
        .text("Depart")
        .ws()
        .parse_with(|c| parse_clock(c)) // closure accepted
        .validate()?;
    Ok((c, TrainTime { city, arr, dep }))
}

fn parse_timetable(s: &str) -> Result<Vec<TrainTime>, ParseError> {
    let mut vec = vec![];
    for line in s.lines() {
        let c = Cursor::from(line);
        let (_c, tt) = c
            .parse_with(parse_traintime)
            .ws()
            .end_of_stream() // split by lines iterator means eos = end of this line
            .validate()?;
        vec.push(tt);
    }
    Ok(vec)
}



fn parse_str_clock(s: &str) -> Result<(&str, Time), ParseError> {
    let (c, time) = Cursor::from(s).parse_with(parse_clock).validate()?;
    Ok((c.str()?, time))
}

fn parse_str_traintime(c: &str) -> Result<(&str, TrainTime), ParseError> {
    let (c, city, arr, dep) = Cursor::from(c)
        .word()
        .parse_selection()
        .ws()
        .text("Arrive")
        .ws()
        .parse_with_str(parse_str_clock) // free function accepted
        .ws()
        .text("Depart")
        .ws()
        .parse_with_str(|c| parse_str_clock(c)) // closure accepted
        .validate()?;
    Ok((c.str()?, TrainTime { city, arr, dep }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_time_period() {
        let tp: TimePeriod = ("13:00-14:30").parse().unwrap();
        assert_eq!(tp, TimePeriod(Time::new(13, 00), Time::new(14, 30)));
    }

    #[test]
    fn test_traintime() {
        let c = Cursor::from("London Arrive 11:20 PM Depart 11:30 PM");
        let (c, tt) = parse_traintime(c).unwrap();
        assert_eq!(
            tt,
            TrainTime {
                city: "London".to_string(),
                arr: Time::new(23, 20),
                dep: Time::new(23, 30)
            }
        );
        assert_eq!(c.str().unwrap(), "");
    }

    #[test]
    fn test_timetable() {
        let s = "London Arrive 11:20 PM Depart 11:30 PM\nParis Arrive 13:05 Depart 13:10\nFrankfurt Arrive 10:30 Depart 10:35";
        let table = parse_timetable(s).unwrap();
        assert_eq!(table.len(), 3);
        assert_eq!(table[2].city, "Frankfurt");
    }
}
