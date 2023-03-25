//! The [file header][1].
//!
//! [1]: https://www.w3.org/TR/WOFF/#WOFFHeader

table! {
    #[doc = "A file header."]
    #[derive(Copy)]
    pub Header {
        signature                  (u32) = { 0x774F4646 }, // signature
        flavor                     (u32), // flavor
        size                       (u32), // length
        table_count                (u16), // numTables
        reserved                   (u16) = { 0 }, // reserved
        uncompressed_data_size     (u32), // totalSfntSize
        major_version              (u16), // majorVersion
        minor_version              (u16), // minorVersion
        metadata_offset            (u32), // metaOffset
        compressed_metadata_size   (u32), // metaLength
        uncompressed_metadata_size (u32), // metaOrigLength
        private_data_offset        (u32), // privOffset
        private_data_size          (u32), // privLength
    }
}
