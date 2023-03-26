//! The table directory.
//!
//! [1]: https://www.w3.org/TR/WOFF2/#table_dir_format

use opentype::truetype::Tag;

use crate::number::v32;
use crate::{Result, Tape, Walue};

const TAG_MASK: u8 = 0b00111111;

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
            if this.flags & TAG_MASK == 63 {
                Ok(Some(tape.take()?))
            } else {
                Ok(None)
            }
        },

        uncompressed_size (v32), // origLength

        compressed_size (Option<v32>) |this, tape| { // transformLength
            if this.flags & !TAG_MASK > 0 {
                Ok(Some(tape.take()?))
            } else {
                Ok(None)
            }
        },
    }
}

impl Walue<'static> for TableDirectory {
    type Parameter = usize;

    fn read<T: Tape>(tape: &mut T, table_count: usize) -> Result<Self> {
        Ok(TableDirectory {
            records: tape.take_given(table_count)?,
        })
    }
}

impl Record {
    /// Return the tag.
    pub fn tag(&self) -> Tag {
        println!("{:?}", self);
        self.tag.unwrap_or_else(|| Tag(*map(self.flags & TAG_MASK)))
    }

    /// Return the transformation version
    #[inline]
    pub fn transformation(&self) -> u8 {
        self.flags.wrapping_shr(6)
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
