//! Parser for fonts in Web Open Font Format.

pub extern crate opentype;

#[macro_use(table)]
extern crate typeface;

pub mod version1;
pub mod version2;

pub use typeface::{Error, Result, Tape, Value, Walue};
