pub mod vector {
    /// u8 vector to array with 32 bytes fixed length
    pub fn vect_to_array_32(v: &Vec<u8>) -> [u8; 32] {
        let mut arr: [u8; 32] = [0u8; 32];
        for i in 0..v.len() {
            arr[31 - i] = v[i];
        }

        arr
    }

    /// u8 byte array from hex string (two chars for single byte).
    pub fn string_to_bytes(s: &str) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::with_capacity(s.len() / 2);

        for i in (0..s.len()).step_by(2) {
            let byte = u8::from_str_radix(&s[i..i + 2], 16).unwrap();
            res.push(byte);
        }

        res
    }
}
