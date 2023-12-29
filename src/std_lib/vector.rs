use super::std_result::StdResult;

/// u8 Msf vector to Msf array with 32 bytes fixed length.
pub fn vect_to_array_32(v: &[u8]) -> [u8; 32] {
    let mut arr: [u8; 32] = [0u8; 32];
    let cursor = 32 - v.len();
    arr[cursor..(v.len() + cursor)].copy_from_slice(v);

    arr
}

/// u8 byte array from hex string (two chars for single byte).
pub fn string_to_bytes(s: &str) -> StdResult<Vec<u8>> {
    let mut input: String = s.to_string();
    if s.len() % 2 != 0 {
        input = format!("0{}", s);
    }

    let mut res: Vec<u8> = Vec::with_capacity(input.len() / 2);

    for i in (0..input.len()).step_by(2) {
        let byte = u8::from_str_radix(&input[i..i + 2], 16)?;
        res.push(byte);
    }

    Ok(res)
}

pub fn bytes_to_string(bytes: &[u8]) -> String {
    let strs: Vec<String> = bytes.iter().map(|b| format!("{:02X}", b)).collect();
    strs.join("")
}

pub fn bytes_to_string_64(bytes: &[u8]) -> String {
    let strs: Vec<String> = bytes.iter().map(|b| format!("{:02X}", b)).collect();
    format!("{:0>64}", strs.join(""))
}

/// trim left leading byte
pub fn trim_left(v: &[u8], value: u8) -> Vec<u8> {
    let mut l: usize = 0;
    let len = v.len();

    while l != len && v[l] == value {
        l += 1;
    }

    v[l..v.len()].to_vec()
}

/// trim right leading byte
pub fn trim_right(v: &[u8], value: u8) -> Vec<u8> {
    let mut l: usize = 0;
    let len = v.len();

    while l != len && v[len - l - 1] == value {
        l += 1;
    }

    v[..len - l].to_vec()
}

pub fn padding_left(v: &[u8], length: usize, value: u8) -> Vec<u8> {
    if v.len() > length {
        return v.to_vec();
    }

    let mut arr: Vec<u8> = vec![value; length];
    arr[length - v.len()..length].copy_from_slice(v);

    arr
}

pub fn padding_right(v: &[u8], length: usize, value: u8) -> Vec<u8> {
    if v.len() > length {
        return v.to_vec();
    }

    let mut arr: Vec<u8> = vec![value; length];
    arr[0..v.len()].copy_from_slice(v);

    arr
}

#[cfg(test)]
mod helper_test {
    use super::*;

    #[test]
    fn no_left_trim() {
        let v = trim_left(&vec![1, 2, 3, 4, 5], 0);
        assert_eq!(v, vec![1, 2, 3, 4, 5])
    }

    #[test]
    fn one_left_trim() {
        let v = trim_left(&vec![0, 1, 2, 3, 4, 5], 0);
        assert_eq!(v, vec![1, 2, 3, 4, 5])
    }

    #[test]
    fn more_left_trim() {
        let v = trim_left(&vec![0, 0, 0, 1, 2, 3, 4, 5], 0);
        assert_eq!(v, vec![1, 2, 3, 4, 5])
    }

    #[test]
    fn some_right_left_trim() {
        let v = trim_left(&vec![0, 0, 0, 1, 2, 3, 4, 5, 0, 0], 0);
        assert_eq!(v, vec![1, 2, 3, 4, 5, 0, 0])
    }

    #[test]
    fn all_left_trim() {
        let v = trim_left(&vec![0, 0, 0, 0, 0, 0, 0], 0);
        let expected: Vec<u8> = Vec::new();
        assert_eq!(v, expected)
    }

    #[test]
    fn none_left_trim() {
        let v = trim_left(&vec![], 0);
        let expected: Vec<u8> = Vec::new();
        assert_eq!(v, expected)
    }

    #[test]
    fn no_right_trim() {
        let v = trim_right(&vec![1, 2, 3, 4, 5], 0);
        assert_eq!(v, vec![1, 2, 3, 4, 5])
    }

    #[test]
    fn one_right_trim() {
        let v = trim_right(&vec![1, 2, 3, 4, 5, 0], 0);
        assert_eq!(v, vec![1, 2, 3, 4, 5])
    }

    #[test]
    fn more_right_trim() {
        let v = trim_right(&vec![1, 2, 3, 4, 5, 0, 0, 0], 0);
        assert_eq!(v, vec![1, 2, 3, 4, 5])
    }

    #[test]
    fn some_left_right_trim() {
        let v = trim_right(&vec![0, 0, 1, 2, 3, 4, 5, 0, 0, 0], 0);
        assert_eq!(v, vec![0, 0, 1, 2, 3, 4, 5])
    }

    #[test]
    fn all_right_trim() {
        let v = trim_right(&vec![0, 0, 0, 0, 0, 0, 0], 0);
        let expected: Vec<u8> = Vec::new();
        assert_eq!(v, expected)
    }

    #[test]
    fn none_right_trim() {
        let v = trim_right(&vec![], 0);
        let expected: Vec<u8> = Vec::new();
        assert_eq!(v, expected)
    }

