use rug::{integer::Order, Integer};

use crate::std_lib::{std_result::StdResult, varint::VarInt};

pub fn le_bytes_to_u32(bytes: &[u8], from: usize) -> StdResult<u32> {
    if bytes.len() < (from + 4) {
        Err("invalid_4_bytes_length")?;
    }

    let mut v: [u8; 4] = [0; 4];
    v.copy_from_slice(&bytes[from..(from + 4)]);
    Ok(u32::from_le_bytes(v))
}

pub fn u32_to_le_bytes(v: u32) -> [u8; 4] {
    v.to_le_bytes()
}

pub fn le_bytes_to_u64(bytes: &[u8], from: usize) -> StdResult<u64> {
    if bytes.len() < (from + 8) {
        Err("invalid_8_bytes_length")?;
    }

    let mut v: [u8; 8] = [0; 8];
    v.copy_from_slice(&bytes[from..(from + 8)]);
    Ok(u64::from_le_bytes(v))
}

pub fn varint_decode(bytes: &[u8], from: usize) -> StdResult<VarInt> {
    let vi = crate::std_lib::varint::decode(bytes, from);
    if let Err(_e) = &vi {
        Err("varint_error")?;
    }

    Ok(vi.unwrap())
}

pub fn le_32_bytes_to_integer(bytes: &[u8], from: usize) -> StdResult<Integer> {
    if bytes.len() < (from + 32) {
        Err("invalid_32_bytes_length")?;
    }

    let slice = &bytes[from..(from + 32)];
    Ok(Integer::from_digits(slice, Order::Lsf))
}

pub fn integer_to_le_32_bytes(i: &Integer) -> [u8; 32] {
    let mut bytes = [0; 32];
    i.write_digits(&mut bytes, Order::Lsf);
    bytes
}
