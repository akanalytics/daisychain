use std::{error::Error, fmt};

#[derive(Debug)]
pub enum ParseError {
    Fatal(Option<Box<dyn Error>>),
    NoMatch {
        action: &'static str,
        args: &'static str,
    },
}

impl Clone for ParseError {
    #[inline]
    fn clone(&self) -> Self {
        match self {
            Self::Fatal(_e) => Self::Fatal(None),
            Self::NoMatch { action, args } => Self::NoMatch { action, args },
        }
    }
}

#[inline]
pub fn failure(action: &'static str, _args: &str) -> ParseError {
    ParseError::NoMatch { action, args: "" }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Fatal(e) => write!(
                f,
                "Fatal:{msg}",
                msg = e.as_ref().map(|e| e.to_string()).unwrap_or_default()
            )?,
            Self::NoMatch { action, args } => {
                write!(f, "FailedMatch: (action='{action}' args='{args}')")?
            }
        };
        Ok(())
    }
}
impl std::error::Error for ParseError {}
