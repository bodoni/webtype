//! The [Web Open Font Format][1] of version 2.0.
//!
//! [1]: https://www.w3.org/TR/WOFF2/

pub mod table_directory;

mod file_header;

pub use file_header::FileHeader;
pub use table_directory::TableDirectory;
