# WebType [![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url] [![Build][build-img]][build-url]

The package provides a parser for fonts in Web Open Font Format. It might be
helpful to have a look at a higher-level parser called [`font`][font], which
internally relies on this package.

## Example

```rust
use webtype::opentype::truetype::FontHeader;
use webtype::File;

macro_rules! ok(($result:expr) => ($result.unwrap()));

let path = "NotoNaskhArabic-Regular.woff2";
let mut tape = ok!(std::fs::File::open(path));
let File { mut fonts, mut tape } = ok!(File::read(&mut tape));

let font_header = ok!(ok!(fonts[0].take::<_, FontHeader>(&mut tape)));
assert_eq!(font_header.units_per_em, 2048);
```

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[font]: https://github.com/bodoni/font

[build-img]: https://github.com/bodoni/webtype/workflows/build/badge.svg
[build-url]: https://github.com/bodoni/webtype/actions/workflows/build.yml
[documentation-img]: https://docs.rs/webtype/badge.svg
[documentation-url]: https://docs.rs/webtype
[package-img]: https://img.shields.io/crates/v/webtype.svg
[package-url]: https://crates.io/crates/webtype
