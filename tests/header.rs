#[macro_use]
mod support;

mod noto_naskh_arabic {
    use webtype::version2::Header;
    use webtype::Value;

    #[test]
    fn read() {
        let table = ok!(Header::read(&mut setup!(NotoNaskhArabic)));
        assert_eq!(table.major_version, 1);
        assert_eq!(table.minor_version, 0);
    }
}
