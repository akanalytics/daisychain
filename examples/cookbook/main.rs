#![allow(dead_code)]
// to produce the documentaion
// cargo doc --no-deps --examples
//

/*!
`daisychain` provides a library for parsing unicode text. It aims to have a gentle and intuitive API, without sacrificing performance (it can be zero-copy). Being a library, rather than a framework means that it can be used alongside and complement other parsing toolkits.

Main concepts:

- Cursor - represents
-- a point in the file/string being parsed
-- the concept of "selected text", which like an editor is a section highlighted
-- a sense of whether a parsing matching issue has arisen

- Parser -  a function which accepts a cursor/str and produces a result, along with a new cursor position

Typically methods on cursor are invoked in a chained fashion. If a matching issue arises,
subsequent methods has no effect (similar to repeatedly calling next() having reached the end of a fused rust iterator..)

Option<&str> is a simple Cursor (without the ability to select text), but is useful for test harnesses. None is used to represent a matching issue

In the examples below text() tries to match on the initial text of the &str, returning either the remaining &str or None (where the string doesnt match)
```
use daisychain::prelude::*;
assert_eq!(Some("Hello").text("He"), Some("llo"));
assert_eq!(Some("Hello").text("He").text("ll"), Some("o"));

// None indicates a match fail
assert_eq!(Some("Hello").text("Bye"), None);  

// after a fail, subsequent operations are ignored
assert_eq!(Some("Hello").text("Bye").text("He"), None);  

// text_alt matches on any of the alternatives
assert_eq!(Some("Hello").text_alt(&["Bye", "He"]), Some("llo"));
assert_eq!(Some("Hello").text_alt(&["He", "Hello"]), Some("llo"));

// "" will always match the beginning of the string
assert_eq!(Some("Hello").text_alt(&[""]), Some("Hello"));
```

*/

pub mod section_7_alternate;
pub mod section_9_parser_api;
pub mod section_3_binding_vars;
pub mod section_8_composition;
pub mod section_4_enum_handcrafted;
pub mod section_5_enum_strum;
pub mod section_6_enum_variant;
pub mod section_1_getting_started;
pub mod section_2_simple_example;

fn main() {}
