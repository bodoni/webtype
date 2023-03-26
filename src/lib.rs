//! Parser for fonts in Web Open Font Format.

pub extern crate opentype;

#[macro_use(dereference, error, raise, table)]
extern crate typeface;

pub mod version1;
pub mod version2;

mod number;

pub use typeface::{Error, Result, Tape, Value, Walue};

pub use number::v32;
