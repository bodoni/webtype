//! Parser for fonts in Web Open Font Format.

pub extern crate opentype;

#[macro_use(table)]
extern crate typeface;

pub mod version1;
pub mod version2;

pub use typeface::{Error, Result, Tape, Value, Walue};

/// A variable-size u32.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Default)]
pub struct v32(u32);

impl Value for v32 {
    fn read<T: Tape>(_: &mut T) -> Result<Self> {
        let value = 0;
        Ok(v32(value))
    }
}
