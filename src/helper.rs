pub mod vector {
    /// u8 Msf vector to Msf array with 32 bytes fixed length.
    pub fn vect_to_array_32(v: &Vec<u8>) -> [u8; 32] {
        let mut arr: [u8; 32] = [0u8; 32];
        let cursor = 32 - v.len();
        for i in 0..v.len() {
            arr[cursor + i] = v[i];
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

    /// trim left leading byte
    pub fn trim_left(v: &Vec<u8>, value: u8) -> Vec<u8> {
        let mut l: usize = 0;
        let len = v.len();

        while l != len && v[l] == value {
            l += 1;
        }

        v[l..v.len()].to_vec()
    }
}

#[cfg(test)]
mod helper_test {
    use crate::helper::vector;

    #[test]
    fn no_left_trim() {
        let v = vector::trim_left(&vec![1, 2, 3, 4, 5], 0);
        assert_eq!(v, vec![1, 2, 3, 4, 5])
    }

    #[test]
    fn one_left_trim() {
        let v = vector::trim_left(&vec![0, 1, 2, 3, 4, 5], 0);
        assert_eq!(v, vec![1, 2, 3, 4, 5])
    }

    #[test]
    fn more_left_trim() {
        let v = vector::trim_left(&vec![0, 0, 0, 1, 2, 3, 4, 5], 0);
        assert_eq!(v, vec![1, 2, 3, 4, 5])
    }

    #[test]
    fn some_right_left_trim() {
        let v = vector::trim_left(&vec![0, 0, 0, 1, 2, 3, 4, 5, 0, 0], 0);
        assert_eq!(v, vec![1, 2, 3, 4, 5, 0, 0])
    }

    #[test]
    fn all_left_trim() {
        let v = vector::trim_left(&vec![0, 0, 0, 0, 0, 0, 0], 0);
        let expected: Vec<u8> = Vec::new();
        assert_eq!(v, expected)
    }

    #[test]
    fn none_left_trim() {
        let v = vector::trim_left(&vec![], 0);
        let expected: Vec<u8> = Vec::new();
        assert_eq!(v, expected)
    }
}
