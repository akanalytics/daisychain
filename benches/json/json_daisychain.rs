use std::collections::HashMap;

use daisychain::prelude::dc::{Cursor, ParseError};
use daisychain::prelude::*;

use crate::JsonValue;

pub fn daisychain_parser(s: &str) -> JsonValue {
    root(s).unwrap().1
}

fn boolean(s: &str) -> Result<(&str, JsonValue), ParseError> {
    if let Ok((c, boolean)) = Cursor::from(s)
        .debug_context("boolean")
        .text_alt(&["true", "false"])
        .parse_selection()
        .validate()
    {
        Ok((c, JsonValue::Boolean(boolean)))
    } else {
        Err(ParseError::default())
    }
}

fn double(s: &str) -> Result<(&str, JsonValue), ParseError> {
    if let Ok((c, float64)) = Cursor::from(s)
        .debug_context("double")
        .chars_in(
            1..,
            &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.'],
        )
        .parse_selection()
        .validate()
    {
        Ok((c, JsonValue::Num(float64)))
    } else {
        Err(ParseError::default())
    }
}

fn null(s: &str) -> Result<(&str, JsonValue), ParseError> {
    if let Ok(c) = Cursor::from(s).text("null").validate() {
        Ok((c, JsonValue::Null))
    } else {
        Err(ParseError::default())
    }
}

fn string(s: &str) -> Result<(&str, JsonValue), ParseError> {
    if let Ok((c, s)) = Cursor::from(s)
        .debug_context("string")
        .ws()
        .text("\"")
        .chars_not_in(0.., &['"'])
        .parse_selection()
        .ws()
        .validate()
    {
        Ok((c, JsonValue::Str(s)))
    } else {
        Err(ParseError::default())
    }
}

fn array(s: &str) -> Result<(&str, JsonValue), ParseError> {
    if let Ok((c, v)) = Cursor::from(s)
        .debug_context("array")
        .ws()
        .text("[")
        // .chars_not_in(0.., &[']'])
        .parse_struct_vec(|s| Cursor::from(s).parse_with(json_value).maybe(",").validate())
        .text("]")
        .ws()
        .validate()
    {
        Ok((c, JsonValue::Array(v)))
    } else {
        Err(ParseError::default())
    }
}

fn key_value(s: &str) -> Result<(&str, (&str, JsonValue)), ParseError> {
    if let Ok((c, k, v)) = Cursor::from(s)
        .ws()
        .chars_not_in(0.., &[':'])
        .parse_selection_as_str()
        .ws()
        .text(":")
        .ws()
        .parse_with(json_value)
        .validate()
    {
        Ok((c, (k, v)))
    } else {
        Err(ParseError::default())
    }
}

fn hash(s: &str) -> Result<(&str, JsonValue), ParseError> {
    if let Ok((c, vec)) = Cursor::from(s)
        .debug_context("hash")
        .ws()
        .text("{")
        .parse_struct_vec(|s| Cursor::from(s).parse_with(key_value).maybe(",").validate())
        .ws()
        .text("}")
        .ws()
        .validate()
    {
        let map: HashMap<String, JsonValue> =
            vec.into_iter().map(|(k, v)| (k.to_owned(), v)).collect();
        Ok((c, JsonValue::Object(map)))
    } else {
        Err(ParseError::default())
    }
}

fn json_value(s: &str) -> Result<(&str, JsonValue), ParseError> {
    if let Ok((c, jv)) = Cursor::from(s).parse_with(hash).validate() {
        Ok((c, jv))
    } else if let Ok((c, jv)) = Cursor::from(s).parse_with(array).validate() {
        Ok((c, jv))
    } else if let Ok((c, jv)) = Cursor::from(s).parse_with(string).validate() {
        Ok((c, jv))
    } else if let Ok((c, jv)) = Cursor::from(s).parse_with(double).validate() {
        Ok((c, jv))
    } else if let Ok((c, jv)) = Cursor::from(s).parse_with(boolean).validate() {
        Ok((c, jv))
    } else {
        Cursor::from(s).parse_with(null).validate()
    }
}

fn root(s: &str) -> Result<(&str, JsonValue), ParseError> {
    if let Ok((c, jv)) = Cursor::from(s).parse_with(hash).validate() {
        Ok((c, jv))
    } else if let Ok((c, jv)) = Cursor::from(s).parse_with(array).validate() {
        Ok((c, jv))
    } else {
        Cursor::from(s).parse_with(null).validate()
    }
}
