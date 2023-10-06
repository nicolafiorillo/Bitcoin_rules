use rug::{integer::Order, Integer};

use crate::{std_lib::varint::VarInt, transaction::tx_error::TxError};

pub fn le_bytes_to_u32(bytes: &[u8], from: usize) -> Result<u32, TxError> {
    if bytes.len() < (from + 4) {
        return Err(TxError::Invalid4BytesLength);
    }

    let mut v: [u8; 4] = [0; 4];
    v.copy_from_slice(&bytes[from..(from + 4)]);
    Ok(u32::from_le_bytes(v))
}

pub fn u32_to_le_bytes(v: u32) -> [u8; 4] {
    v.to_le_bytes()
}

pub fn u64_le_bytes(bytes: &[u8], from: usize) -> Result<u64, TxError> {
    if bytes.len() < (from + 8) {
        return Err(TxError::Invalid8BytesLength);
    }

    let mut v: [u8; 8] = [0; 8];
    v.copy_from_slice(&bytes[from..(from + 8)]);
    Ok(u64::from_le_bytes(v))
}

pub fn varint_decode(bytes: &[u8], from: usize) -> Result<VarInt, TxError> {
    let vi = crate::std_lib::varint::varint_decode(bytes, from);
    if let Err(_e) = vi {
        return Err(TxError::VarIntError);
    }

    Ok(vi.unwrap())
}

pub fn le_32_bytes_to_integer(bytes: &[u8], from: usize) -> Result<Integer, TxError> {
    if bytes.len() < (from + 32) {
        return Err(TxError::Invalid32BytesLength);
    }

    let slice = &bytes[from..(from + 32)];
    Ok(Integer::from_digits(slice, Order::Lsf))
}

pub fn integer_to_le_32_bytes(i: &Integer) -> [u8; 32] {
    let mut bytes = [0; 32];
    i.write_digits(&mut bytes, Order::Lsf);
    bytes
}
