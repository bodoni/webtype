//! The table directory.
//!
//! [1]: https://www.w3.org/TR/WOFF2/#table_dir_format

use opentype::truetype::tables::Offsets;
use opentype::truetype::Tag;

use crate::number::v32;
use crate::version2::file_header::FileHeader;
use crate::Result;

const TAG_MASK: u8 = 0b0011_1111;

/// A table directory.
pub struct TableDirectory {
    pub records: Vec<Record>,
}

table! {
    #[doc = "A table-directory record."]
    #[derive(Copy)]
    pub Record {
        flags (u8), // flags

        tag (Option<Tag>) |this, tape| { // tag
            if this.is_arbitrary() {
                Ok(Some(tape.take()?))
            } else {
                Ok(None)
            }
        },

        uncompressed_untransformed_size (v32), // origLength

        uncompressed_transformed_size (Option<v32>) |this, tape| { // transformLength
            if this.is_transformed() {
                Ok(Some(tape.take()?))
            } else {
                Ok(None)
            }
        },
    }
}

impl TableDirectory {
    /// Convert to an offset table.
    pub fn as_offsets(&self, file_header: &FileHeader) -> Offsets {
        use opentype::truetype::tables::offsets::{Header, Record};

        Offsets {
            header: Header {
                version: file_header.flavor,
                table_count: file_header.table_count,
                ..Default::default()
            },
            records: self
                .iter()
                .scan(0, |offset, record| {
                    let record = Record {
                        tag: record.tag(),
                        offset: *offset,
                        size: record.uncompressed_size() as u32,
                        ..Default::default()
                    };
                    *offset += record.size;
                    Some(record)
                })
                .collect(),
        }
    }

    /// Decompress all tables.
    pub fn decompress<T: crate::tape::Read>(&self, mut tape: T, _: &FileHeader) -> Result<Vec<u8>> {
        let size = self.iter().map(|record| record.uncompressed_size()).sum();
        let mut data = Vec::with_capacity(size);
        brotli_decompressor::BrotliDecompress(&mut tape, &mut data)?;
        Ok(data)
    }
}

dereference! { TableDirectory::records => [Record] }

impl<'l> crate::walue::Read<'l> for TableDirectory {
    type Parameter = &'l FileHeader;

    fn read<T: crate::tape::Read>(tape: &mut T, file_header: &'l FileHeader) -> Result<Self> {
        let records = tape.take_given(file_header.table_count as usize)?;
        Ok(TableDirectory { records })
    }
}

impl Record {
    /// Check if the table is transformed.
    #[inline]
    pub fn is_transformed(&self) -> bool {
        let transformation = self.transformation();
        matches!(self.tag().as_str(), Some("glyf") | Some("loca")) && transformation != 3
            || transformation != 0
    }

    /// Return the tag of the table.
    pub fn tag(&self) -> Tag {
        self.tag.unwrap_or_else(|| Tag(*map(self.flags & TAG_MASK)))
    }

    /// Return the transformation version.
    #[inline]
    pub fn transformation(&self) -> u8 {
        self.flags.wrapping_shr(6)
    }

    /// Return the size of the uncompressed data.
    pub fn uncompressed_size(&self) -> usize {
        self.uncompressed_transformed_size
            .unwrap_or(self.uncompressed_untransformed_size)
            .0 as usize
    }

    #[inline]
    fn is_arbitrary(&self) -> bool {
        self.flags & TAG_MASK == TAG_MASK
    }
}

#[inline]
fn map(value: u8) -> &'static [u8; 4] {
    match value {
        0 => b"cmap",
        1 => b"head",
        2 => b"hhea",
        3 => b"hmtx",
        4 => b"maxp",
        5 => b"name",
        6 => b"OS/2",
        7 => b"post",
        8 => b"cvt ",
        9 => b"fpgm",
        10 => b"glyf",
        11 => b"loca",
        12 => b"prep",
        13 => b"CFF ",
        14 => b"VORG",
        15 => b"EBDT",
        16 => b"EBLC",
        17 => b"gasp",
        18 => b"hdmx",
        19 => b"kern",
        20 => b"LTSH",
        21 => b"PCLT",
        22 => b"VDMX",
        23 => b"vhea",
        24 => b"vmtx",
        25 => b"BASE",
        26 => b"GDEF",
        27 => b"GPOS",
        28 => b"GSUB",
        29 => b"EBSC",
        30 => b"JSTF",
        31 => b"MATH",
        32 => b"CBDT",
        33 => b"CBLC",
        34 => b"COLR",
        35 => b"CPAL",
        36 => b"SVG ",
        37 => b"sbix",
        38 => b"acnt",
        39 => b"avar",
        40 => b"bdat",
        41 => b"bloc",
        42 => b"bsln",
        43 => b"cvar",
        44 => b"fdsc",
        45 => b"feat",
        46 => b"fmtx",
        47 => b"fvar",
        48 => b"gvar",
        49 => b"hsty",
        50 => b"just",
        51 => b"lcar",
        52 => b"mort",
        53 => b"morx",
        54 => b"opbd",
        55 => b"prop",
        56 => b"trak",
        57 => b"Zapf",
        58 => b"Silf",
        59 => b"Glat",
        60 => b"Gloc",
        61 => b"Feat",
        62 => b"Sill",
        _ => unreachable!(),
    }
}
