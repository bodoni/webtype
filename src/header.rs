//! The file header of [version 1][1] and [version 2][2].
//!
//! [1]: https://www.w3.org/TR/WOFF/#WOFFHeader
//! [2]: https://www.w3.org/TR/WOFF2/#woff20Header

use crate::{Result, Tape, Value};

/// A file header.
#[derive(Clone, Copy, Debug)]
pub enum Header {
    /// Version 1.
    Version1(Header1),
    /// Version 2.
    Version2(Header2),
}

table! {
    #[doc = "A file header of version 1."]
    #[derive(Copy)]
    pub Header1 {
        signature                  (u32) = { 0x774F4646 }, // signature
        flavor                     (u32), // flavor
        length                     (u32), // length
        table_count                (u16), // numTables
        reserved                   (u16) = { 0 }, // reserved
        total_uncompressed_size    (u32), // totalSfntSize
        major_version              (u16), // majorVersion
        minor_version              (u16), // minorVersion
        metadata_offset            (u32), // metaOffset
        compressed_metadata_size   (u32), // metaLength
        uncompressed_metadata_size (u32), // metaOrigLength
        private_data_offset        (u32), // privOffset
        private_data_size          (u32), // privLength
    }
}

table! {
    #[doc = "A file header of version 2."]
    #[derive(Copy)]
    pub Header2 {
        signature                  (u32) = { 0x774F4632 }, // signature
        flavor                     (u32), // flavor
        length                     (u32), // length
        table_count                (u16), // numTables
        reserved                   (u16) = { 0 }, // reserved
        total_uncompressed_size    (u32), // totalSfntSize
        total_compressed_size      (u32), // totalCompressedSize
        major_version              (u16), // majorVersion
        minor_version              (u16), // minorVersion
        metadata_offset            (u32), // metaOffset
        compressed_metadata_size   (u32), // metaLength
        uncompressed_metadata_size (u32), // metaOrigLength
        private_data_offset        (u32), // privOffset
        private_data_size          (u32), // privLength
    }
}

impl Value for Header {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(match tape.peek::<u32>()? {
            0x774F4646 => Header::Version1(tape.take()?),
            0x774F4632 => Header::Version2(tape.take()?),
            _ => raise!("found an unknown version of the file header"),
        })
    }
}
