//! Parser for fonts in Web Open Font Format.
//!
//! ## Example
//!
//! ```
//! use webtype::opentype::truetype::tables::FontHeader;
//! use webtype::File;
//!
//! macro_rules! ok(($result:expr) => ($result.unwrap()));
//!
//! let path = "NotoNaskhArabic-Regular.woff2";
//! # let path = "tests/fixtures/NotoNaskhArabic-Regular.woff2";
//! let mut tape = ok!(std::fs::File::open(path));
//! let File { mut fonts, mut tape } = ok!(File::read(&mut tape));
//!
//! let font_header = ok!(ok!(fonts[0].take::<_, FontHeader>(&mut tape)));
//! assert_eq!(font_header.units_per_em, 2048);
//! ```

pub extern crate opentype;

#[macro_use(dereference, error, raise, table)]
extern crate typeface;

pub mod version1;
pub mod version2;

mod file;
mod number;

pub use opentype::Font;
pub use typeface::{tape, value, walue, Error, Result};

pub use file::File;
pub use number::v32;

/// Check if a tag is recognized.
#[inline]
pub fn accept(tag: &opentype::truetype::Tag) -> bool {
    matches!(&tag.0, b"wOFF" | b"wOF2")
}
