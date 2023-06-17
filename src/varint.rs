use once_cell::sync::Lazy;

use rug::Integer;

use crate::integer_ex::IntegerEx;

pub static FE_LIMIT: Lazy<Integer> = Lazy::new(|| Integer::from_hex_str("100000000"));
pub static FF_LIMIT: Lazy<Integer> = Lazy::new(|| Integer::from_hex_str("10000000000000000"));

#[derive(Debug, PartialEq)]
pub struct VarInt {
    pub value: Integer,
    pub length: usize,
}

#[derive(Debug, PartialEq)]
pub enum VarIntError {
    InvalidLength,
    IntegerTooLarge,
    InvalidFrom,
}

impl VarInt {
    pub fn new(value: Integer, length: usize) -> Self {
        Self { value, length }
    }
}

pub fn varint_encode(v: Integer) -> Result<Vec<u8>, VarIntError> {
    if v < 0xFD {
        return Ok(vec![v.to_u8().unwrap()]);
    } else if v < 0x10000 {
        return Ok([[0xFD].as_slice(), v.to_little_endian_bytes(2).as_slice()].concat());
    } else if v < (*FE_LIMIT) {
        return Ok([[0xFE].as_slice(), v.to_little_endian_bytes(4).as_slice()].concat());
    } else if v < (*FF_LIMIT) {
        return Ok([[0xFF].as_slice(), v.to_little_endian_bytes(8).as_slice()].concat());
    }

    log::error!("varint_encode: Integer too large: {}", v);
    Err(VarIntError::IntegerTooLarge)
}

pub fn varint_decode(v: &[u8], from: usize) -> Result<VarInt, VarIntError> {
    if v.is_empty() {
        log::error!("varint_decode: invalid length: {}", v.len());
        return Err(VarIntError::InvalidLength);
    }

    if v.len() <= from {
        log::error!("varint_decode: invalid from: {}, v len: {}", from, v.len());
        return Err(VarIntError::InvalidFrom);
    }

    let (range, length) = match v[from] {
        0xFD => ((from + 1)..(from + 3), 3), // 2 bytes + 1 byte marker
        0xFE => ((from + 1)..(from + 5), 5), // 4 bytes + 1 byte marker
        0xFF => ((from + 1)..(from + 9), 9), // 8 bytes + 1 byte marker
        _ => (from..(from + 1), 1),
    };

    let v = Integer::from_little_endian_bytes(&v[range.clone()]);

    Ok(VarInt::new(v, length))
}

#[cfg(test)]
mod tests {
    use rug::Integer;

    use crate::{
        integer_ex::IntegerEx,
        varint::{varint_decode, varint_encode, VarIntError},
    };

    #[test]
    fn varint_encode_0x00() {
        assert_eq!(varint_encode(Integer::from(0x00)).unwrap(), [0x00]);
    }

    #[test]
    fn varint_encode_0x01() {
        assert_eq!(varint_encode(Integer::from(0x01)).unwrap(), [0x01]);
    }

    #[test]
    fn varint_encode_0xfc() {
        assert_eq!(varint_encode(Integer::from(0xFC)).unwrap(), [0xFC]);
    }

    #[test]
    fn varint_encode_0xfd() {
        assert_eq!(varint_encode(Integer::from(0xFD)).unwrap(), [0xFD, 0xFD, 0x00]);
    }

    #[test]
    fn varint_encode_0xffff() {
        assert_eq!(varint_encode(Integer::from(0xFFFF)).unwrap(), [0xFD, 0xFF, 0xFF]);
    }

    #[test]
    fn varint_encode_0x10000() {
        assert_eq!(
            varint_encode(Integer::from(0x10000)).unwrap(),
            [0xFE, 0x00, 0x00, 0x01, 0x00]
        );
    }

    #[test]
    fn varint_encode_0xffffffffffffffff() {
        assert_eq!(
            varint_encode(Integer::from_hex_str("FFFFFFFFFFFFFFFF")).unwrap(),
            [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]
        );
    }

