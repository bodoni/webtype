//! Parser for fonts in Web Open Font Format.

pub extern crate opentype;

#[macro_use(table)]
extern crate typeface;

pub mod header;

pub use typeface::{Error, Result, Tape, Value, Walue};
