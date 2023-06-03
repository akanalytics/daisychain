// use crate::prelude::Matchable;

// pub fn formatter<'a, C: Matchable<'a>>(c: &C) -> String {
//     let s = c.str().unwrap_or_default();
//     formatter_str(s)
// }

pub fn formatter_str(c: &str) -> String {
    let s = c[..c.len().min(33)].escape_default().to_string();
    let s = s.replace("\\\"", "\"");
    let s = s.replace("\\\'", "\'");
    let s = &s[..s.len().min(33)];
    format!("{:<35}", "|".to_string() + &s + &"|")
}
