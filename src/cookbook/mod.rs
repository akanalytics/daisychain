#![allow(dead_code)]
// to produce the documentaion
// cargo doc --no-deps --examples
//

/*!
`daisychain` 
- provides a library for parsing unicode text
-  aims to have a gentle and intuitive API, without sacrificing performance (it can be zero-copy)
- as library, rather than a framework, it can be used alongside and complement other parsing toolkits


Main concepts:

# Cursor - represents
- a point in the file/string being parsed
- the concept of "selected text", which like an editor is a section highlighted
- a sense of whether a parsing matching issue has arisen

# Parser -  a function which accepts a cursor/str and produces a result, along with a new cursor position

Typically methods on cursor are invoked in a chained fashion. If a matching issue arises,
subsequent methods has no effect (similar to repeatedly calling next() having reached the end of a fused rust iterator..)

Option<&str> is a simple Cursor (without the ability to select text), but is useful for test harnesses. None is used to represent a matching issue

*/

pub mod ch_1_getting_started;
pub mod ch_2_simple_example;
pub mod ch_3_binding_vars;
pub mod ch_4_enum_handcrafted;
pub mod ch_5_enum_strum;
pub mod ch_6_enum_variant;
pub mod ch_7_alternate;
pub mod ch_8_composition;
pub mod ch_9_parser_api;

fn main() {}
