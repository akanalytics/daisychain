// use crate::prelude::Matchable;

// pub fn formatter<'a, C: Matchable<'a>>(c: &C) -> String {
//     let s = c.str().unwrap_or_default();
//     formatter_str(s)
// }

pub fn formatter_str(c: &str) -> String {
    format!("{:<20}", c[..c.len().min(20)].escape_default().to_string())
}
