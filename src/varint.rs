use once_cell::sync::Lazy;

use rug::Integer;

use crate::integer_ex::IntegerEx;

pub static FE_LIMIT: Lazy<Integer> = Lazy::new(|| Integer::from_hex_str("100000000"));
pub static FF_LIMIT: Lazy<Integer> = Lazy::new(|| Integer::from_hex_str("10000000000000000"));

pub fn varint_encode(v: Integer) -> Vec<u8> {
    if v < 0xFD {
        return vec![v.to_u8().unwrap()];
    } else if v < 0x10000 {
        return [[0xFD].as_slice(), v.to_little_endian_bytes(2).as_slice()].concat();
    } else if v < (*FE_LIMIT) {
        return [[0xFE].as_slice(), v.to_little_endian_bytes(4).as_slice()].concat();
    } else if v < (*FF_LIMIT) {
        return [[0xFF].as_slice(), v.to_little_endian_bytes(8).as_slice()].concat();
    }

    panic!("integer too large for varint");
}

#[cfg(test)]
mod tests {
    use rug::Integer;

    use crate::{integer_ex::IntegerEx, varint::varint_encode};

    #[test]
    fn varint_encode_0x00() {
        assert_eq!(varint_encode(Integer::from(0x00)), [0x00]);
    }

    #[test]
    fn varint_encode_0x01() {
        assert_eq!(varint_encode(Integer::from(0x01)), [0x01]);
    }

    #[test]
    fn varint_encode_0xfc() {
        assert_eq!(varint_encode(Integer::from(0xFC)), [0xFC]);
    }

    #[test]
    fn varint_encode_0xfd() {
        assert_eq!(varint_encode(Integer::from(0xFD)), [0xFD, 0xFD, 0x00]);
    }

    #[test]
    fn varint_encode_0xffff() {
        assert_eq!(varint_encode(Integer::from(0xFFFF)), [0xFD, 0xFF, 0xFF]);
    }

    #[test]
    fn varint_encode_0x10000() {
        assert_eq!(varint_encode(Integer::from(0x10000)), [0xFE, 0x00, 0x00, 0x01, 0x00]);
    }

    #[test]
    fn varint_encode_0xffffffffffffffff() {
        assert_eq!(
            varint_encode(Integer::from_hex_str("FFFFFFFFFFFFFFFF")),
            [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]
        );
    }

    #[test]
    #[should_panic(expected = "integer too large for varint")]
    fn varint_encode_0x10000000000000000() {
        varint_encode(Integer::from_hex_str("10000000000000000"));
    }
}
