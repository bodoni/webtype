use std::io::Cursor;

use opentype::truetype::Tag;
use opentype::Font;

use crate::Result;

/// A file.
pub struct File {
    /// The fonts.
    pub fonts: Vec<Font>,
    /// The decompressed font data.
    pub tape: Cursor<Vec<u8>>,
}

impl File {
    /// Read a file.
    pub fn read<T: crate::tape::Read>(tape: &mut T) -> Result<File> {
        use crate::tape::Read;

        match Read::peek::<(Tag, Tag)>(tape)? {
            (tag, _) if tag.0 == *b"wOFF" => {
                raise!("found version 1, which is not supported yet");
            }
            (_, tag) if tag.0 == *b"ttfc" => {
                raise!("found a TrueType collection, which is not supported yet");
            }
            _ => {}
        }
        let (font, data) = read_version2(tape)?;
        Ok(File {
            fonts: vec![font],
            tape: Cursor::new(data),
        })
    }
}

dereference! { File::fonts => [Font] }

fn read_version2<T: crate::tape::Read>(mut tape: T) -> Result<(Font, Vec<u8>)> {
    use crate::value::Read as ValueRead;
    use crate::version2::{FileHeader, TableDirectory};
    use crate::walue::Read as WalueRead;

    let file_header = FileHeader::read(&mut tape)?;
    let table_directory = TableDirectory::read(&mut tape, &file_header)?;
    let offsets = table_directory.as_offsets(&file_header);
    let data = table_directory.decompress(&mut tape, &file_header)?;
    Ok((Font { offsets }, data))
}
