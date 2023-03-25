//! The table directory.
//!
//! [1]: https://www.w3.org/TR/WOFF2/#table_dir_format

use opentype::truetype::Tag;

use crate::v32;

table! {
    #[doc = "A table-directory record."]
    #[derive(Copy)]
    pub Record {
        flags             (u8 ), // flags
        tag               (Tag), // tag
        uncompressed_size (v32), // origLength
        compressed_size   (v32), // transformLength
    }
}
