#[macro_use]
mod support;

mod noto_naskh_arabic {
    use webtype::version2::FileHeader;
    use webtype::Value;

    #[test]
    fn read() {
        let table = ok!(FileHeader::read(&mut setup!(NotoNaskhArabic)));
        assert_eq!(table.flavor, 0x00010000);
        assert_eq!(table.major_version, 1);
        assert_eq!(table.minor_version, 0);
        assert_eq!(table.table_count, 17);
        assert_eq!(table.uncompressed_data_size, 253_268);
        assert_eq!(table.compressed_data_size, 92_474);
    }
}
