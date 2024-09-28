use std::fmt::{Display, Formatter};

use super::{
    fee_filter::FeeFilter, get_header::GetHeader, headers::Headers, ping::Ping, pong::Pong, send_compact::SendCompact,
    version::Version,
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Command {
    pub bytes: [u8; 12],
}

#[derive(Debug, PartialEq)]
pub enum Commands {
    VerAck,
    Version(Version),
    GetHeaders(GetHeader),
    SendCompact(SendCompact),
    Ping(Ping),
    Pong(Pong),
    FeeFilter(FeeFilter),
    // SendAddrV2 is read but behaviour is not implemented.
    // Ref: https://github.com/bitcoin/bips/blob/master/bip-0155.mediawiki
    SendAddrV2,
    // WtxIdRelay is read but behaviour is not implemented. It should be in version >= 70016.
    // Ref: https://github.com/bitcoin/bips/blob/master/bip-0339.mediawiki
    WtxIdRelay,
    Headers(Headers),
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            VERACK_COMMAND => "VerAck",
            VERSION_COMMAND => "Version",
            GET_HEADERS_COMMAND => "GetHeaders",
            SEND_COMPACT_COMMAND => "SendCompact",
            PING_COMMAND => "Ping",
            PONG_COMMAND => "Pong",
            FEE_FILTER_COMMAND => "FeeFilter",
            WTXID_RELAY_COMMAND => "WtxIdRelay",
            SENDADDRV2_COMMAND => "SendAddrV2",
            HEADERS_COMMAND => "Headers",
            _ => panic!("unknown_command"),
        };

        write!(f, "{:}", s)
    }
}

// Use https://www.rapidtables.com/convert/number/ascii-to-hex.html to convert ASCII to HEX

pub const VERACK_COMMAND: Command = Command {
    bytes: [0x76, 0x65, 0x72, 0x61, 0x63, 0x6B, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
};

pub const VERSION_COMMAND: Command = Command {
    bytes: [0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x00, 0x00, 0x00, 0x00, 0x00],
};

pub const GET_HEADERS_COMMAND: Command = Command {
    bytes: [0x67, 0x65, 0x74, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x73, 0x00, 0x00],
};

pub const SEND_COMPACT_COMMAND: Command = Command {
    bytes: [0x73, 0x65, 0x6E, 0x64, 0x63, 0x6D, 0x70, 0x63, 0x74, 0x00, 0x00, 0x00],
};

pub const PING_COMMAND: Command = Command {
    bytes: [0x70, 0x69, 0x6E, 0x67, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
};

pub const PONG_COMMAND: Command = Command {
    bytes: [0x70, 0x6F, 0x6E, 0x67, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
};

pub const FEE_FILTER_COMMAND: Command = Command {
    bytes: [0x66, 0x65, 0x65, 0x66, 0x69, 0x6C, 0x74, 0x65, 0x72, 0x00, 0x00, 0x00],
};

pub const WTXID_RELAY_COMMAND: Command = Command {
    bytes: [0x77, 0x74, 0x78, 0x69, 0x64, 0x72, 0x65, 0x6C, 0x61, 0x79, 0x00, 0x00],
};

pub const SENDADDRV2_COMMAND: Command = Command {
    bytes: [0x73, 0x65, 0x6E, 0x64, 0x61, 0x64, 0x64, 0x72, 0x76, 0x32, 0x00, 0x00],
};

pub const HEADERS_COMMAND: Command = Command {
    bytes: [0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x73, 0x00, 0x00, 0x00, 0x00, 0x00],
};
