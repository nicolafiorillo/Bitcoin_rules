pub mod vector {
    pub fn vect_to_array_32(v: &Vec<u8>) -> [u8; 32] {
        let mut arr: [u8; 32] = [0u8; 32];
        for i in 0..v.len() {
            arr[31 - i] = v[i];
        }

        arr
    }
}