    #[test]
    fn padding_left_needed_1() {
        let v = padding_left(&vec![1, 2, 3], 10, 0);
        let expected: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 1, 2, 3];
        assert_eq!(v, expected)
    }

    #[test]
    fn padding_left_not_needed() {
        let v = padding_left(&vec![1, 2, 3], 3, 0);
        let expected: Vec<u8> = vec![1, 2, 3];
        assert_eq!(v, expected)
    }

    #[test]
    fn padding_left_length_greater_then_vect_length() {
        let v = padding_left(&vec![1, 2, 3], 2, 0);
        let expected: Vec<u8> = vec![1, 2, 3];
        assert_eq!(v, expected)
    }

    #[test]
    fn padding_left_needed_2() {
        let v = padding_left(&vec![], 3, 1);
        let expected: Vec<u8> = vec![1, 1, 1];
        assert_eq!(v, expected)
    }

    #[test]
    fn padding_left_needed_3() {
        let v = padding_left(&vec![1], 3, 1);
        let expected: Vec<u8> = vec![1, 1, 1];
        assert_eq!(v, expected)
    }

    #[test]
    fn padding_left_needed_4() {
        let v = padding_left(&vec![0], 3, 1);
        let expected: Vec<u8> = vec![1, 1, 0];
        assert_eq!(v, expected)
    }

    #[test]
    fn padding_right_needed_1() {
        let v = padding_right(&vec![1, 2, 3], 10, 0);
        let expected: Vec<u8> = vec![1, 2, 3, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(v, expected)
    }

    #[test]
    fn padding_right_not_needed() {
        let v = padding_right(&vec![1, 2, 3], 3, 0);
        let expected: Vec<u8> = vec![1, 2, 3];
        assert_eq!(v, expected)
    }

    #[test]
    fn padding_right_length_greater_then_vect_length() {
        let v = padding_right(&vec![1, 2, 3], 2, 0);
        let expected: Vec<u8> = vec![1, 2, 3];
        assert_eq!(v, expected)
    }

    #[test]
    fn padding_right_needed_2() {
        let v = padding_right(&vec![], 3, 1);
        let expected: Vec<u8> = vec![1, 1, 1];
        assert_eq!(v, expected)
    }

    #[test]
    fn padding_right_needed_3() {
        let v = padding_right(&vec![1], 3, 1);
        let expected: Vec<u8> = vec![1, 1, 1];
        assert_eq!(v, expected)
    }

    #[test]
    fn padding_right_needed_4() {
        let v = padding_right(&vec![0], 3, 1);
        let expected: Vec<u8> = vec![0, 1, 1];
        assert_eq!(v, expected)
    }

    macro_rules! string_to_bytes {
        ($s:literal, $v:expr, $f:ident) => {
            #[test]
            fn $f() {
                assert_eq!(string_to_bytes($s).unwrap(), $v);
            }
        };
    }

    string_to_bytes!("", Vec::<u8>::new(), string_to_bytes_0);
    string_to_bytes!("0", vec![0], string_to_bytes_1);
    string_to_bytes!("00", vec![0], string_to_bytes_2);
    string_to_bytes!("0101", vec![1, 1], string_to_bytes_3);
    string_to_bytes!("101", vec![1, 1], string_to_bytes_4);
    string_to_bytes!("FFFF", vec![0xFF, 0xFF], string_to_bytes_5);
    string_to_bytes!("FFF", vec![0x0F, 0xFF], string_to_bytes_6);
    string_to_bytes!("0FFF", vec![0x0F, 0xFF], string_to_bytes_7);
    string_to_bytes!("0F0F", vec![0x0F, 0x0F], string_to_bytes_8);

    macro_rules! bytes_to_string {
        ($s:literal, $v:expr, $f:ident) => {
            #[test]
            fn $f() {
                assert_eq!(bytes_to_string($v), $s);
            }
        };
    }

    bytes_to_string!("", &Vec::<u8>::new(), bytes_to_string_0);
    bytes_to_string!("00", &vec![0], bytes_to_string_1);
    bytes_to_string!("0101", &vec![1, 1], bytes_to_string_2);
    bytes_to_string!("FFFF", &vec![0xFF, 0xFF], bytes_to_string_3);
    bytes_to_string!("0FFF", &vec![0x0F, 0xFF], bytes_to_string_4);
    bytes_to_string!("0F0F", &vec![0x0F, 0x0F], bytes_to_string_5);

    macro_rules! bytes_to_string_64 {
        ($s:literal, $v:expr, $f:ident) => {
            #[test]
            fn $f() {
                assert_eq!(bytes_to_string_64($v), $s);
            }
        };
    }

    bytes_to_string_64!(
        "0000000000000000000000000000000000000000000000000000000000000000",
        &Vec::<u8>::new(),
        bytes_to_string_64_0
    );

    bytes_to_string_64!(
        "0000000000000000000000000000000000000000000000000000000000000000",
        &vec![0],
        bytes_to_string_64_1
    );

    bytes_to_string_64!(
        "0000000000000000000000000000000000000000000000000000000000000101",
        &vec![1, 1],
        bytes_to_string_64_2
    );

    bytes_to_string_64!(
        "000000000000000000000000000000000000000000000000000000000000FFFF",
        &vec![0xFF, 0xFF],
        bytes_to_string_64_3
    );

    bytes_to_string_64!(
        "0000000000000000000000000000000000000000000000000000000000000FFF",
        &vec![0x0F, 0xFF],
        bytes_to_string_64_4
    );

    bytes_to_string_64!(
        "0000000000000000000000000000000000000000000000000000000000000F0F",
        &vec![0x0F, 0x0F],
        bytes_to_string_64_5
    );
}