    #[test]
    fn varint_encode_0x10000000000000000() {
        assert_eq!(
            varint_encode(Integer::from_hex_str("10000000000000000")),
            Err(VarIntError::IntegerTooLarge)
        );
    }

    #[test]
    fn varint_decode_0x00() {
        let v = varint_decode(&vec![0x00], 0).unwrap();
        assert_eq!(v.value, Integer::from(0x00));
        assert_eq!(v.length, 1);
    }

    #[test]
    fn varint_decode_0x00_with_offset() {
        let v = varint_decode(&vec![0x00, 0x00], 1).unwrap();
        assert_eq!(v.value, Integer::from(0x00));
        assert_eq!(v.length, 1);
    }

    #[test]
    fn varint_decode_0x01() {
        let v = varint_decode(&vec![0x01], 0).unwrap();
        assert_eq!(v.value, Integer::from(0x01));
        assert_eq!(v.length, 1);
    }

    #[test]
    fn varint_decode_0x01_with_offset() {
        let v = varint_decode(&vec![0x00, 0x01], 1).unwrap();
        assert_eq!(v.value, Integer::from(0x01));
        assert_eq!(v.length, 1);
    }

    #[test]
    fn varint_decode_0xfc() {
        let v = varint_decode(&vec![0xFC], 0).unwrap();
        assert_eq!(v.value, Integer::from(0xFC));
        assert_eq!(v.length, 1);
    }

    #[test]
    fn varint_decode_0xfd() {
        let v = varint_decode(&vec![0xFD, 0xFD, 0x00], 0).unwrap();
        assert_eq!(v.value, Integer::from(0xFD));
        assert_eq!(v.length, 3);
    }

    #[test]
    fn varint_decode_0xfd_with_offset() {
        let v = varint_decode(&vec![0x01, 0xFD, 0xFD, 0x00], 1).unwrap();
        assert_eq!(v.value, Integer::from(0xFD));
        assert_eq!(v.length, 3);
    }
    #[test]
    fn varint_decode_0xffff() {
        let v = varint_decode(&vec![0xFD, 0xFF, 0xFF], 0).unwrap();
        assert_eq!(v.value, Integer::from(0xFFFF));
        assert_eq!(v.length, 3);
    }

    #[test]
    fn varint_decode_0x10000() {
        let v = varint_decode(&vec![0xFE, 0x00, 0x00, 0x01, 0x00], 0).unwrap();
        assert_eq!(v.value, Integer::from(0x10000));
        assert_eq!(v.length, 5);
    }

    #[test]
    fn varint_decode_0x10000_with_offset() {
        let v = varint_decode(&vec![0x00, 0x00, 0x00, 0xFE, 0x00, 0x00, 0x01, 0x00], 3).unwrap();
        assert_eq!(v.value, Integer::from(0x10000));
        assert_eq!(v.length, 5);
    }

    #[test]
    fn varint_decode_0xffffffffffffffff() {
        let v = varint_decode(&vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF], 0).unwrap();
        assert_eq!(v.value, Integer::from_hex_str("FFFFFFFFFFFFFFFF"));
        assert_eq!(v.length, 9);
    }

    #[test]
    fn varint_decode_0xffffffffffffffff_with_offset() {
        let v = varint_decode(
            &vec![0x01, 0x02, 0x03, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF],
            3,
        )
        .unwrap();
        assert_eq!(v.value, Integer::from_hex_str("FFFFFFFFFFFFFFFF"));
        assert_eq!(v.length, 9);
    }

    #[test]
    fn varint_decode_invalid_length() {
        assert_eq!(varint_decode(&Vec::<u8>::new(), 0), Err(VarIntError::InvalidLength));
    }

    #[test]
    fn varint_decode_invalid_from() {
        assert_eq!(varint_decode(&vec![0x00], 1), Err(VarIntError::InvalidFrom));
    }
}
