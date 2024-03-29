use super::std_result::StdResult;

pub static FE_LIMIT: u64 = 0x100000000;

// TODO: use enum with size
#[derive(Debug, PartialEq)]
pub struct VarInt {
    pub value: u64,
    pub length: usize, // length when serialized
}

impl VarInt {
    pub fn new(value: u64, length: usize) -> Self {
        Self { value, length }
    }
}

pub fn encode(v: u64) -> Vec<u8> {
    if v < 0xFD {
        return vec![v as u8];
    } else if v < 0x10000 {
        return [[0xFD].as_slice(), (v as u16).to_le_bytes().as_slice()].concat();
    } else if v < FE_LIMIT {
        return [[0xFE].as_slice(), (v as u32).to_le_bytes().as_slice()].concat();
    }

    [[0xFF].as_slice(), v.to_le_bytes().as_slice()].concat()
}

pub fn decode(v: &[u8], from: usize) -> StdResult<VarInt> {
    if v.is_empty() {
        log::error!("varint_decode: invalid length: {}", v.len());
        Err("invalid_length")?;
    }

    if v.len() <= from {
        log::error!("varint_decode: invalid from: {}, v len: {}", from, v.len());
        Err("invalid_from")?;
    }

    let (v, length) = match v[from] {
        0xFD => {
            let mut b: [u8; 2] = [0; 2];
            b.copy_from_slice(&v[(from + 1)..(from + 3)]);
            (u16::from_le_bytes(b) as u64, 3)
        } // 2 bytes + 1 byte marker
        0xFE => {
            let mut b: [u8; 4] = [0; 4];
            b.copy_from_slice(&v[(from + 1)..(from + 5)]);
            (u32::from_le_bytes(b) as u64, 5)
        } // 4 bytes + 1 byte marker
        0xFF => {
            let mut b: [u8; 8] = [0; 8];
            b.copy_from_slice(&v[(from + 1)..(from + 9)]);
            (u64::from_le_bytes(b), 9)
        } // 8 bytes + 1 byte marker
        value => (value as u64, 1),
    };

    Ok(VarInt::new(v, length))
}

#[cfg(test)]
mod tests {
    use super::{decode, encode};

    #[test]
    fn encode_0x00() {
        assert_eq!(encode(0x00), [0x00]);
    }

    #[test]
    fn encode_0x01() {
        assert_eq!(encode(0x01), [0x01]);
    }

    #[test]
    fn encode_0xfc() {
        assert_eq!(encode(0xFC), [0xFC]);
    }

    #[test]
    fn encode_0xfd() {
        assert_eq!(encode(0xFD), [0xFD, 0xFD, 0x00]);
    }

    #[test]
    fn encode_0xffff() {
        assert_eq!(encode(0xFFFF), [0xFD, 0xFF, 0xFF]);
    }

    #[test]
    fn encode_0x10000() {
        assert_eq!(encode(0x10000), [0xFE, 0x00, 0x00, 0x01, 0x00]);
    }

    #[test]
    fn encode_0xffffffffffffffff() {
        assert_eq!(
            encode(0xFFFFFFFFFFFFFFFF),
            [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]
        );
    }

    #[test]
    fn decode_0x00() {
        let v = decode(&vec![0x00], 0).unwrap();
        assert_eq!(v.value, 0x00);
        assert_eq!(v.length, 1);
    }

    #[test]
    fn decode_0x00_with_offset() {
        let v = decode(&vec![0x00, 0x00], 1).unwrap();
        assert_eq!(v.value, 0x00);
        assert_eq!(v.length, 1);
    }

    #[test]
    fn decode_0x01() {
        let v = decode(&vec![0x01], 0).unwrap();
        assert_eq!(v.value, 0x01);
        assert_eq!(v.length, 1);
    }

    #[test]
    fn decode_0x01_with_offset() {
        let v = decode(&vec![0x00, 0x01], 1).unwrap();
        assert_eq!(v.value, 0x01);
        assert_eq!(v.length, 1);
    }

    #[test]
    fn decode_0xfc() {
        let v = decode(&vec![0xFC], 0).unwrap();
        assert_eq!(v.value, 0xFC);
        assert_eq!(v.length, 1);
    }

    #[test]
    fn decode_0xfd() {
        let v = decode(&vec![0xFD, 0xFD, 0x00], 0).unwrap();
        assert_eq!(v.value, 0xFD);
        assert_eq!(v.length, 3);
    }

    #[test]
    fn decode_0xfd_with_offset() {
        let v = decode(&vec![0x01, 0xFD, 0xFD, 0x00], 1).unwrap();
        assert_eq!(v.value, 0xFD);
        assert_eq!(v.length, 3);
    }

    #[test]
    fn decode_0xffff() {
        let v = decode(&vec![0xFD, 0xFF, 0xFF], 0).unwrap();
        assert_eq!(v.value, 0xFFFF);
        assert_eq!(v.length, 3);
    }

    #[test]
    fn decode_0x10000() {
        let v = decode(&vec![0xFE, 0x00, 0x00, 0x01, 0x00], 0).unwrap();
        assert_eq!(v.value, 0x10000);
        assert_eq!(v.length, 5);
    }

    #[test]
    fn decode_0x10000_with_offset() {
        let v = decode(&vec![0x00, 0x00, 0x00, 0xFE, 0x00, 0x00, 0x01, 0x00], 3).unwrap();
        assert_eq!(v.value, 0x10000);
        assert_eq!(v.length, 5);
    }

    #[test]
    fn decode_0xffffffffffffffff() {
        let v = decode(&vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF], 0).unwrap();
        assert_eq!(v.value, 0xFFFFFFFFFFFFFFFF);
        assert_eq!(v.length, 9);
    }

    #[test]
    fn decode_0xffffffffffffffff_with_offset() {
        let v = decode(
            &vec![0x01, 0x02, 0x03, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF],
            3,
        )
        .unwrap();
        assert_eq!(v.value, 0xFFFFFFFFFFFFFFFF);
        assert_eq!(v.length, 9);
    }

    #[test]
    fn decode_invalid_length() {
        assert_eq!(
            decode(&Vec::<u8>::new(), 0).err().unwrap().to_string(),
            "invalid_length"
        );
    }

    #[test]
    fn decode_invalid_from() {
        assert_eq!(decode(&vec![0x00], 1).err().unwrap().to_string(), "invalid_from");
    }
}
