use crate::{Result, Tape, Value};

/// A variable-size u32.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Default)]
pub struct v32(u32);

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
    fn parse() {
        let mut tape = Cursor::new(&[0x3F]);
        assert_eq!(ok!(super::parse(&mut tape)), 63);

        let mut tape = Cursor::new(&[0x80, 0x3F]);
        assert!(super::parse(&mut tape).is_err());
    }
}
