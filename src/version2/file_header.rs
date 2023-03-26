//! The [file header][1].
//!
//! [1]: https://www.w3.org/TR/WOFF2/#woff20Header

use opentype::truetype::Tag;

table! {
    #[doc = "A file header."]
    #[derive(Copy)]
    pub FileHeader {
        signature                  (Tag) = { Tag(*b"wOF2") }, // signature
        flavor                     (u32), // flavor
        size                       (u32), // length
        table_count                (u16), // numTables
        reserved                   (u16) = { 0 }, // reserved
        uncompressed_data_size     (u32), // totalSfntSize
        ompressed_data_size        (u32), // totalCompressedSize
        major_version              (u16), // majorVersion
        minor_version              (u16), // minorVersion
        metadata_offset            (u32), // metaOffset
        compressed_metadata_size   (u32), // metaLength
        uncompressed_metadata_size (u32), // metaOrigLength
        private_data_offset        (u32), // privOffset
        private_data_size          (u32), // privLength
    }
}
