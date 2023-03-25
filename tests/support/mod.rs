use std::fs::File;
use std::path::PathBuf;

macro_rules! ok(($result:expr) => ($result.unwrap()));

macro_rules! setup(
    ($fixture:ident) => (crate::support::setup(crate::support::Fixture::$fixture));
);

#[derive(Clone, Copy, Debug)]
pub enum Fixture {
    NotoNaskhArabic,
}

impl Fixture {
    pub fn file_name(&self) -> &'static str {
        match *self {
            Fixture::NotoNaskhArabic => "NotoNaskhArabic-Regular.woff2",
        }
    }

    pub fn path(&self) -> PathBuf {
        PathBuf::from("tests")
            .join("fixtures")
            .join(self.file_name())
    }
}

pub fn setup(fixture: Fixture) -> File {
    ok!(File::open(fixture.path()))
}
