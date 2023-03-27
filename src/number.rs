use crate::{Result, Tape, Value};

/// A four-byte unsigned integer with a variable-length encoding.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Default)]
pub struct v32(pub u32);

impl Value for v32 {
    #[inline]
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        Ok(v32(parse(tape)?))
    }
}

fn parse<T: Tape>(tape: &mut T) -> Result<u32> {
    let mut value: u32 = 0;
    for i in 0..5 {
        let byte: u32 = tape.take::<u8>()? as _;
        if i == 0 && byte == 0x80 {
            break;
        }
        if value & 0xFE000000 > 0 {
            break;
        }
        value = (value << 7) | (byte & 0x7F);
        if (byte & 0x80) == 0 {
            return Ok(value);
        }
    }
    error!("found a malformed number")
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    macro_rules! ok(($result:expr) => ($result.unwrap()));

    #[test]
    fn parse_failure() {
        let mut tape = Cursor::new(&[0x80, 0x3F]);
        assert!(super::parse(&mut tape).is_err());
    }

    #[test]
    fn parse_success() {
        macro_rules! test(
            ($input:expr, $output:expr) => (
                let mut tape = Cursor::new($input);
                assert_eq!(ok!(super::parse(&mut tape)), $output);
            );
        );

        test!(&[0], 0);
        test!(&[0x3F], 63);
        test!(&[0x81, 0], 128);
        test!(&[0x8F, 0xFF, 0xFF, 0xFF, 0x7F], 4294967295);
    }
}
