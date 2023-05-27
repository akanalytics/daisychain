pub use crate::text_parser::{Bind, Matchable, Selectable};

pub mod dc {
    pub use crate::cursor::Cursor;
    pub use crate::error::ParseError;
}


pub mod lazy {
    pub use crate::combo::SP;
    pub use crate::combo::Parser;
    pub use crate::combo::StrParser;
}