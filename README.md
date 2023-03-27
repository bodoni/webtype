# WebType [![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url] [![Build][build-img]][build-url]

The package provides a parser for fonts in Web Open Font Format.

## Example

```rust
use opentype::truetype::FontHeader;
use webtype::File;

macro_rules! ok(($result:expr) => ($result.unwrap()));

let path = "NotoNaskhArabic-Regular.woff2";
let mut tape = ok!(std::fs::File::open(path));
let File { mut fonts, data } = ok!(File::read(&mut tape));
let mut tape = std::io::Cursor::new(&data);

let font_header = ok!(ok!(fonts[0].take::<_, FontHeader>(&mut tape)));
assert_eq!(font_header.units_per_em, 2048);
```

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[build-img]: https://github.com/bodoni/webtype/workflows/build/badge.svg
[build-url]: https://github.com/bodoni/webtype/actions/workflows/build.yml
[documentation-img]: https://docs.rs/webtype/badge.svg
[documentation-url]: https://docs.rs/webtype
[package-img]: https://img.shields.io/crates/v/webtype.svg
[package-url]: https://crates.io/crates/webtype
