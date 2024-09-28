use crate::{
    std_lib::std_result::StdResult,
    transaction::tx_lib::{le_bytes_to_u32, le_bytes_to_u64},
};

#[derive(Debug, PartialEq, Eq)]
pub struct NetworkAddress {
    pub time: u32,         // LE - not present in version message
    pub services: u64,     // LE
    pub address: [u8; 16], // Network byte order. IPV6 or IPV4 (00 00 00 00 00 00 00 00 00 00 FF FF + IPV4 address).
    pub port: u16,         // BE
}

static NETWORK_ADDRESS_STRUCT_LEN: usize = std::mem::size_of::<NetworkAddress>(); // 26 bytes (30 with time)

impl NetworkAddress {
    pub fn new(time: u32, services: u64, address: [u8; 16], port: u16) -> Self {
        Self {
            time,
            services,
            address,
            port,
        }
    }

    pub fn serialize(&self, with_time: bool) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();
        v.reserve_exact(NETWORK_ADDRESS_STRUCT_LEN);

        if with_time {
            v.extend_from_slice(&self.time.to_le_bytes());
        }

        v.extend_from_slice(&self.services.to_le_bytes());
        v.extend_from_slice(&self.address);
        v.extend_from_slice(&self.port.to_be_bytes());
        v
    }

    pub fn deserialize(buf: &[u8], with_time: bool) -> StdResult<(Self, usize)> {
        let mut offset: usize = 0;

        let mut time: u32 = 0;
        if with_time {
            time = le_bytes_to_u32(buf, offset)?;
            offset += 4;
        }

        let services = le_bytes_to_u64(buf, offset)?;
        offset += 8;

        let mut address: [u8; 16] = [0; 16];
        address.copy_from_slice(&buf[offset..(offset + 16)]);
        offset += 16;

        let mut p: [u8; 2] = [0; 2];
        p.copy_from_slice(&buf[offset..(offset + 2)]);
        let port = u16::from_be_bytes(p);
        offset += 2;

        Ok((
            Self {
                time,
                services,
                address,
                port,
            },
            offset,
        ))
    }
}
