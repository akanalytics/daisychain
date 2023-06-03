#![allow(dead_code)]
/*!
# DaisyChain
- provides a library for parsing unicode text
-  aims to have a gentle and intuitive API, without sacrificing performance (it can be zero-copy)
- as library, rather than a framework, it can be used alongside and complement other parsing toolkits


Main concepts:

# Cursor 
represents:
- a point in the file/string being parsed
- the concept of "selected text", which like a text-editor, is a highlighted section of text
- a sense of whether parse matching has succeeded or not

Logically , one could image a Cursor as a structure with

```unknown
  usize: the index into the parsed text/the current curosr position
  &str: a reference to the highlighted selected text
  bool: whether matching has failed
```

`The quick brown fox jumps over the lazy dog`<br>
`    ===============       ^`

In the example above the cursor has position at the `o` in over, with "quick bown fox" being the selected text.

Typically methods on cursor are invoked in a chained fashion. If a matching issue arises,
subsequent methods has no effect (similar to repeatedly calling next() having reached the end of a fused rust iterator..)

```
use daisychain::prelude::*;
use daisychain::prelude::Cursor;

let _ = Cursor::from("The quick brown fox jumps over the lazy dog")
    .find("quick")
    .selection_start()
    .word()
    .word()
    .word()
    .selection_end()
    .find("over");
```



# Parser
Parsers are "functions" that take text and produce structured data.

Different styles are encouraged.

- an implmentation of the trait FromStr which accepts a &str and produces some parsed data inside a Result

OR
- a function accepting a &str (or Cursor) and produces a Result containing a new cursor position, along with some parsed data
- parsers can be free-functions, associated-functions or closures



A composition pattern is typical, with small parsers handling constituent objects, being invoked by parsers of larger structs in turn


# Testing

For test harnesses and experimentation, Option<&str> is a simple Cursor (without the ability to select text), but is useful for test harnesses. 
None is used to represent a matching issue. 

For more substantial tests during development of your parsers, using a logging framework for test-harnesses is encoraged

```toml
[dev-dependencies]
env_logger = "0.9"
test-log = {version = "0.2"}
```

This will allow the action of parsing to be traced with
```sh
RUST_LOG=dc=trace cargo test mytest -- --nocapture
```

*/

pub mod ch_1_getting_started;
pub mod ch_2_simple_example;
pub mod ch_3_binding_vars;
pub mod ch_4_enum_handcrafted;
pub mod ch_5_enum_strum;
pub mod ch_6_enum_variant;
pub mod ch_7_alternate;
pub mod ch_8_alternate_opt;
pub mod ch_8_composition;
pub mod ch_9_parser_api;

// fn main() {}
