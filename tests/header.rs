#[macro_use]
mod support;

mod noto_naskh_arabic {
    use webtype::Header;
    use webtype::Value;

    #[test]
    fn read() {
        let table = ok!(Header::read(&mut setup!(NotoNaskhArabic)));
        match table {
            Header::Version2(table) => {
                assert_eq!(table.major_version, 1);
                assert_eq!(table.minor_version, 0);
            }
            _ => unreachable!(),
        }
    }
}
