// #![cfg_attr(debug_assertions, allow(dead_code))]
#![allow(dead_code)]
#![warn(clippy::all)]
#![warn(clippy::correctness)]
#![warn(clippy::style)]
#![warn(clippy::complexity)]
#![warn(clippy::perf)]
#![allow(mixed_script_confusables)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::bool_assert_comparison)]



use std::cell::Cell;


mod contrib;
mod logging;
mod parser;
mod cursor;
mod error;
mod text_parser;
mod util;
mod combo;

pub mod prelude;

#[cfg(any(feature="cookbook", doc))] 
pub mod cookbook;

pub(crate) const LOG_TARGET: &str = "dc"; // env!("CARGO_PKG_NAME");

thread_local!(pub(crate) static LABEL: Cell<&'static str> = Cell::new(""));

