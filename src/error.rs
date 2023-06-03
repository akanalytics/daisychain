use std::{
    convert::Infallible,
    error::Error,
    fmt, matches,
    num::{ParseFloatError, ParseIntError},
    str::ParseBoolError,
};

/// Indicates whether an error can be recovered from, and parsing can continue.
/// Errors such as "config file not found" in parse functions are likely fatal and
/// should be flagged non-recoverable
pub trait Recoverable {
    fn is_recoverable(&self) -> bool;
}

#[derive(Debug)]
pub enum ParsingError {
    Fatal(Option<Box<dyn Error>>),
    NoMatch {
        action: &'static str,
        args: &'static str,
    },
}
impl Recoverable for ParsingError {
    fn is_recoverable(&self) -> bool {
        matches!(self, Self::NoMatch { .. })
    }
}

impl Default for ParsingError {
    fn default() -> Self {
        Self::NoMatch {
            action: "",
            args: "",
        }
    }
}

impl From<ParseIntError> for ParsingError {
    fn from(_value: ParseIntError) -> Self {
        ParsingError::NoMatch {
            action: "parse int error",
            args: "",
        }
    }
}

impl From<ParseFloatError> for ParsingError {
    fn from(_value: ParseFloatError) -> Self {
        ParsingError::NoMatch {
            action: "parse float error",
            args: "",
        }
    }
}

impl From<ParseBoolError> for ParsingError {
    fn from(_value: ParseBoolError) -> Self {
        ParsingError::NoMatch {
            action: "parse bool error",
            args: "",
        }
    }
}

impl From<Infallible> for ParsingError {
    fn from(_value: Infallible) -> Self {
        unreachable!()
    }
}

impl Clone for ParsingError {
    #[inline]
    fn clone(&self) -> Self {
        match self {
            Self::Fatal(_e) => Self::Fatal(None),
            Self::NoMatch { action, args } => Self::NoMatch { action, args },
        }
    }
}

#[inline]
pub fn failure(action: &'static str, _args: &str) -> ParsingError {
    ParsingError::NoMatch { action, args: "" }
}

impl fmt::Display for ParsingError {
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
impl std::error::Error for ParsingError {}
