#[macro_use]
mod support;

mod noto_naskh_arabic {
    use webtype::version2::{FileHeader, TableDirectory};
    use webtype::{Value, Walue};

    #[test]
    fn read() {
        let mut tape = setup!(NotoNaskhArabic);
        let table = ok!(FileHeader::read(&mut tape));
        let table = ok!(TableDirectory::read(&mut tape, table.table_count as usize));
        let tags = table
            .records
            .iter()
            .map(|record| record.tag())
            .collect::<Vec<_>>();
        let tags = tags.iter().map(|tag| ok!(tag.as_str())).collect::<Vec<_>>();
        assert_eq!(
            tags,
            &[
                "GDEF", "GPOS", "GSUB", "OS/2", "cmap", "cvt ", "fpgm", "gasp", "glyf", "glyf",
                "fvar", "head", "hhea", "hmtx", "maxp", "name", "post",
            ]
        );
        let transformations = table
            .records
            .iter()
            .map(|record| record.transformation())
            .collect::<Vec<_>>();
        assert_eq!(
            transformations,
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0],
        );
    }

    #[test]
    #[should_panic]
    fn decompress() {
        let mut tape = setup!(NotoNaskhArabic);
        let table = ok!(FileHeader::read(&mut tape));
        let table = ok!(TableDirectory::read(&mut tape, table.table_count as usize));
        ok!(table.decompress(tape));
    }
}
