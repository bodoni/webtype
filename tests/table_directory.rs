#[macro_use]
mod support;

mod noto_naskh_arabic {
    use std::io::Cursor;

    use webtype::opentype::truetype::FontHeader;
    use webtype::opentype::Font;
    use webtype::version2::{FileHeader, TableDirectory};
    use webtype::{Value, Walue};

    #[test]
    fn read() {
        let mut tape = setup!(NotoNaskhArabic);
        let file_header = ok!(FileHeader::read(&mut tape));
        let table = ok!(TableDirectory::read(&mut tape, &file_header));
        let tags = table
            .records
            .iter()
            .map(|record| record.tag())
            .collect::<Vec<_>>();
        let tags = tags.iter().map(|tag| ok!(tag.as_str())).collect::<Vec<_>>();
        assert_eq!(
            tags,
            &[
                "GDEF", "GPOS", "GSUB", "OS/2", "cmap", "cvt ", "fpgm", "gasp", "glyf", "loca",
                "head", "hhea", "hmtx", "maxp", "name", "post", "prep",
            ]
        );
        assert!(table
            .records
            .iter()
            .all(|record| record.transformation() == 0));
    }

    #[test]
    fn decompress() {
        let mut tape = setup!(NotoNaskhArabic);
        let file_header = ok!(FileHeader::read(&mut tape));
        let table = ok!(TableDirectory::read(&mut tape, &file_header));
        let data = ok!(table.decompress(&mut tape, &file_header));
        let mut tape = Cursor::new(&data);
        let font = Font {
            offset_table: table.as_offset_table(&file_header),
        };
        let table = ok!(ok!(font.take::<_, FontHeader>(&mut tape)));
        assert_eq!(table.major_version, 1);
        assert_eq!(table.minor_version, 0);
    }
}
