use super::{std_result::StdResult, varint::VarInt};

// TODO: use enum with size
#[derive(Debug, PartialEq)]
pub struct VarString {
    pub length: VarInt,
    pub value: Vec<u8>,
}

impl VarString {
    pub fn new(v: &str) -> Self {
        let l = super::varint::encode(v.len() as u64);
        let length = super::varint::decode(&l, 0).unwrap();
        let value = v.as_bytes().to_vec();
        Self { length, value }
    }

    pub fn encode(&self) -> Vec<u8> {
        let length_encoded = super::varint::encode(self.length.value);
        [length_encoded.as_slice(), self.value.as_slice()].concat()
    }
}

pub fn decode(v: &[u8], from: usize) -> StdResult<VarString> {
    let length: VarInt = super::varint::decode(v, from)?;
    let value = v[(from + length.length)..(from + length.length + length.value as usize)].to_vec();
    Ok(VarString { length, value })
}

#[cfg(test)]
mod tests {
    use crate::std_lib::varstring::{decode, VarString};

    #[test]
    fn empty_string() {
        let s = VarString::new("");
        let decoded = [0x00];
        assert_eq!(s.encode(), decoded);

        let v = decode(&decoded, 0).unwrap();
        assert_eq!(s, v);
    }

    #[test]
    fn encode_string_len_1() {
        let s = VarString::new("a");
        let decoded = [0x01, 0x61];
        assert_eq!(s.encode(), decoded);

        let v = decode(&decoded, 0).unwrap();
        assert_eq!(s, v);
    }

    #[test]
    fn encode_string_len_2() {
        let s = VarString::new("ab");
        let decoded = [0x02, 0x61, 0x62];
        assert_eq!(s.encode(), decoded);

        let v = decode(&decoded, 0).unwrap();
        assert_eq!(s, v);
    }

    #[test]
    fn encode_string_len_255() {
        let s = VarString::new("a".repeat(255).as_str());
        let body = [0x61].repeat(255);
        let decoded = [[0xFD, 0xFF, 0x00].as_slice(), body.as_slice()].concat();
        assert_eq!(s.encode(), decoded);

        let v = decode(&decoded, 0).unwrap();
        assert_eq!(s, v);
    }
}
