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
}
